


#[derive(Debug)]
pub struct DID {
    pub network: String,
    pub uuid: String,
    pub urlScheme: String,
    pub didMethod: String,
}

impl DID {
    pub fn new(did: String) -> Self {
        println!("DID::new called!");
        DID {
            network: "main".to_owned(),
            urlScheme: "did".to_owned(),
            didMethod: "IOTA".to_owned(),
            uuid: did.to_owned(),
        }
    }
    pub fn GetDID(&self) -> String {
        println!("DID::GetDID called!");
        format!("{}:{}:{}", &self.urlScheme, &self.didMethod, &self.uuid)
    }
}


// pub fn new(did: String) -> String {
    
//     "Hello World!".to_owned()
// }
