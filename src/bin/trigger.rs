use std::str::FromStr;
use rand::Rng;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::pubkey::Pubkey;
use bonsol_test_program::{TriggerProof, BONSOL_IMAGE_ID};
use bonsol_interface::util::{deployment_address, execution_address};
use shellexpand::tilde;
use solana_client::rpc_client;
use solana_program::message::{v0, VersionedMessage};
use solana_program::system_program;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::compute_budget::ComputeBudgetInstruction;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::{keypair, Signer};
use solana_sdk::transaction::VersionedTransaction;
use bonsol_test_data::{SEED_MESSAGE, NUM_ITERATIONS};
use sha2::Sha256;
use sha2::Digest;

pub fn reformat_instruction_prefix(instruction: &mut Instruction) {
    instruction.data.insert(0, 1);
}

pub fn offchain_calculation() {
    println!("--- Performing Offchain Calculation ---");
    let seed_bytes: Vec<u8> = SEED_MESSAGE.as_bytes().to_vec();
    let mut current_hash: Vec<u8> = seed_bytes;
    println!("Starting {} SHA-256 iterations...", NUM_ITERATIONS);
    for _ in 0..NUM_ITERATIONS {
        let next_hash = Sha256::digest(&current_hash).to_vec();
        current_hash = next_hash;

    }
    println!("Finished {} SHA-256 iterations.", NUM_ITERATIONS);

    let final_hash_hex = hex::encode(&current_hash);

    println!("Offchain Calculated Final Hash: {}", final_hash_hex);
}

pub fn get_bonsol_prove_instruction(bonsol_test_program_id: Pubkey, signer_account: AccountMeta) -> Instruction {
    let execution_id = rand::thread_rng().sample_iter(&rand::distributions::Alphanumeric).take(10).map(char::from).collect::<String>();

    let (requester, bump) =
        Pubkey::find_program_address(&[execution_id.as_bytes()], &bonsol_test_program_id);

    let requester_meta = AccountMeta::new(requester, false);

    let (execution_account, _) = execution_address(&requester, execution_id.as_bytes());
    let (deployment_account, _) = deployment_address(BONSOL_IMAGE_ID);

    let system_account = AccountMeta::new_readonly(system_program::id(),false );
    let execution_account_meta = AccountMeta::new(execution_account,false);
    let deployment_account_meta = AccountMeta::new_readonly(deployment_account,false);
    let bonsol_program = Pubkey::from_str("BoNsHRcyLLNdtnoDf8hiCNZpyehMC4FDMxs6NTxFi3ew").unwrap();
    let bonsol_program_meta = AccountMeta::new_readonly(bonsol_program, false);
    let bonsol_test_program_meta = AccountMeta::new_readonly(bonsol_test_program_id, false);

    let args = TriggerProof {
        bump,
        execution_id,
    };
    let mut instruction = Instruction::new_with_bincode(
        bonsol_test_program_id,
        &args,
        vec![signer_account, requester_meta, execution_account_meta, deployment_account_meta, system_account, bonsol_program_meta, bonsol_test_program_meta]);
    reformat_instruction_prefix(&mut instruction);
    instruction
}

pub fn submit_transactions(fee_payer: Keypair, signers: Vec<Keypair>, instructions_vec: Vec<Vec<Instruction>>) {
    for instructions in instructions_vec {
        let client = rpc_client::RpcClient::new_with_commitment("http://127.0.0.1:8899".to_string(), CommitmentConfig::confirmed());
        let latest_block_hash = client.get_latest_blockhash().unwrap();
        let vm = VersionedMessage::V0(v0::Message::try_compile(&fee_payer.pubkey(),
                                                               &instructions, &[], latest_block_hash).unwrap());
        let signers: Vec<&Keypair> = signers.iter().map(|x|x).collect();
        let transaction = VersionedTransaction::try_new(vm.clone(), &signers).unwrap();

        println!("{:?}",transaction);
        client.send_and_confirm_transaction(&transaction).unwrap();
    }
}



fn main() {
    let kp = keypair::read_keypair_file(tilde("~/.keys/local.json").into_owned()).unwrap();
    let program_keypair = keypair::read_keypair_file(tilde("~/.keys/bonsol_test_program.json").into_owned()).unwrap();
    let bonsol_test_program_id = program_keypair.pubkey();

    let signer_account = AccountMeta::new(kp.pubkey(), true);
    let mut signers = vec![];
    signers.push(kp.insecure_clone());
    offchain_calculation();
    let mut instructions = vec![];
    instructions.push(vec![ComputeBudgetInstruction::set_compute_unit_limit(1_400_000),
                           get_bonsol_prove_instruction(bonsol_test_program_id, signer_account.clone())]);
    submit_transactions(kp, signers, instructions);
}