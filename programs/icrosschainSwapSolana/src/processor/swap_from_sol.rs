use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::SwapError;
use crate::utils::*;
use crate::{state::*, swap_raydium::handle_swap_raydium};
use spl_associated_token_account::get_associated_token_address;

pub fn handle_swap_from_solana(
    ctx: Context<SwapAccount>,
    amount_in: u64,
    minimum_amount_out: u64,
    version: u8,
) -> Result<()> {
    let usdc_account_address = get_associated_token_address(&PDA_KEY, &USDC_MINT_ADDRESS);
    // verified the program's pda usdc_account_address
    if usdc_account_address
        != *ctx
            .accounts
            .uer_destination_token_account
            .to_account_info()
            .key
    {
        Err(SwapError::DestinationInvalid.into())
    } else {
        let fee = minimum_amount_out * FEE_PERCENT / 1000;
        let treasure_ata =
            get_associated_token_address(&TREASURY_KEY, &ctx.accounts.token_mint.key);
        if *ctx.accounts.treasure_ata.key != treasure_ata {
            Err(SwapError::TreasureInvalid.into())
        } else {
            handle_swap_raydium(&ctx, amount_in, minimum_amount_out, version)?;
            spl_token_transfer_with_seed(TokenTransferParams {
                source: ctx.accounts.uer_destination_token_account.to_account_info(),
                destination: ctx.accounts.treasure_ata.to_account_info(),
                authority: ctx.accounts.pda_address.to_account_info(),
                authority_signer_seeds: &[&[PDA_SEED.as_bytes(), &[BUMP]]],
                token_program: ctx.accounts.token_program.to_account_info(),
                amount: fee,
            })
            .map_err(Into::into)
        }
    }
}
