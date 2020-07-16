use crate::did::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DIDDocument {
    did: DID
}

impl DIDDocument {
    /// Get the instance of IOTA Client. It will init the instance if it's not created yet.
    pub fn new(did: DID) -> Self {
        println!("DIDDocument::new called.");
        DIDDocument {
            did: did
        }
    }

}