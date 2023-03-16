use crate::constants::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;
use anchor_spl::token::{Token, TokenAccount};

#[derive(Accounts)]
pub struct Initialize {}

#[account]
#[derive(Default)]
pub struct UserProfile {
    pub name: String,
    pub avatar_url: String,
    pub owner: Pubkey,
    pub status: String,
}

#[derive(Accounts)]
pub struct UserProfileAccount<'info> {
    #[account(init, payer = user, space = 9000)]
    pub profile: Account<'info, UserProfile>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug)]
pub struct SwapFromSolanaArgs {
    pub min_evm_amount_out: String,
    pub to_evm_chain_id: String,
    pub to_evm_address: String,
    pub to_evm_router: String,
    pub to_evm_swap_path: String,
    pub to_evm_fee: String,
    pub to_evm_sqrt_ratio_x96: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug)]
pub struct SwapToSolanaArgs {
    pub to_address: String,
    pub from_address: String,
    pub tx_hash: String,
}

#[derive(Accounts)]
pub struct SwapAccount<'info> {
    /// CHECK: account checked in CPI
    pub pool_program_id: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    pub token_program: Program<'info, Token>,
    /// CHECK: account checked in CPI
    #[account(mut)]
    pub amm_id: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    pub amm_authority: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    #[account(mut)]
    pub amm_open_orders: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    #[account(mut)]
    pub ato_or_mda: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI : To token
    #[account(mut)]
    pub pool_coin_token_account: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI: From token
    #[account(mut)]
    pub pool_pc_token_account: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    pub serum_program_id: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    #[account(mut)]
    pub serum_market: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    #[account(mut)]
    pub serum_bids: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    #[account(mut)]
    pub serum_asks: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    #[account(mut)]
    pub serum_event_queue: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    #[account(mut)]
    pub serum_coin_vault_account: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    #[account(mut)]
    pub serum_pc_vault_account: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    pub serum_vault_signer: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI : Source account of token
    #[account(mut)]
    pub uer_source_token_account: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    #[account(mut)]
    pub uer_destination_token_account: Box<Account<'info, TokenAccount>>,
    /// CHECK: account checked in CPI
    #[account(mut)]
    // pub user_source_owner: UncheckedAccount<'info>,
    pub user_source_owner: Signer<'info>,
    /// CHECK: account checked in CPI
    #[account(mut)]
    pub treasure_ata: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    pub token_mint: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    pub pda_address: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct SwapUSDCAccount<'info> {
    /// CHECK: account checked in CPI
    pub token_program: Program<'info, Token>,
    /// CHECK: account checked in CPI
    #[account(mut)]
    pub uer_source_token_account: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    #[account(mut)]
    pub uer_destination_token_account: Box<Account<'info, TokenAccount>>,
    /// CHECK: account checked in CPI
    #[account(mut)]
    pub user_source_owner: Signer<'info>,
    /// CHECK: account checked in CPI
    #[account(mut)]
    pub treasure_ata: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    #[account(mut)]
    pub pda_address: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct SwapToSolanaAccount<'info> {
    /// CHECK: account checked in CPI
    pub pool_program_id: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    pub token_program: Program<'info, Token>,
    /// CHECK: account checked in CPI
    #[account(mut)]
    pub amm_id: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    pub amm_authority: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    #[account(mut)]
    pub amm_open_orders: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    #[account(mut)]
    pub ato_or_mda: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI : To token
    #[account(mut)]
    pub pool_coin_token_account: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI: From token
    #[account(mut)]
    pub pool_pc_token_account: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    pub serum_program_id: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    #[account(mut)]
    pub serum_market: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    #[account(mut)]
    pub serum_bids: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    #[account(mut)]
    pub serum_asks: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    #[account(mut)]
    pub serum_event_queue: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    #[account(mut)]
    pub serum_coin_vault_account: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    #[account(mut)]
    pub serum_pc_vault_account: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    pub serum_vault_signer: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI : Source account of token
    #[account(mut)]
    pub uer_source_token_account: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    #[account(mut)]
    pub uer_destination_token_account: AccountInfo<'info>,
    /// CHECK: account checked in CPI
    #[account(mut)]
    pub user_source_owner: UncheckedAccount<'info>,
    pub author: Signer<'info>,
    #[account(mut, seeds = [b"seed_src_tx".as_ref()], bump = source_tx.bump)]
    pub source_tx: Account<'info, SourceTx>,
}

#[derive(Accounts)]
pub struct SwapAccountFee<'info> {
    pub token_program: Program<'info, Token>,
    /// CHECK: The associated token account that we are transferring the token from
    #[account(mut)]
    pub from: UncheckedAccount<'info>,
    /// CHECK: The associated token account that we are transferring the token to
    #[account(mut)]
    pub to: UncheckedAccount<'info>,
    // the authority of the from account
    /// CHECK: The associated token account that we are transferring the token to
    pub from_authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct TransferToPDA<'info> {
    pub token_program: Program<'info, Token>,
    /// CHECK: The associated token account that we are transferring the token from
    #[account(mut)]
    pub from: UncheckedAccount<'info>,
    /// CHECK: The associated token account that we are transferring the token to
    #[account(mut)]
    pub to: AccountInfo<'info>,
    // the authority of the from account
    /// CHECK: The associated token account that we are transferring the token to
    pub from_authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    pub token_program: Program<'info, Token>,
    /// CHECK: The associated token account that we are transferring the token from
    #[account(mut)]
    pub from: UncheckedAccount<'info>,
    /// CHECK: The associated token account that we are transferring the token to
    #[account(mut)]
    pub to: AccountInfo<'info>,
    // the authority of the from account
    /// CHECK: The associated token account that we are transferring the token to
    pub from_authority: UncheckedAccount<'info>,
    /// CHECK: The associated token account that we are transferring the token to
    #[account(mut)]
    pub mint_address: UncheckedAccount<'info>,
    #[account(mut)]
    pub caller: Signer<'info>,
}

#[derive(Accounts)]
pub struct InitSourceTxAccount<'info> {
    #[account(init, seeds = [b"seed_src_tx".as_ref()], bump, payer = author, space = SourceTx::LEN)]
    pub source_tx: Account<'info, SourceTx>,
    #[account(mut)]
    pub author: Signer<'info>,
    #[account(address = system_program::ID)]
    /// CHECK: account checked in CPI
    pub system_program: AccountInfo<'info>,
}

#[account]
pub struct SourceTx {
    pub author: Pubkey,
    pub content: Vec<String>,
    pub bump: u8,
}

// maxsize of InitSourceTxAccount - 145 txhashes
impl SourceTx {
    const LEN: usize = 10240;
}

#[derive(Accounts)]
pub struct SwapOrcaAccount<'info> {
    /// CHECK: account checked in CPI
    pub pool_program_id: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    pub token_program: Program<'info, Token>,
    /// CHECK: account checked in CPI
    #[account(mut)]
    pub token_authority: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    pub whirl_pool: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    #[account(mut)]
    pub token_owner_account_a: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    #[account(mut)]
    pub token_vault_a: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI : To token
    #[account(mut)]
    pub token_owner_account_b: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI: From token
    #[account(mut)]
    pub token_vault_b: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    pub tick_array_0: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    #[account(mut)]
    pub tick_array_1: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    #[account(mut)]
    pub tick_array_2: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    #[account(mut)]
    pub oracle: UncheckedAccount<'info>,
    #[account(mut)]
    // pub user_source_owner: UncheckedAccount<'info>,
    pub user_source_owner: Signer<'info>,
    /// CHECK: account checked in CPI: pda usdc account of token
    #[account(mut)]
    pub treasure_ata: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI
    pub token_mint: UncheckedAccount<'info>,
    /// CHECK: account checked in CPI: pda account
    pub pda_address: UncheckedAccount<'info>,
}
