use risc0_zkvm::guest::env;
use bytemuck::pod_read_unaligned;
use risc0_zkvm::sha::rust_crypto::Digest;

fn read_bytes_from_env() -> Vec<u8> {
    let mut len_bytes: [u8; 4] = [0u8; 4];
    env::read_slice(&mut len_bytes);
    let len: usize = pod_read_unaligned::<u32>(&len_bytes) as usize;
    let mut buffer = vec![0u8; len];
    env::read_slice(&mut buffer);
    buffer
}

fn main() {
    let mut seed_bytes: Vec<u8> = read_bytes_from_env();
    if seed_bytes.len() < 1 {
        seed_bytes = vec![0];
    }
    let num_iterations_bytes = read_bytes_from_env();
    let mut num_iterations: u32 = u32::from_le_bytes(num_iterations_bytes[0..4].try_into().unwrap());
    if num_iterations == 0 { num_iterations=1;}

    let mut input_data_for_hash = seed_bytes.clone(); // Clone seed_bytes as it's used again below
    input_data_for_hash.extend_from_slice(&num_iterations_bytes[0..4]);

    // commit to the input
    let input_hash = sha2::Sha256::digest(&input_data_for_hash).to_vec(); // This will be 32 bytes
    
    let mut current_hash: Vec<u8> = seed_bytes.clone();
    for _ in 0..num_iterations {
        let next_hash = sha2::Sha256::digest(&current_hash).to_vec();
        current_hash = next_hash;
    }
    let mut combined_result = input_hash;
    
    // also commit to the output
    combined_result.extend_from_slice(&current_hash);

    // 32 byte commitment to input + 32 byte commitment to actual output of the computation
    // we need to commit to the input because the chain needs to know the correct input was passed in
    // The chain will verify this by ensuring the input hash matches before accepting the result
    // of the computation
    env::commit_slice(&combined_result);
}