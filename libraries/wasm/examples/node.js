const did = require('../wasm-node/iota_did_wasm')
console.log(did)

const greet = did.Greet()

console.log("greet: ", greet)

const seed = did.GenerateSeed()

console.log("seed: ", seed)