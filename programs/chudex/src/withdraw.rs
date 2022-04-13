use {
    crate::{constants::*, error::ChudexError, state::*},
    anchor_lang::prelude::*,
    anchor_spl::{associated_token::AssociatedToken, mint, token::*},
};

pub fn process_withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    // withdraw token a
    // withdraw token b
    // burn lp tokens
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
