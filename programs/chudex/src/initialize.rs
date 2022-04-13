
use {
    anchor_lang::prelude::*,
    anchor_spl::{associated_token::AssociatedToken, mint, token::*},
    crate::{
        error::ChudexError,
        constants::*,
        state::*,
    },
};

pub fn process_initialize(ctx: Context<Initialize>) -> Result<()> {
    // vvv all done in accounts struct
    // init pool metadata
    // init token a vault
    // init token b vault
    // init mint

    // init pool metadata
    let pool = Pool {
        bump: *ctx.bumps.get("pool").unwrap(),
        mint_a: ctx.accounts.mint_a.key(),
        mint_b: ctx.accounts.mint_b.key(),
        mint_lp: ctx.accounts.mint_lp.key(),
        k: 0,
        fee: FEE_START,
        record_index: 0,
        records: [Record::default(); 16],
    };
    ctx.accounts.pool.set_inner(pool);

    Ok(())
}


#[derive(Accounts)]
pub struct Initialize<'info> {
    // data accounts
    #[account(
        init,
        space = 1 + 32 + 32 + 32 + 8 + 8 + 1 + 16 * (8 + 8) + 8, // final +8 for discrim bytes
        payer = user,
        seeds = [&POOL_SEED[..], mint_a.key().as_ref(), mint_b.key().as_ref()],
        bump,
    )]
    pub pool: Account<'info, Pool>,

    /// CHECK: shouldn't be initialized
    #[account(
        seeds = [&POOL_SEED[..], mint_b.key().as_ref(), mint_a.key().as_ref()],
        bump,
    )]
    pub other_pool: AccountInfo<'info>,

    #[account(
        init,
        payer = user,
        associated_token::mint = mint_a,
        associated_token::authority = pool,
    )]
    pub vault_a: Box<Account<'info, TokenAccount>>,

    #[account(
        init,
        payer = user,
        associated_token::mint = mint_b,
        associated_token::authority = pool,
    )]
    pub vault_b: Box<Account<'info, TokenAccount>>,

    #[account(
        owner = Token::id(),
        constraint = mint_a.key() != mint_b.key(),
    )]
    pub mint_a: Account<'info, Mint>,

    #[account(
        owner = Token::id()
    )]
    pub mint_b: Account<'info, Mint>,

    #[account(
        init,
        payer = user,
        seeds = [&MINT_LP_SEED[..], pool.key().as_ref()],
        bump,
        mint::authority = pool,
        mint::decimals = mint_a.decimals, //std::cmp::max(mint_a.decimals, mint_b.decimals)
    )]
    pub mint_lp: Box<Account<'info, Mint>>,

    // signers
    #[account(mut)]
    pub user: Signer<'info>,

    // programs
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>, // in anchor book examples this is absent, but compiler says to include
}
