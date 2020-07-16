import("../pkg/index.js").then((did) => {

    console.log(did)

    const greet = did.Greet()
    
    console.log("greet: ", greet)
    
    const seed = did.GenerateSeed()
    
    console.log("seed: ", seed)
});
