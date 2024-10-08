//! A simple script to generate and verify the proof of a given program.
use num_bigint::BigUint;
use num_traits::Num;
use sp1_sdk::{
    install::try_install_circuit_artifacts, utils, ProverClient, SP1ProofWithPublicValues, SP1Stdin,
};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const FIBONACCI_ELF: &[u8] = include_bytes!("../../fibonacci-riscv32im-succinct-zkvm-elf");
pub const ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    // Setup logging for the application
    utils::setup_logger();

    // Set the input value for the Fibonacci calculation
    let n = 20;

    // Prepare the input for the zkVM
    let mut stdin = SP1Stdin::new();
    stdin.write(&n);

    // Initialize the prover client
    let client = ProverClient::new();
    let (pk, _) = client.setup(FIBONACCI_ELF);

    // Generate a proof for the Fibonacci program
    let proof = client
        .prove(&pk, stdin)
        .plonk()
        .run()
        .expect("Proving failed");

    // Save the generated proof to a binary file
    let proof_file = "proof.bin";
    proof.save(proof_file).unwrap();

    // Retrieve the verification key
    let vk_dir_entry = try_install_circuit_artifacts();
    let vk_bin_path = vk_dir_entry.join("plonk_vk.bin"); // For Groth16, use "groth16_vk.bin"

    // Read the verification key from file
    let vk = std::fs::read(vk_bin_path).unwrap();

    // Load the saved proof and convert it to a Plonk proof
    let proof = SP1ProofWithPublicValues::load("proof.bin")
        .map(|sp1_proof_with_public_values| {
            sp1_proof_with_public_values.proof.try_as_plonk().unwrap() // Use `try_as_groth_16()` for Groth16
        })
        .expect("Failed to load proof");

    // Extract the raw proof and public inputs
    let raw_proof = hex::decode(proof.raw_proof).unwrap();
    let public_inputs = proof.public_inputs;

    // Convert public inputs to byte representations
    let vkey_hash = BigUint::from_str_radix(&public_inputs[0], 10)
        .unwrap()
        .to_bytes_be();
    let committed_values_digest = BigUint::from_str_radix(&public_inputs[1], 10)
        .unwrap()
        .to_bytes_be();

    // Prepare input for the verifier program
    let mut stdin = SP1Stdin::new();
    stdin.write_slice(&raw_proof);
    stdin.write_slice(&vk);
    stdin.write_slice(&vkey_hash);
    stdin.write_slice(&committed_values_digest);

    // Setup the verifier program
    let (pk, vk) = client.setup(ELF);
    // Generate a proof for the verifier program
    let proof = client
        .prove(&pk, stdin)
        .plonk()
        .run()
        .expect("Proving failed");

    // Verify the proof of the verifier program
    client.verify(&proof, &vk).expect("verification failed");

    println!("Successfully verified proof for the program!")
}
