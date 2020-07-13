//! Get an unused address from a connected node.
//!
//! Run with:
//!
//! ```
//! cargo run --example generate_ed25519_keypair
//! ```

use did::utils::generate_ed25519_keypair;

fn main() {

    let ed25519 = generate_ed25519_keypair::new();
    println!("ed25519 public key: {:?}", ed25519);
    
}