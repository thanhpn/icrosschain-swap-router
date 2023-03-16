use crate::errors::SwapError;
use anchor_lang::prelude::*;
use anchor_spl::token::*;

use solana_program::{
    account_info::AccountInfo,
    clock::Clock,
    program::{invoke, invoke_signed},
    program_memory::sol_memcmp,
    program_pack::{IsInitialized, Pack},
    pubkey::{Pubkey, PUBKEY_BYTES},
};
use spl_associated_token_account::get_associated_token_address;

pub fn assert_initialized<T: Pack + IsInitialized>(account_info: &AccountInfo) -> Result<T> {
    let account: T = T::unpack_unchecked(&account_info.data.borrow())?;
    if !account.is_initialized() {
        Err(SwapError::Uninitialized.into())
    } else {
        Ok(account)
    }
}

pub fn cmp_pubkeys(a: &Pubkey, b: &Pubkey) -> bool {
    sol_memcmp(a.as_ref(), b.as_ref(), PUBKEY_BYTES) == 0
}

pub fn assert_owned_by(account: &AccountInfo, owner: &Pubkey) -> Result<()> {
    if !cmp_pubkeys(account.owner, owner) {
        Err(SwapError::IncorrectOwner.into())
    } else {
        Ok(())
    }
}
///TokenTransferParams
pub struct TokenTransferParams<'a: 'b, 'b> {
    /// source
    /// CHECK: account checked in CPI
    pub source: AccountInfo<'a>,
    /// destination
    /// CHECK: account checked in CPI
    pub destination: AccountInfo<'a>,
    /// amount
    pub amount: u64,
    /// authority
    /// CHECK: account checked in CPI
    pub authority: AccountInfo<'a>,
    /// authority_signer_seeds
    pub authority_signer_seeds: &'b [&'b [&'b [u8]]],
    /// token_program
    /// CHECK: account checked in CPI
    pub token_program: AccountInfo<'a>,
}

#[inline(always)]
pub fn spl_token_transfer(params: TokenTransferParams<'_, '_>) -> Result<()> {
    let TokenTransferParams {
        source,
        destination,
        authority,
        token_program,
        amount,
        authority_signer_seeds,
    } = params;

    let transfer_instruction = Transfer {
        from: source,
        to: destination,
        authority: authority,
    };

    let cpi_program = token_program;
    // Create the Context for our Transfer request
    let cpi_ctx = CpiContext::new(cpi_program, transfer_instruction);

    // Execute anchor's helper function to transfer tokens
    anchor_spl::token::transfer(cpi_ctx, amount)?;
    Ok(())
}

#[inline(always)]
pub fn spl_token_transfer_with_seed(params: TokenTransferParams<'_, '_>) -> Result<()> {
    let TokenTransferParams {
        source,
        destination,
        authority,
        token_program,
        amount,
        authority_signer_seeds,
    } = params;

    let transfer_instruction = Transfer {
        from: source,
        to: destination,
        authority: authority,
    };

    let cpi_program = token_program;
    // Create the Context for our Transfer request
    let cpi_ctx =
        CpiContext::new_with_signer(cpi_program, transfer_instruction, authority_signer_seeds);

    // Execute anchor's helper function to transfer tokens
    anchor_spl::token::transfer(cpi_ctx, amount)?;
    Ok(())
}

pub fn assert_is_ata(
    ata: &AccountInfo,
    wallet: &Pubkey,
    mint: &Pubkey,
) -> core::result::Result<spl_token::state::Account, ProgramError> {
    assert_owned_by(ata, &spl_token::id())?;
    let ata_account: spl_token::state::Account = assert_initialized(ata)?;
    assert_keys_equal(ata_account.owner, *wallet)?;
    assert_keys_equal(ata_account.mint, *mint)?;
    assert_keys_equal(get_associated_token_address(wallet, mint), *ata.key)?;
    Ok(ata_account)
}

pub fn assert_keys_equal(key1: Pubkey, key2: Pubkey) -> Result<()> {
    if !cmp_pubkeys(&key1, &key2) {
        err!(SwapError::PublicKeyMismatch)
    } else {
        Ok(())
    }
}
