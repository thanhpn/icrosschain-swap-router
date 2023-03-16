use anchor_lang::prelude::*;

use crate::constants::*;
use crate::processor::raydium::RaydiumSwap;
use crate::state::*;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    instruction::{AccountMeta, Instruction},
    program::invoke,
    program::invoke_signed,
};

pub fn handle_swap_raydium(
    ctx: &Context<SwapAccount>,
    amount_in: u64,
    minimum_amount_out: u64,
    version: u8,
) -> Result<()> {
    let pool_program_id = ctx.accounts.pool_program_id.to_account_info();
    let ray_program_id = ctx.accounts.token_program.to_account_info();
    let amm_id = ctx.accounts.amm_id.to_account_info();
    let amm_authority = ctx.accounts.amm_authority.to_account_info();
    let amm_open_orders = ctx.accounts.amm_open_orders.to_account_info();
    let ato_or_mda = ctx.accounts.ato_or_mda.to_account_info();
    let pool_coin_token_account = ctx.accounts.pool_coin_token_account.to_account_info();
    let pool_pc_token_account = ctx.accounts.pool_pc_token_account.to_account_info();
    let serum_program_id = ctx.accounts.serum_program_id.to_account_info();
    let serum_market = ctx.accounts.serum_market.to_account_info();
    let serum_bids = ctx.accounts.serum_bids.to_account_info();
    let serum_asks = ctx.accounts.serum_asks.to_account_info();
    let serum_event_queue = ctx.accounts.serum_event_queue.to_account_info();
    let serum_coin_vault_account = ctx.accounts.serum_coin_vault_account.to_account_info();
    let serum_pc_vault_account = ctx.accounts.serum_pc_vault_account.to_account_info();
    let serum_vault_signer = ctx.accounts.serum_vault_signer.to_account_info();
    let uer_source_token_account = ctx.accounts.uer_source_token_account.to_account_info();
    let uer_destination_token_account =
        ctx.accounts.uer_destination_token_account.to_account_info();
    let user_source_owner = ctx.accounts.user_source_owner.to_account_info();
    let accounts;
    let raydium_accounts;

    if version == 4 {
        accounts = [
            amm_id.clone(),
            amm_authority.clone(),
            amm_open_orders.clone(),
            ato_or_mda.clone(),
            pool_coin_token_account.clone(),
            pool_pc_token_account.clone(),
            serum_program_id.clone(),
            serum_market.clone(),
            serum_bids.clone(),
            serum_asks.clone(),
            serum_event_queue.clone(),
            serum_coin_vault_account.clone(),
            serum_pc_vault_account.clone(),
            serum_vault_signer.clone(),
            uer_source_token_account.clone(),
            uer_destination_token_account.clone(),
            user_source_owner.clone(),
            ray_program_id.clone(),
        ];
        raydium_accounts = vec![
            AccountMeta::new_readonly(*ray_program_id.key, false),
            AccountMeta::new(*amm_id.key, false),
            AccountMeta::new_readonly(*amm_authority.key, false),
            AccountMeta::new(*amm_open_orders.key, false),
            AccountMeta::new(*ato_or_mda.key, false), // amm_target
            AccountMeta::new(*pool_coin_token_account.key, false),
            AccountMeta::new(*pool_pc_token_account.key, false),
            AccountMeta::new_readonly(*serum_program_id.key, false),
            AccountMeta::new(*serum_market.key, false),
            AccountMeta::new(*serum_bids.key, false),
            AccountMeta::new(*serum_asks.key, false),
            AccountMeta::new(*serum_event_queue.key, false),
            AccountMeta::new(*serum_coin_vault_account.key, false),
            AccountMeta::new(*serum_pc_vault_account.key, false),
            AccountMeta::new_readonly(*serum_vault_signer.key, false),
            AccountMeta::new(*uer_source_token_account.key, false),
            AccountMeta::new(*uer_destination_token_account.key, false),
            AccountMeta::new_readonly(*user_source_owner.key, true),
        ];
    } else {
        accounts = [
            amm_id.clone(),
            amm_authority.clone(),
            amm_open_orders.clone(),
            pool_coin_token_account.clone(),
            pool_pc_token_account.clone(),
            ato_or_mda.clone(),
            serum_program_id.clone(),
            serum_market.clone(),
            serum_bids.clone(),
            serum_asks.clone(),
            serum_event_queue.clone(),
            serum_coin_vault_account.clone(),
            serum_pc_vault_account.clone(),
            serum_vault_signer.clone(),
            uer_source_token_account.clone(),
            uer_destination_token_account.clone(),
            user_source_owner.clone(),
            ray_program_id.clone(),
        ];
        raydium_accounts = vec![
            AccountMeta::new_readonly(*ray_program_id.key, false),
            AccountMeta::new(*amm_id.key, false),
            AccountMeta::new_readonly(*amm_authority.key, false),
            AccountMeta::new(*amm_open_orders.key, false),
            AccountMeta::new(*pool_coin_token_account.key, false),
            AccountMeta::new(*pool_pc_token_account.key, false),
            AccountMeta::new(*ato_or_mda.key, false), // amm_target
            AccountMeta::new_readonly(*serum_program_id.key, false),
            AccountMeta::new(*serum_market.key, false),
            AccountMeta::new(*serum_bids.key, false),
            AccountMeta::new(*serum_asks.key, false),
            AccountMeta::new(*serum_event_queue.key, false),
            AccountMeta::new(*serum_coin_vault_account.key, false),
            AccountMeta::new(*serum_pc_vault_account.key, false),
            AccountMeta::new_readonly(*serum_vault_signer.key, false),
            AccountMeta::new(*uer_source_token_account.key, false),
            AccountMeta::new(*uer_destination_token_account.key, false),
            AccountMeta::new_readonly(*user_source_owner.key, true),
        ];
    }

    let instruction = Instruction {
        program_id: *pool_program_id.key,
        accounts: raydium_accounts,
        data: RaydiumSwap {
            instruction: 9,
            amount_in: amount_in,
            min_amount_out: minimum_amount_out,
        }
        .to_vec()?,
    };

    invoke(&instruction, &accounts).map_err(Into::into)
}

/**
 * Swap token from PDA
 */
pub fn handle_swap_to_solana_raydium(
    ctx: Context<SwapToSolanaAccount>,
    amount_in: u64,
    minimum_amount_out: u64,
    version: u8,
) -> Result<()> {
    let pool_program_id = ctx.accounts.pool_program_id.to_account_info();
    let ray_program_id = ctx.accounts.token_program.to_account_info();
    let amm_id = ctx.accounts.amm_id.to_account_info();
    let amm_authority = ctx.accounts.amm_authority.to_account_info();
    let amm_open_orders = ctx.accounts.amm_open_orders.to_account_info();
    let ato_or_mda = ctx.accounts.ato_or_mda.to_account_info();
    let pool_coin_token_account = ctx.accounts.pool_coin_token_account.to_account_info();
    let pool_pc_token_account = ctx.accounts.pool_pc_token_account.to_account_info();
    let serum_program_id = ctx.accounts.serum_program_id.to_account_info();
    let serum_market = ctx.accounts.serum_market.to_account_info();
    let serum_bids = ctx.accounts.serum_bids.to_account_info();
    let serum_asks = ctx.accounts.serum_asks.to_account_info();
    let serum_event_queue = ctx.accounts.serum_event_queue.to_account_info();
    let serum_coin_vault_account = ctx.accounts.serum_coin_vault_account.to_account_info();
    let serum_pc_vault_account = ctx.accounts.serum_pc_vault_account.to_account_info();
    let serum_vault_signer = ctx.accounts.serum_vault_signer.to_account_info();
    let uer_source_token_account = ctx.accounts.uer_source_token_account.to_account_info();
    let uer_destination_token_account = ctx.accounts.uer_destination_token_account.clone();
    let user_source_owner = ctx.accounts.user_source_owner.to_account_info();

    let accounts;
    let raydium_accounts;

    if version == 4 {
        accounts = [
            amm_id.clone(),
            amm_authority.clone(),
            amm_open_orders.clone(),
            ato_or_mda.clone(),
            pool_coin_token_account.clone(),
            pool_pc_token_account.clone(),
            serum_program_id.clone(),
            serum_market.clone(),
            serum_bids.clone(),
            serum_asks.clone(),
            serum_event_queue.clone(),
            serum_coin_vault_account.clone(),
            serum_pc_vault_account.clone(),
            serum_vault_signer.clone(),
            uer_source_token_account.clone(),
            uer_destination_token_account.clone(),
            user_source_owner.clone(),
            ray_program_id.clone(),
        ];

        raydium_accounts = vec![
            AccountMeta::new_readonly(*ray_program_id.key, false),
            AccountMeta::new(*amm_id.key, false),
            AccountMeta::new_readonly(*amm_authority.key, false),
            AccountMeta::new(*amm_open_orders.key, false),
            AccountMeta::new(*ato_or_mda.key, false), // amm_target
            AccountMeta::new(*pool_coin_token_account.key, false),
            AccountMeta::new(*pool_pc_token_account.key, false),
            AccountMeta::new_readonly(*serum_program_id.key, false),
            AccountMeta::new(*serum_market.key, false),
            AccountMeta::new(*serum_bids.key, false),
            AccountMeta::new(*serum_asks.key, false),
            AccountMeta::new(*serum_event_queue.key, false),
            AccountMeta::new(*serum_coin_vault_account.key, false),
            AccountMeta::new(*serum_pc_vault_account.key, false),
            AccountMeta::new_readonly(*serum_vault_signer.key, false),
            AccountMeta::new(*uer_source_token_account.key, false),
            AccountMeta::new(*uer_destination_token_account.key, false),
            AccountMeta::new_readonly(*user_source_owner.key, true),
        ];
    } else {
        accounts = [
            amm_id.clone(),
            amm_authority.clone(),
            amm_open_orders.clone(),
            pool_coin_token_account.clone(),
            pool_pc_token_account.clone(),
            ato_or_mda.clone(),
            serum_program_id.clone(),
            serum_market.clone(),
            serum_bids.clone(),
            serum_asks.clone(),
            serum_event_queue.clone(),
            serum_coin_vault_account.clone(),
            serum_pc_vault_account.clone(),
            serum_vault_signer.clone(),
            uer_source_token_account.clone(),
            uer_destination_token_account.clone(),
            user_source_owner.clone(),
            ray_program_id.clone(),
        ];

        raydium_accounts = vec![
            AccountMeta::new_readonly(*ray_program_id.key, false),
            AccountMeta::new(*amm_id.key, false),
            AccountMeta::new_readonly(*amm_authority.key, false),
            AccountMeta::new(*amm_open_orders.key, false),
            AccountMeta::new(*pool_coin_token_account.key, false),
            AccountMeta::new(*pool_pc_token_account.key, false),
            AccountMeta::new(*ato_or_mda.key, false), // amm_target
            AccountMeta::new_readonly(*serum_program_id.key, false),
            AccountMeta::new(*serum_market.key, false),
            AccountMeta::new(*serum_bids.key, false),
            AccountMeta::new(*serum_asks.key, false),
            AccountMeta::new(*serum_event_queue.key, false),
            AccountMeta::new(*serum_coin_vault_account.key, false),
            AccountMeta::new(*serum_pc_vault_account.key, false),
            AccountMeta::new_readonly(*serum_vault_signer.key, false),
            AccountMeta::new(*uer_source_token_account.key, false),
            AccountMeta::new(*uer_destination_token_account.key, false),
            AccountMeta::new_readonly(*user_source_owner.key, true),
        ];
    }
    let instruction = Instruction {
        program_id: *pool_program_id.key,
        accounts: raydium_accounts,
        data: RaydiumSwap {
            instruction: 9,
            amount_in: amount_in,
            min_amount_out: minimum_amount_out,
        }
        .to_vec()?,
    };

    invoke_signed(&instruction, &accounts, &[&[PDA_SEED.as_bytes(), &[BUMP]]]).map_err(Into::into)
}
