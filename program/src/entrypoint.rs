#![cfg(not(feature = "no-entrypoint"))]

use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey, pubkey::Pubkey};

solana_program::entrypoint!(entry);

fn entry(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    match instruction_data[0] {
        0 => {
            crate::processor::process_callback(program_id, accounts, &instruction_data[1..])
        }
        _ => {
            crate::processor::trigger_proof(program_id, accounts, &instruction_data[1..])
        }
    }
}