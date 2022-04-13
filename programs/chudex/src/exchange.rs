use {
    crate::{constants::*, error::ChudexError, state::*},
    anchor_lang::prelude::*,
    anchor_spl::{associated_token::AssociatedToken, mint, token::*},
};

pub fn process_exchange(ctx: Context<Exchange>, amount_in: u64) -> Result<()> {
    // calculate stuff
    // deposit token_in
    // withdraw token_out
    // make record
    // change fee
    Ok(())
}

#[derive(Accounts)]
pub struct Exchange {}
