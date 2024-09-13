# BN254 Rust Verifier

The `bn254-rust-verifier` crate is used for verifying Groth16 and PlonK proofs on the `Bn254` curve, ensuring compatibility with proofs generated by the `gnark` library. One can save proofs and verification keys from `gnark` and subsequently load them to this library for verifying the proofs with ease.

### How to save proofs and verification keys from gnark

To save the proof and verification key from `gnark`, one can use the following code snippet:

```go
// Write the verifier key.
vkFile, err := os.Create("plonk_vk.bin") // or "groth16_vk.bin"
if err != nil {
    panic(err)
}
defer vkFile.Close()
_, err = vk.WriteTo(vkFile)
if err != nil {
    panic(err)
}

// Write the proving key.
proofFile, err := os.Create("proof.bin")
if err != nil {
    panic(err)
}
defer proofFile.Close()
_, err = proof.WriteTo(proofFile)
if err != nil {
    panic(err)
}
```

## Usage

To use this library, add it as a dependency in your `Cargo.toml`:
```toml
[dependencies]
bn254-rust-verifier = "1.0.2"
```

Then, you can verify a proof by calling the `verify` function:
```rs
use bn254_rust_verifier::{verify, ProvingSystem, Fr};

fn main() {

    let proof = std::fs::read("proof.bin").unwrap();
    let vk = std::fs::read("vk.bin").unwrap();

    match PlonkVerifier::verify(&proof, &vk, &[vkey_hash, committed_values_digest]) {
        Ok(true) => {
            println!("Proof is valid");
        }
        Ok(false) | Err(_) => {
            println!("Proof is invalid");
            panic!();
        }
    }
}

```

## Features

- Verification of Groth16 and PlonK proofs generated using `gnark` or `sp1` on the `Bn254` curve.
- Easy integration into Rust projects.