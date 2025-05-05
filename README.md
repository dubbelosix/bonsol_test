# Simple Bonsol Test program

In order to demonstrate bonsol's benefit in compute compression, we're going to zk prove an iterated hashing function.
Solana has a syscall and can do a couple of hashes very easily, but in our case, we want to do an arbitrary number of hashes repeatedly over the same seed (much like PoH)

`./bonsol_test_data` contains the seed and number of iterations (declared as constants for the purpose of this simple exercise)
`./iterated_hashing/src/main.rs` contains the zk proof code.
`./program` contains the onchain code that verifies the proof and accepts the result of computation

`./src/bin/public_inputs.rs` contains the server that serves the seed and number of iterations to the bonsol node
`./src/bin/trigger.rs` contains the code that triggers the proof and submits the result to the onchain program

## Clone the repos locally
```
git clone https://github.com/dubbelosix/bonsol_test.git
git clone https://github.com/bonsol-collective/bonsol
```

## From the bonsol repo
### install bonsol to run locally
```
cd bonsol
./bin/install.sh
./bin/install_prover.sh --prefix .
./bin/setup.sh
```

### Start validator
* Start the validator from the bonsol repo (with the bonsol and metaplex-core programs)
```
./bin/validator.sh -r
```

### Start the bonsol relay
* Ensure the cuda flag is present if running on a machine with GPUs
```
./bin/run-node.sh -F cuda
```

## From the bonsol_test repo

### Bonsol build and deploy
```
bonsol build -z iterated_hashing
```
* Check `iterated_hashing/manifest.json` and get the image-id. Ensure the same image-id is used in the below copy command
* Upload the compiled riscv32 ELF to S3. Replace the bucket with your own
```
aws s3 cp iterated_hashing/target/riscv32im-risc0-zkvm-elf/docker/iterated_hashing.bin s3://rubicon-images/iterated_hashing-a50a57236235f45a610d47417c3489ab097909986a625f74a5c3a9ea4fa01a53
bonsol deploy url -m iterated_hashing/manifest.json --url https://rubicon-images.s3.amazonaws.com -y
```
* The same image id also needs to be updated in `program/src/lib.rs` as the value of `BONSOL_IMAGE_ID`

### Deploy the bonsol_test on-chain program
```
cd program
./deploy.sh
```
* Monitor logs with
```
solana logs
```

### Run the public url server
```
cargo run --release --bin public_inputs
```

### Run the trigger
```
cargo run --release --bin trigger
```
* This makes use of the CPI to the bonsol program to create the job (as opposed to using the `bonsol execute` command, which requires the `execution-request.json`)