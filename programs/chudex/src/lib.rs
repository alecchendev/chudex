use anchor_lang::prelude::*;
use anchor_spl::{associated_token::{AssociatedToken}, token::{Token, TokenAccount, Mint}, mint};
use std;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

const FEE_START: u64 = 10; // start at 0.1% fees
const FEE_DECIMALS: u8 = 5;

const FEE_MIN: u64 = 1;
const FEE_MAX: u64 = 200;

const TARGET_DEMAND: u64 = 100;
const PRICE_CHANGE_DECIMALS: u8 = 4;

const POOL_SEED: &[u8] = b"pool";

#[program]
pub mod chudex {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, k: u64) -> Result<()> {
        // init pool metadata - done
        // init token a vault

        // init token b vault
        // init mint
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
        space = 1 + 32 + 32 + 32 + 8 + 8 + 1 + 16 * (8 + 8),
        payer = user,
        seeds = [],
        bump
    )]
    pub pool: Account<'info, Pool>,
    #[account(
        seeds = [],
        bump,
        constraint = true, // length == 0
    )]
    pub other_pool: Account<'info, Pool>,

    #[account(
        init,
        payer = user,
        associated_token::mint = mint_a,
        associated_token::authority = pool,
    )]
    pub vault_a: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = user,
        associated_token::mint = mint_b,
        associated_token::authority = pool,
    )]
    pub vault_b: Account<'info, TokenAccount>,

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
        mint::decimals = std::cmp::max(mint_a.decimals, mint_b.decimals),
        mint::authority = pool,
    )]
    pub mint_lp: Account<'info, Mint>,

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
