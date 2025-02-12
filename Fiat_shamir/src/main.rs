
use ark_ff::PrimeField;
use sha3::{Digest, Keccak256};
use std::marker::PhantomData;

// This struct represents a cryptographic transcript that accumulates data and hashes it.
struct Transcript<F: PrimeField, T: HashTrait> {
    hasher: T,
    f_element: PhantomData<F>,
}

// Defines how to interact with the transcript.
impl<F: PrimeField, T: HashTrait> Transcript<F, T> {
    fn init(hash_function: T) -> Self {
        Transcript {
            hasher: hash_function,
            f_element: PhantomData,
        }
    }

    // Feeds data into the hasher
    fn append(&mut self, data: &[u8]) {
        self.hasher.absorb(data);
    }

    // Computes the final hash and ensures consistency across multiple calls
    fn hash(&mut self) -> F {
        let hash = self.hasher.squeeze();
        F::from_be_bytes_mod_order(&hash)
    }
}

// Trait defining hash functions
trait HashTrait {
    fn absorb(&mut self, data: &[u8]);
    fn squeeze(&mut self) -> Vec<u8>;
}

// Implement HashTrait for Keccak256
impl HashTrait for Keccak256 {
    fn absorb(&mut self, data: &[u8]) {
        self.update(data);
    }

    fn squeeze(&mut self) -> Vec<u8> {
        let mut cloned_hasher = Keccak256::new();
        cloned_hasher.update(self.clone().finalize_reset()); // Clone and finalize
        cloned_hasher.finalize_reset().to_vec() // Get final hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fq;
    use ark_ff::BigInteger;

    #[test]
    fn test_hash() {
        let mut transcript = Transcript::<Fq, Keccak256>::init(Keccak256::new());
        transcript.append(b"data1");
        transcript.append(Fq::from(7).into_bigint().to_bytes_be().as_slice());
        transcript.append(b"data2");

        let hash = transcript.hash();
        let hash2 = transcript.hash();
        let hash3 = transcript.hash();

        assert_eq!(hash, hash2);
        assert_eq!(hash, hash3);
    }
}

// fn main() {
//     println!("Hello, world!");
// }
