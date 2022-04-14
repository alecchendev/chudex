
pub mod initialize;
pub mod deposit;
pub mod withdraw;
pub mod exchange;

use {
    anchor_lang::prelude::*,
    crate::{
        initialize::*,
        deposit::*,
        withdraw::*,
        exchange::*,
    },
};

declare_id!("BqaAuVM2Bj7Lnsq3VPorVuq5u57gwEz3F4dna72q6M3s");

#[program]
pub mod chudex {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        process_initialize(ctx)?;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount_a: u64, amount_b: u64) -> Result<()> {
        process_deposit(ctx, amount_a, amount_b)?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        process_withdraw(ctx, amount)?;
        Ok(())
    }

    pub fn exchange(ctx: Context<Exchange>, amount_in: u64) -> Result<()> {
        process_exchange(ctx, amount_in)?;
        Ok(())
    }
}

pub mod constants {
    pub const FEE_START: u64 = 10; // start at 0.1% fees
    pub const DECIMAL_BASE: u64 = 10;
    pub const FEE_DECIMALS: u32 = 5;

    pub const FEE_MIN: u64 = 1;
    pub const FEE_MAX: u64 = 200;

    pub const TARGET_DEMAND: u64 = 100;
    pub const PRICE_CHANGE_DECIMALS: u32 = 4;

    pub const POOL_SEED: &[u8] = b"pool";
    pub const MINT_LP_SEED: &[u8] = b"mint_lp";
}

pub mod state {
    use super::*;

    #[account]
    #[derive(Default)]
    pub struct Pool {
        pub bump: u8,
        pub mint_a: Pubkey,
        pub mint_b: Pubkey,
        pub mint_lp: Pubkey,
        pub k: u64,
        pub fee: u64,
        pub record_index: u8,
        pub records: [Record; 16], // ring buffer
    }

    #[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone, Default, Copy)]
    pub struct Record {
        pub time: i64,
        pub price_change: u64, // out of 10 ** PRICE_CHANGE_DECIMALS
    }
}

pub mod error {
    use super::*;

    #[error_code]
    pub enum ChudexError {
        #[msg("Deposit liquidity amounts don't match current supply ratio.")]
        AsymmetricLiquidity,
        #[msg("Don't have enough LP tokens to withdraw attempted amount.")]
        InsufficientLpTokens,
    }
}