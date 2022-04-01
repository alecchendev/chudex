use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

const FEE_DECIMALS: u8 = 4;

#[program]
pub mod chudex {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>) -> Result<()> {
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct Deposit {}

#[derive(Accounts)]
pub struct Withdraw {}

#[account]
pub struct Pool {
    bump: u8,
    mint_a: Pubkey,
    mint_b: Pubkey,
    mint_lp: Pubkey,
    k_a: u64,
    k_b: u64,
    fee: u64,
}

#[account]
pub struct FeeState {}

/*
Instrs
- initialize - init pool metadata, constant k, fee calculation state
- deposit (2-sided) - transfer tokens to pool, mint LP token
- withdraw (2-sided) - burn LP token, transfer tokens from pool

State
- Pool metadata
- Pool tx state for fee calculation

*/
