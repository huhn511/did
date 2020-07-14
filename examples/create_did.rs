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
//! did string: "did:iota:WUDVYBCONWBLKMHECWXIUROMMKYOZAEQTZEICFRZGSXBIZJONJAWQWOVOJNNWKKIEPVATUB9KKD9PMOLU"
//! ```

use did_helpers::utils::generate_seed;
use did::DID;

fn main() {

    let seed = generate_seed::new();
    println!("Seed: {:?}", seed);
    
    let did =  DID::new(seed);
    println!("did: {:?}", did);
    println!("did string: {:?}", did.get_did());
    
}