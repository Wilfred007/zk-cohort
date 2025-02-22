use ark_ff::{BigInteger, PrimeField};
use sha3::{Digest, Keccak256};
use multilinear::{partial_evaluate, evaluate, MultilinearEvalForm };
use fiat_shamir::Transcript;




struct Prover<F: PrimeField> {
    initial_poly: MultilinearEvalForm<F>; // 2ab + 3bc
    initial_claimed_sum: F, // Summation of the evaluation at the boolean hypercube
    transcript: Transcript, // The hash
    round_proof_poly: Vec<MultilinearEvalForm<F>>, // 
}


struct Proof<F: PrimeField> {
    initial_poly: MultilinearEvalForm<F>,
    initial_claimed_sum: F,
    round_proof_poly: Vec<MultilinearEvalForm>
}












fn main() {
    println!("Hello, world!");
}
