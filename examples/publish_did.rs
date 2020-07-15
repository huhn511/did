//! Create a new Identity
//!
//! Run with:
//!
//! ```
//! cargo run --example publish_did
//! ```
//! 
//! Example return:
//! ```
//! 
//! ```

use did_helpers::utils::generate_seed;
use did_iota::DIDPublisher;
use did::DID;
use did::DIDDocument;
#[smol_potat::main]
async fn main() -> () {

    let mut seed = generate_seed::new();
    println!("Seed: {:?}", seed);
    let did =  DID::new(seed.clone());
    println!("did: {:?}", did);
    println!("did string: {:?}", did.get_did());
    let document =  DIDDocument::new(did);
    println!("document: {:?}", document);
    
    let publisher = DIDPublisher::new("provider".to_string(), seed);
    
    
    let root = publisher.publish_document(document).await;
    println!("root: {:#?}", root);
}