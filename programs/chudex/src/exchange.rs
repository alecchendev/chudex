use {
    crate::{constants::*, error::ChudexError, state::*},
    anchor_lang::prelude::*,
    anchor_spl::{associated_token::AssociatedToken, mint, token::*},
};

pub fn process_exchange(ctx: Context<Exchange>, amount_in: u64) -> Result<()> {

    // match vaults to deposits/withdraws
    let (vault_deposit, vault_withdraw) =
        if ctx.accounts.vault_a.key() == ctx.accounts.user_token_deposit.mint {
            (&ctx.accounts.vault_a, &ctx.accounts.vault_b)
        } else {
            (&ctx.accounts.vault_b, &ctx.accounts.vault_a)
        };

    // calculate stuff
    let p = ctx.accounts.pool.fee as f64 / DECIMAL_BASE.pow(FEE_DECIMALS) as f64;
    let amount_out = vault_withdraw.amount - (1.0 / (1.0 + (amount_in as f64 / vault_deposit.amount as f64 * (1.0 - p))) * vault_withdraw.amount as f64) as u64;

    let price_change = (amount_out as f64 / vault_withdraw.amount as f64 * DECIMAL_BASE.pow(PRICE_CHANGE_DECIMALS) as f64) as u64;

    // deposit token_in
    anchor_spl::token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info().clone(),
            anchor_spl::token::Transfer {
                from: ctx.accounts.user_token_deposit.to_account_info().clone(),
                to: vault_deposit.to_account_info().clone(),
                authority: ctx.accounts.user.to_account_info().clone(),
            },
        ),
        amount_in,
    )?;

    // withdraw token_out
    anchor_spl::token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info().clone(),
            anchor_spl::token::Transfer {
                from: vault_withdraw.to_account_info().clone(),
                to: ctx.accounts.user_token_withdraw.to_account_info().clone(),
                authority: ctx.accounts.pool.to_account_info().clone(),
            },
        )
        .with_signer(&[&[
            &POOL_SEED[..],
            ctx.accounts.pool.mint_a.clone().as_ref(),
            ctx.accounts.pool.mint_b.clone().as_ref(),
            &[ctx.accounts.pool.bump],
        ]]),
        amount_out,
    )?;

    // make record
    let pool = &mut ctx.accounts.pool;
    let record_index = pool.record_index as usize;
    pool.records[record_index] = Record { time: Clock::get()?.unix_timestamp, price_change: price_change };

    // change fee (TO DO)

    Ok(())
}

#[derive(Accounts)]
pub struct Exchange<'info> {
    #[account(
        seeds = [&POOL_SEED[..], pool.mint_a.as_ref(), pool.mint_b.as_ref()],
        bump = pool.bump,
    )]
    pub pool: Account<'info, Pool>,

    #[account(
        mut,
        associated_token::mint = pool.mint_a,
        associated_token::authority = pool,
    )]
    pub vault_a: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = pool.mint_b,
        associated_token::authority = pool,
    )]
    pub vault_b: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = if mint_withdraw.key() == vault_a.key() { vault_b.mint } else { vault_a.mint },
        associated_token::authority = user,
    )]
    pub user_token_deposit: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_withdraw,
        associated_token::authority = user,
    )]
    pub user_token_withdraw: Box<Account<'info, TokenAccount>>,

    #[account(
        owner = Token::id(),
        constraint = mint_withdraw.key() == vault_a.mint || mint_withdraw.key() == vault_b.mint
    )]
    pub mint_withdraw: Account<'info, Mint>,

    // signers
    #[account(mut)]
    pub user: Signer<'info>,

    // programs
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
