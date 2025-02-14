use soft_serialize::serialize::{aes_dec_ecb};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn wasm_serialize(
    ciphertext: Vec<u8>,
    key: Vec<u8>,
    padding: Option<String>,
) -> Result<Vec<u8>, JsValue> {
    let padding_str = padding.as_deref();
    aes_dec_ecb(&ciphertext, &key, padding_str).map_err(|e| JsValue::from_str(&e.to_string()))
}
