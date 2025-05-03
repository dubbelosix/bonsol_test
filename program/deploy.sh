#!/bin/bash
set -e
cargo build-sbf
solana program deploy target/deploy/bonsol_test_program.so
cp target/deploy/bonsol_test_program-keypair.json ~/.keys/bonsol_test_program.json