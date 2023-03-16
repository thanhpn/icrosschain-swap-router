pub mod constants;
pub mod errors;
pub mod pack;
pub mod processor;
pub mod state;
pub mod string;
pub mod utils;

use crate::swap_from_sol::handle_swap_from_solana;
use crate::swap_from_sol_usdc::handle_swap_usdc_from_solana;
use crate::swap_sol::handle_swap_solana;
use anchor_lang::prelude::*;
pub use errors::*;
pub use pack::*;
pub use processor::*;
pub use state::*;
pub use utils::*;

declare_id!("8AjFdB883udAJC1883po4KpgkSzyrWtL6KaTRCkzAiX9");

#[program]
pub mod icrosschain_swap_solana {

    use super::*;

    pub fn swap_solana(
        ctx: Context<SwapAccount>,
        amount_in: u64,
        minimum_amount_out: u64,
        version: u8,
    ) -> Result<()> {
        handle_swap_solana(ctx, amount_in, minimum_amount_out, version).map_err(Into::into)
    }

    pub fn swap_from_solana(
        ctx: Context<SwapAccount>,
        amount_in: u64,
        minimum_amount_out: u64,
        version: u8,
        from_solana_arg: SwapFromSolanaArgs,
    ) -> Result<()> {
        handle_swap_from_solana(ctx, amount_in, minimum_amount_out, version).map_err(Into::into)
    }

    pub fn swap_from_solana_usdc(
        ctx: Context<SwapUSDCAccount>,
        amount_in: u64,
        minimum_amount_out: u64,
        from_solana_arg: SwapFromSolanaArgs,
    ) -> Result<()> {
        handle_swap_usdc_from_solana(ctx, amount_in, minimum_amount_out, from_solana_arg)
            .map_err(Into::into)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
