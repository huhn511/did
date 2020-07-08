use wasm_bindgen::prelude::*;
use did::utils::generate_seed;


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    pub fn error(s: &str);
}

macro_rules! console_log {
  ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

macro_rules! console_error {
  ($($t:tt)*) => (error(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen(js_name = "GenerateSeed")]
pub fn generate_seed() -> Result<String, JsValue> {
    console_error_panic_hook::set_once();

    let seed = generate_seed::new();
    println!("Seed: {:?}", seed);
    Ok(seed)
  }
  
  #[wasm_bindgen(js_name = "Greet")]
  pub fn greet() -> Result<String, JsValue> {
    console_error_panic_hook::set_once();
    let greet = did::greet();
    println!("greet: {:?}", greet);
    Ok(greet)
}