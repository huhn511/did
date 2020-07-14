//!
//! To Generate a new RSA Keypair
//!

// use openssl::rsa::{Rsa};
// use openssl::pkey::{PKey};

pub fn new() -> String {
    
    // let rsa = Rsa::generate(2048).unwrap();
    // let privkey = PKey::from_rsa(rsa).unwrap();
    // println!("privkey: {:?}", &privkey);
    "41".to_owned()
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