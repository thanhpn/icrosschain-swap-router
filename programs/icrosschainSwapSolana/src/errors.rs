use anchor_lang::prelude::*;

#[error_code]
pub enum SwapError {
    #[msg("Account is not initialized!")]
    Uninitialized,
    #[msg("Account does not have correct owner!")]
    IncorrectOwner,
    #[msg("Token transfer failed")]
    TokenTransferFailed,
    #[msg("Derived key invalid")]
    DerivedKeyInvalid,
    #[msg("Public key mismatch")]
    PublicKeyMismatch,
    #[msg("Destination key invalid")]
    DestinationInvalid,
    #[msg("Signer key invalid")]
    SignerInvalid,
    #[msg("Can not swap over 5000 usdc")]
    OverMaxAmount,
    #[msg("Can not swap below 5 usdc")]
    BelowMinAmout,
    #[msg("Treasure address is invalid")]
    TreasureInvalid,
    #[msg("Duplicate Source tx hash")]
    DuplicateSourceTx,
    #[msg("Expired transaction")]
    ExpiredTx,
}
