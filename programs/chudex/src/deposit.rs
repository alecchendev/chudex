use {
    anchor_lang::prelude::*,
    anchor_spl::{associated_token::AssociatedToken, mint, token::*},
    crate::{
        error::ChudexError,
        constants::*,
        state::*,
    },
};


pub fn process_deposit(ctx: Context<Deposit>, amount_a: u64, amount_b: u64) -> Result<()> {

    let pool = &mut ctx.accounts.pool;
    let vault_a = &mut ctx.accounts.vault_a;
    let vault_b = &mut ctx.accounts.vault_b;
    let mint_a = &mut ctx.accounts.mint_a;
    let mint_b = &mut ctx.accounts.mint_b;
    let mint_lp = &mut ctx.accounts.mint_lp;

    // calculate amounts
    let initial_deposit =
        pool.k == 0 && vault_a.amount == 0 && vault_b.amount == 0 && mint_lp.supply == 0;
    if initial_deposit {
        // set initial price/ratio
        pool.k = amount_a * amount_b;
    } else {
        // check if matches price/ratio
        let current_ratio = (vault_a.amount as f64 / vault_b.amount as f64);
        let proposed_ratio =
            ((vault_a.amount + amount_a) as f64 / (vault_b.amount + amount_b) as f64);
        let diff = (current_ratio - proposed_ratio).abs();
        if diff > 0.001 {
            return err!(ChudexError::AsymmetricLiquidity);
        }
    }
    let amount_lp = if initial_deposit {
        amount_a
    } else {
        ((amount_a as f64) / (vault_a.amount as f64) * (mint_lp.supply as f64)) as u64
    };

    // deposit token a
    anchor_spl::token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info().clone(),
            anchor_spl::token::Transfer {
                from: ctx.accounts.user_token_a.to_account_info().clone(),
                to: ctx.accounts.vault_a.to_account_info().clone(),
                authority: ctx.accounts.user.to_account_info().clone(),
            },
        ),
        amount_a,
    )?;

    // deposit token b
    anchor_spl::token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info().clone(),
            anchor_spl::token::Transfer {
                from: ctx.accounts.user_token_b.to_account_info().clone(),
                to: ctx.accounts.vault_b.to_account_info().clone(),
                authority: ctx.accounts.user.to_account_info().clone(),
            },
        ),
        amount_b,
    )?;

    // mint lp tokens
    anchor_spl::token::mint_to(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info().clone(),
            anchor_spl::token::MintTo {
                mint: ctx.accounts.mint_lp.to_account_info().clone(),
                to: ctx.accounts.user_token_lp.to_account_info().clone(),
                authority: ctx.accounts.pool.to_account_info().clone(),
            },
        )
        .with_signer(&[&[
            &POOL_SEED[..],
            mint_a.key().as_ref(),
            mint_b.key().as_ref(),
            &[ctx.accounts.pool.bump],
        ]]),
        amount_lp,
    )?;

    Ok(())

}



#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(
        mut,
        seeds = [&POOL_SEED[..], mint_a.key().as_ref(), mint_b.key().as_ref()],
        bump = pool.bump,
    )]
    pub pool: Account<'info, Pool>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = pool,
    )]
    pub vault_a: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = mint_b,
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
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = user,
    )]
    pub user_token_a: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = user,
    )]
    pub user_token_b: Box<Account<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        payer = user,
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