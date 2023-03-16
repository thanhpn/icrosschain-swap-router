//! Raydium protocol native instructions
//! See https://github.com/raydium-io/raydium-contract-instructions/blob/master/amm_instruction.rs
//! for more details and accounts references

use {
    crate::pack::check_data_len,
    arrayref::{array_mut_ref, mut_array_refs},
    solana_program::program_error::ProgramError,
};

#[derive(Clone, Copy, Debug)]
pub struct RaydiumSwap {
    pub instruction: u8,
    pub amount_in: u64,
    pub min_amount_out: u64,
}

impl RaydiumSwap {
    pub const LEN: usize = 17;

    pub fn get_size(&self) -> usize {
        RaydiumSwap::LEN
    }

    pub fn pack(&self, output: &mut [u8]) -> Result<usize, ProgramError> {
        check_data_len(output, RaydiumSwap::LEN)?;

        let output = array_mut_ref![output, 0, RaydiumSwap::LEN];

        let (instruction_out, amount_in_out, min_amount_out_out) = mut_array_refs![output, 1, 8, 8];

        instruction_out[0] = self.instruction as u8;
        *amount_in_out = self.amount_in.to_le_bytes();
        *min_amount_out_out = self.min_amount_out.to_le_bytes();

        Ok(RaydiumSwap::LEN)
    }

    pub fn to_vec(&self) -> Result<Vec<u8>, ProgramError> {
        let mut output: [u8; RaydiumSwap::LEN] = [0; RaydiumSwap::LEN];
        if let Ok(len) = self.pack(&mut output[..]) {
            Ok(output[..len].to_vec())
        } else {
            Err(ProgramError::InvalidInstructionData)
        }
    }
}
