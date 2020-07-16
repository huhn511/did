use did::DIDDocument;

use iota::{
    client::Transfer,
    crypto::ternary::Kerl,
    signing::ternary::{Seed, TernarySeed},
    ternary::{T1B1Buf, TryteBuf},
    transaction::bundled::{Address, BundledTransactionField},
};
use iota_conversion::Trinary;

pub struct DIDPublisher {}

impl DIDPublisher {
    pub fn new(provider: String, seed: String) -> Self {
        println!("DIDPublisher::new called!");
        DIDPublisher {}
    }
    pub async fn publish_document(&self, document: DIDDocument) -> () {
        // Prepare a vector of transfers
        let mut transfers = Vec::new();

        println!("document: {:?}", document);

        let string = serde_json::to_string(&document).unwrap();

            // Push the transfer to vector.
        transfers.push(Transfer {
            // Address is 81 trytes.
            address: Address::from_inner_unchecked(
                TryteBuf::try_from_str(
                    "ADDRESSADDRESSADDRESSADDRESSADDRESSADDRESSADDRESSADDRESSADDRESSADDRESSADDRESSADDR",
                )
                .unwrap()
                .as_trits()
                .encode(),
            ),
            // We are using a zero balance seed so we make a zero value transfer here
            value: 0,
            message: Some(String::from(string)),
            tag: None,
        });

        // Create a client instance
        iota::Client::add_node("https://nodes.comnet.thetangle.org").unwrap();
        // Call send_transfers api
        // Below is just a dummy seed which just serves as an example.
        // If you want to replace your own. It probably should be a seed with balance on comnet/devnet.
        let res =  iota::Client::send_transfers(
                    Some(
                    &TernarySeed::<Kerl>::from_trits(
                        TryteBuf::try_from_str(
                            "RVORZ9SIIP9RCYMREUIXXVPQIPHVCNPQ9HZWYKFWYWZRE9JQKG9REPKIASHUUECPSQO9JT9XNMVKWYGVA",
                        )
                        .unwrap()
                        .as_trits()
                        .encode::<T1B1Buf>(),
                    )
                    .unwrap(),
                ))
                // Input the transfers
                .transfers(transfers)
                // We are sending to comnet, so mwm should be 10. It's 14 by default if you don't call this.
                .min_weight_magnitude(10)
                // Sending to the node and receive the response
                .send().await.unwrap();

        // The response of send_transfers is vector of Transaction type. We choose the first one and see what is its bundle hash
        println!(
        "Search in theTangle: https://comnet.thetangle.org/bundle/{}",
        res[0].bundle().to_inner().as_i8_slice().trytes().unwrap()
        );
    }
}
