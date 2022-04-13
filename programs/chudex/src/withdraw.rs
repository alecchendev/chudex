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
pub struct Withdraw {}
