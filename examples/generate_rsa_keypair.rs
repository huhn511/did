//! Get an unused address from a connected node.
//!
//! Run with:
//!
//! ```
//! cargo run --example generate_rsa_keypair
//! ```

use did::utils::generate_rsa_keypair;

fn main() {

    let rsa = generate_rsa_keypair::new();
    println!("RSA: {:?}", rsa);
    
}