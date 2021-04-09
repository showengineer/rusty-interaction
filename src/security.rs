use ed25519_dalek::Verifier;
use ed25519_dalek::SIGNATURE_LENGTH;
use ed25519_dalek::{PublicKey, Signature};

use hex;
use std::convert::TryInto;

fn convert_to_arr<T>(v: Vec<T>) -> [T; SIGNATURE_LENGTH] {
    v.try_into().unwrap_or_else(|v: Vec<T>| {
        panic!(
            "Expected a Vec of length {} but it was {}",
            SIGNATURE_LENGTH,
            v.len()
        )
    })
}

pub enum ValidationError {
    // For anything related to conversion errors
    KeyConversionError { name: &'static str },
    // For invalid keys
    InvalidSignatureError,
}

/// Verifies an incoming Interaction.
/// This verification is mandatory for every incoming Interaction.
/// See [the developer docs](https://discord.com/developers/docs/interactions/slash-commands#security-and-authorization) for more info
pub fn verify_discord_message(
    public_key: PublicKey,
    signature: &str,
    timestamp: &str,
    body: &String,
) -> Result<(), ValidationError> {
    // Format the data to verify (Timestamp + body)
    let msg = format!("{}{}", timestamp, body);
    match hex::decode(signature) {
        Err(_) => {
            return Err(ValidationError::KeyConversionError { name: "Signature" });
        }
        Ok(s) => {
            let sa = convert_to_arr(s);
            let sign = Signature::from(sa);
            match public_key.verify(msg.as_bytes(), &sign) {
                Err(_) => {
                    return Err(ValidationError::InvalidSignatureError);
                }
                Ok(()) => {
                    return Ok(());
                }
            }
        }
    }
}
