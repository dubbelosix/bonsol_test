use bonsol_interface::instructions::{execute_v1, CallbackConfig, ExecutionConfig, InputRef};
use bonsol_schema::root_as_execution_request_v1;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::{msg, system_instruction};
use solana_program::program::invoke_signed;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::sysvar::Sysvar;
use solana_program::clock::Clock;
use solana_program::program_error::ProgramError;
use crate::{TriggerProof, BONSOL_IMAGE_ID};
use bonsol_test_data::{SEED_MESSAGE, NUM_ITERATIONS};

// This needs to be calculated in a way thats compatible with how the zk programming is commiting to the input
// Otherwise it won't match
// This is necessary to ensure that the prover didn't tamper with the inputs
pub fn get_input_commitment() -> [u8;32] {
    solana_program::hash::hashv(&[SEED_MESSAGE.as_bytes(), NUM_ITERATIONS.to_le_bytes().as_slice()]).to_bytes()
}

pub fn process_callback(_program_id: &Pubkey,
                        accounts: &[AccountInfo],
                        instruction_data: &[u8],) -> ProgramResult{
    let (input_digest, committed_outputs) = instruction_data.split_at(32);

    let _callback_owner = accounts[0].owner;
    let execution_request_data = accounts[0].data.borrow();
    let execution_request =
        root_as_execution_request_v1(*execution_request_data).unwrap();

    let _execution_image_id = execution_request.image_id();

    let my_calculated_input_digest = get_input_commitment();
    assert_eq!(input_digest, my_calculated_input_digest.as_slice());

    msg!("Checks passed",);
    msg!("committed_outputs: {:?}", hex::encode(committed_outputs));
    Ok(())
}

pub fn trigger_proof(program_id: &Pubkey,
                     accounts: &[AccountInfo],
                     instruction_data: &[u8],) -> ProgramResult {
    let trigger_proof_data: TriggerProof = bincode::deserialize(&instruction_data).unwrap();
    // vec![signer_account, requester_meta, execution_account_meta, deployment_account_meta, system_account, bonsol_program_meta, bonsol_test_program_meta]
    let payer = &accounts[0];
    let requester = &accounts[1];
    let _execution_account = accounts[2].key;
    let system_account = &accounts[4];

    // create account
    let lamports = Rent::get()?.minimum_balance(32usize) ;
    let create_pda_account_ix =
        system_instruction::create_account(&payer.key, &requester.key, lamports, 32, program_id);

    invoke_signed(
        &create_pda_account_ix,
        &[requester.clone(), payer.clone(), system_account.clone()],
        &[&[trigger_proof_data.execution_id.as_bytes(), &[trigger_proof_data.bump]]],
    )
        .map_err(|_e| ProgramError::Custom(0))?;

    let tip = 12000;
    let expiration = Clock::get()?.slot + 5000;

    // BONSOL_IMAGE_ID is critical because the smart contract needs to
    // be aware of the code image being used by the prover.

    // We pass this to bonsol.
    let my_calculated_input_digest = get_input_commitment();

    let ix = execute_v1(
        requester.key,
        payer.key,
        BONSOL_IMAGE_ID,
        &trigger_proof_data.execution_id,
        vec![
            InputRef::url("http://127.0.0.1:8000/seed_bytes".as_bytes()),
            InputRef::url("http://127.0.0.1:8000/num_iterations".as_bytes()),
        ],
        tip,
        expiration,
        ExecutionConfig {
            verify_input_hash: true,
            input_hash: Some(my_calculated_input_digest.as_slice()),
            forward_output: true,
        },
        Some(CallbackConfig {
            program_id: program_id.clone(),
            instruction_prefix: vec![0],
            extra_accounts: vec![],
        }),
        None
    ).map_err(|_| ProgramError::InvalidInstructionData)?;

    invoke_signed(
        &ix,
        accounts,
        &[&[trigger_proof_data.execution_id.as_bytes(), &[trigger_proof_data.bump]]],
    )?;

    Ok(())
}