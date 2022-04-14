use {
    crate::{constants::*, error::ChudexError, state::*},
    anchor_lang::prelude::*,
    anchor_spl::{associated_token::AssociatedToken, mint, token::*},
};

pub fn process_withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {

    // check user has enough to claim
    let amount_lp = amount;
    require!(
        ctx.accounts.user_token_lp.amount >= amount_lp,
        ChudexError::InsufficientLpTokens
    );

    // calculate amounts
    let portion = amount_lp as f64 / ctx.accounts.mint_lp.supply as f64;
    let mut amount_a = (portion * ctx.accounts.vault_a.amount as f64) as u64;
    let mut amount_b = (portion * ctx.accounts.vault_b.amount as f64) as u64;

    // to solve rounding errors upon last withdrawal
    let withdrawing_all = amount_lp == ctx.accounts.mint_lp.supply;
    if withdrawing_all {
        amount_a = ctx.accounts.vault_a.amount;
        amount_b = ctx.accounts.vault_b.amount;
    }

    // withdraw token a
    anchor_spl::token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info().clone(),
            anchor_spl::token::Transfer {
                from: ctx.accounts.vault_a.to_account_info().clone(),
                to: ctx.accounts.user_token_a.to_account_info().clone(),
                authority: ctx.accounts.pool.to_account_info().clone(),
            },
        )
        .with_signer(&[&[
            &POOL_SEED[..],
            ctx.accounts.pool.mint_a.clone().as_ref(),
            ctx.accounts.pool.mint_b.clone().as_ref(),
            &[ctx.accounts.pool.bump],
        ]]),
        amount_a,
    )?;

    // withdraw token b
    anchor_spl::token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info().clone(),
            anchor_spl::token::Transfer {
                from: ctx.accounts.vault_b.to_account_info().clone(),
                to: ctx.accounts.user_token_b.to_account_info().clone(),
                authority: ctx.accounts.pool.to_account_info().clone(),
            },
        )
        .with_signer(&[&[
            &POOL_SEED[..],
            ctx.accounts.pool.mint_a.clone().as_ref(),
            ctx.accounts.pool.mint_b.clone().as_ref(),
            &[ctx.accounts.pool.bump],
        ]]),
        amount_b,
    )?;

    // burn lp tokens
    anchor_spl::token::burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info().clone(),
            anchor_spl::token::Burn {
                mint: ctx.accounts.mint_lp.to_account_info().clone(),
                to: ctx.accounts.user_token_lp.to_account_info().clone(),
                authority: ctx.accounts.user.to_account_info().clone(),
            },
        ),
        amount_lp,
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
        mut,
        seeds = [&POOL_SEED[..], mint_a.key().as_ref(), mint_b.key().as_ref()],
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
        owner = Token::id(),
        constraint = mint_a.key() == pool.mint_a,
    )]
    pub mint_a: Account<'info, Mint>,

    #[account(
        owner = Token::id(),
        constraint = mint_b.key() == pool.mint_b
    )]
    pub mint_b: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [&MINT_LP_SEED[..], pool.key().as_ref()],
        bump,
        constraint = mint_lp.key() == pool.mint_lp
    )]
    pub mint_lp: Box<Account<'info, Mint>>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_a,
        associated_token::authority = user,
    )]
    pub user_token_a: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_b,
        associated_token::authority = user,
    )]
    pub user_token_b: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = mint_lp,
        associated_token::authority = user,
    )]
    pub user_token_lp: Box<Account<'info, TokenAccount>>,

    // signers
    #[account(mut)]
    pub user: Signer<'info>,

    // programs
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
