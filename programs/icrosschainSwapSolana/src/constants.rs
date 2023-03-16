use solana_program::pubkey::Pubkey;

pub const PROGRAM_KEY: Pubkey =
    solana_program::pubkey!("8AjFdB883udAJC1883po4KpgkSzyrWtL6KaTRCkzAiX9");

pub const TREASURY_KEY: Pubkey =
    solana_program::pubkey!("5fRbDExFSbQUVT5X7NY7pFjpNf2A1hBJU8NwWD6gLfMZ");

pub const PDA_KEY: Pubkey = solana_program::pubkey!("HELPUJfMCE3g9aokKsbjszBZRB71GYkc5ZXbUqHPPJjx");
pub const USDC_MINT_ADDRESS: Pubkey =
    solana_program::pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
pub const DEPLOYER_KEY: Pubkey =
    solana_program::pubkey!("2m3Kpy2CFbXuDShdAqVyR6BBEDthHpwP6PSq3xY2KXz5");
pub const OPERATOR_KEY: Pubkey =
    solana_program::pubkey!("8b15QKRpT4XmaJjsCkpLWQwxkB53CEqJS7YcNSBn9a6B");

pub const PDA_SEED: &str = "icrosschain";
pub const BUMP: u8 = 255;

pub const FEE_PERCENT: u64 = 2; // 0.2%
pub const MAX_SWAP: u64 = 10_000;
pub const MIN_SWAP: u64 = 5_000;

pub const MAX_SOURCETX: usize = 50;
