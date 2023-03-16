use anchor_lang::prelude::*;

use crate::constants::*;
use crate::state::*;
use crate::utils::*;

pub fn handle_transfer_to_pda(ctx: TransferToPDA, amount_in: u64) -> Result<()> {
    spl_token_transfer(TokenTransferParams {
        source: ctx.from.to_account_info(),
        destination: ctx.to.to_account_info(),
        authority: ctx.from_authority.to_account_info(),
        authority_signer_seeds: &[&[PDA_SEED.as_bytes(), &[BUMP]]],
        token_program: ctx.token_program.to_account_info(),
        amount: amount_in,
    })
    .map_err(Into::into)
}
