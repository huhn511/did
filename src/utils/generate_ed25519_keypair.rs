//!
//! To Generate a new RSA Keypair
//!

use ed25519_compact::*;


pub fn new() -> () {
    
// Generates a new key pair using a random seed.
// A given seed will always produce the same key pair.
let key_pair = KeyPair::from_seed(Seed::default());

// Verifies the signature using the public part of the key pair.
println!("key_pair public key: {:?}", key_pair.pk);

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        // Test if result string has 81 chars
        assert_eq!("42", new());
    }
}