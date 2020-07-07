const did = require('../wasm-node/iota_did_wasm')
console.log(did)

const seed = did.GenerateSeed()

console.log("seed: ", seed)