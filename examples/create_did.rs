//! Create a new Identity
//!
//! Run with:
//!
//! ```
//! cargo run --example create_did
//! ```
//! 
//! Example return:
//! ```
//! Seed: "WUDVYBCONWBLKMHECWXIUROMMKYOZAEQTZEICFRZGSXBIZJONJAWQWOVOJNNWKKIEPVATUB9KKD9PMOLU"
//! DID::new called!
//! did: DID { network: "main", uuid: "WUDVYBCONWBLKMHECWXIUROMMKYOZAEQTZEICFRZGSXBIZJONJAWQWOVOJNNWKKIEPVATUB9KKD9PMOLU", urlScheme: "did", didMethod: "IOTA" }
//! DID::GetDID called!
//! did string: "did:IOTA:WUDVYBCONWBLKMHECWXIUROMMKYOZAEQTZEICFRZGSXBIZJONJAWQWOVOJNNWKKIEPVATUB9KKD9PMOLU"
//! ```

use did::utils::generate_seed;
use did::DID::DID;

fn main() {

    let seed = generate_seed::new();
    println!("Seed: {:?}", seed);
    
    let did =  DID::new(seed);
    println!("did: {:?}", did);
    println!("did string: {:?}", did.GetDID());
    
}