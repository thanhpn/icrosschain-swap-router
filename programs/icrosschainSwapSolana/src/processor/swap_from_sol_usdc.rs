use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::SwapError;
use crate::utils::*;
use crate::{state::*, transfer_to_pda::handle_transfer_to_pda};
use spl_associated_token_account::get_associated_token_address;

pub fn handle_swap_usdc_from_solana(
    ctx: Context<SwapUSDCAccount>,
    amount_in: u64,
    minimum_amount_out: u64,
    from_solana_arg: SwapFromSolanaArgs,
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
        let fee_instruction = TransferToPDA {
            token_program: ctx.accounts.token_program.clone(),
            from: ctx.accounts.uer_source_token_account.clone(),
            to: ctx.accounts.uer_destination_token_account.to_account_info(),
            from_authority: ctx.accounts.user_source_owner.clone(),
        };

        handle_transfer_to_pda(fee_instruction, amount_in)?;

        let treasure_ata = get_associated_token_address(&TREASURY_KEY, &USDC_MINT_ADDRESS);
        if treasure_ata != *ctx.accounts.treasure_ata.key {
            Err(SwapError::TreasureInvalid.into())
        } else {
            let fee = amount_in * FEE_PERCENT / 1000;
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
