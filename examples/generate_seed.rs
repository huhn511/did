//! Create a random seed.
//!
//! Run with:
//!
//! ```
//! cargo run --example generate_seed
//! ```

use did::utils::generate_seed;

fn main() {

    let seed = generate_seed::new();
    println!("Seed: {:?}", seed);
    
}