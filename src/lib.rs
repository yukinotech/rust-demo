use napi::bindgen_prelude::*;
use napi_derive::napi;

/// Convert the input string to uppercase and trim surrounding whitespace.
/// JavaScript can call this via N-API.
#[napi]
pub fn shout(input: String) -> Result<String> {
    // Normalize whitespace and emphasize the message.
    let normalized = input.trim().to_uppercase();
    Ok(normalized)
}
