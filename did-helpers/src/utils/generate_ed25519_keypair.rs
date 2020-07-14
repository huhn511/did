//!
//! To Generate a new RSA Keypair
//!

use bs58;
use ed25519_compact::*;

pub fn new() -> String {
    // Generates a new key pair using a random seed.
    // A given seed will always produce the same key pair.
    let key_pair = KeyPair::from_seed(Seed::default());

    // Envode the public key to base58 and return it.
    bs58::encode(key_pair.pk.to_vec()).into_string()
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
