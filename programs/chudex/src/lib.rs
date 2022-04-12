use anchor_lang::prelude::*;
use anchor_spl::{associated_token::{AssociatedToken}, token::{Token, TokenAccount, Mint}, mint};
use std;

declare_id!("BqaAuVM2Bj7Lnsq3VPorVuq5u57gwEz3F4dna72q6M3s");

const FEE_START: u64 = 10; // start at 0.1% fees
const FEE_DECIMALS: u8 = 5;

const FEE_MIN: u64 = 1;
const FEE_MAX: u64 = 200;

const TARGET_DEMAND: u64 = 100;
const PRICE_CHANGE_DECIMALS: u8 = 4;

const POOL_SEED: &[u8] = b"pool";
const MINT_LP_SEED: &[u8] = b"mint_lp";


#[program]
pub mod chudex {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, k: u64) -> Result<()> {
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
            k: k,
            fee: FEE_START,
            record_index: 0,
            records: [Record::default(); 16],
        };
        ctx.accounts.pool.set_inner(pool);

        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        // deposit token a
        // deposit token b
        // mint lp tokens
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        // withdraw token a
        // withdraw token b
        // burn lp tokens
        Ok(())
    }

    pub fn exchange(ctx: Context<Exchange>, amount_in: u64) -> Result<()> {
        // calculate stuff
        // deposit token_in
        // withdraw token_out
        // make record
        // change fee
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {

    // data accounts
    #[account(
        init,
        space = 1 + 32 + 32 + 32 + 8 + 8 + 1 + 16 * (8 + 8) + 8, // discrim bytes..?
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
        owner = Token::id()
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
        mint::decimals = std::cmp::max(mint_a.decimals, mint_b.decimals)
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

#[derive(Accounts)]
pub struct Deposit {}

#[derive(Accounts)]
pub struct Withdraw {}

#[derive(Accounts)]
pub struct Exchange {}

#[account]
#[derive(Default)]
pub struct Pool {
    bump: u8,
    mint_a: Pubkey,
    mint_b: Pubkey,
    mint_lp: Pubkey,
    k: u64,
    fee: u64,
    record_index: u8,
    records: [Record; 16], // ring buffer
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone, Default, Copy)]
pub struct Record {
    time: i64,
    price_change: u64, // out of 10 ** PRICE_CHANGE_DECIMALS
}

/*
Instrs
- initialize - init pool metadata, constant k, fee calculation state
- deposit (2-sided) - transfer tokens to pool, mint LP token
- withdraw (2-sided) - burn LP token, transfer tokens from pool

State
- Pool metadata
- Pool tx state for fee calculation

*/
