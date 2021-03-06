use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DID {
    pub network: String,
    pub uuid: String,
    pub url_scheme: String,
    pub did_method: String,
}

impl DID {
    pub fn new(did: String) -> Self {
        println!("DID::new called!");
        DID {
            network: "main".to_owned(),
            url_scheme: "did".to_owned(),
            did_method: "iota".to_owned(),
            uuid: did.to_owned(),
        }
    }
    pub fn get_did(&self) -> String {
        println!("DID::get_did called!");
        format!("{}:{}:{}", &self.url_scheme, &self.did_method, &self.uuid)
    }
}
