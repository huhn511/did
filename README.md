
# Digital Identity (DID)
This is a work-in-progress library for Digital Identity on IOTA. It follows the Decentralized Identifiers (DIDs) v0.13 and Verifiable Credentials standards created by the W3C Community Group. The concept of digital identity allows people, businesses, devices and anything else to identify themselves online, while remaining fully in control of this process.


> WARNING: THE CURRENT VERSION IS FEATURE INCOMPLETE AND WILL STILL UNDERGO MASSIVE CHANGES If you are interested in using this project or contributing, join our Discord and visit the channel #digital-id.

## Library Structure
<div align = center>
  <img src="assets/structure.svg"/>
</div>
> structure.svg


## Functions
Implemented functions based on [identity.ts](https://github.com/iotaledger/identity.ts)

✅ [1/25]

```javascript
{
  DID: [Function: DID] { urlScheme: 'did', didMethod: 'IOTA' },
  DIDDocument: [Function: DIDDocument] {
    readDIDDocument: [Function],
    createDIDDocument: [Function]
  },
  Service: [Function: Service],
  BaseKeypair: [Function: BaseKeypair],
  RSAKeypair: [Function: RSAKeypair],
  ECDSAKeypair: [Function: ECDSAKeypair],
  Hash: [Function: Hash],
  CreateRandomDID: [Function: CreateRandomDID],
  CreateRandomDIDFromPublicKey: [Function: CreateRandomDIDFromPublicKey],
  DecodeProofDocument: [Function: DecodeProofDocument],
  SignDIDAuthentication: [Function: SignDIDAuthentication],
  VerifyDIDAuthentication: [Function: VerifyDIDAuthentication],
  GenerateRSAKeypair: [Function: GenerateRSAKeypair],
  GenerateECDSAKeypair: [Function: GenerateECDSAKeypair],
  ✅ GenerateSeed: [Function: GenerateSeed], 
  DIDPublisher: [Function: DIDPublisher],
  MAMSettings: [Function: MAMSettings],
  MAM_MODE: { PRIVATE: 'private', PUBLIC: 'public', RESTRICTED: 'restricted' },
  Proof: [Function: Proof],
  ProofTypeManager: [Function: ProofTypeManager] { GetInstance: [Function] },
  Credential: [Function: Credential] {
    Create: [Function],
    DecodeFromJSON: [Function]
  },
  Presentation: [Function: Presentation] {
    Create: [Function],
    DecodeFromJSON: [Function]
  },
  Schema: [Function: Schema],
  SchemaManager: [Function: SchemaManager] { GetInstance: [Function] },
  VerifiableCredential: [Function: VerifiableCredential] {
    Create: [Function],
    DecodeFromJSON: [Function]
  },
  VerifiablePresentation: [Function: VerifiablePresentation] {
    Create: [Function],
    DecodeFromJSON: [Function]
  }
}
```