use anchor_lang::prelude::*;

use crate::{charge_fee::handle_swap_fee, state::*, swap_raydium::handle_swap_raydium};

pub fn handle_swap_solana(
    ctx: Context<SwapAccount>,
    amount_in: u64,
    minimum_amount_out: u64,
    version: u8,
) -> Result<()> {
    // switch dex
    handle_swap_raydium(&ctx, amount_in, minimum_amount_out, version).map_err(Into::into)
}
