pub mod utils;

pub fn greet() -> String {
    "Hello World!".to_owned()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!("Hello World!", greet())
    }
}


#[derive(Debug)]
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
            did_method: "IOTA".to_owned(),
            uuid: did.to_owned(),
        }
    }
    pub fn get_did(&self) -> String {
        println!("DID::get_did called!");
        format!("{}:{}:{}", &self.url_scheme, &self.did_method, &self.uuid)
    }
}
