use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod chudex {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

/*
Instrs
- initialize - init pool metadata, constant k, fee calculation state
- deposit (2-sided) - transfer tokens to pool, mint LP token
- withdraw (2-sided) - burn LP token, transfer tokens from pool

State
- Pool metadata
- Pool tx state for fee calculation

*/
