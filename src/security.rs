use ed25519_dalek::Verifier;
use ed25519_dalek::SIGNATURE_LENGTH;
use ed25519_dalek::{PublicKey, Signature};

use hex;
use std::convert::TryInto;

#[doc(hidden)]
/// Simple vector to array convertor.
pub fn convert_to_arr<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

/// If verification failes, it will return the `ValidationError` enum.
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
    body: &str,
) -> Result<(), ValidationError> {
    // Format the data to verify (Timestamp + body)
    let msg = format!("{}{}", timestamp, body);
    match hex::decode(signature) {
        Err(_) => Err(ValidationError::KeyConversionError { name: "Signature" }),
        Ok(s) => {
            if s.len() != SIGNATURE_LENGTH {
                return Err(ValidationError::KeyConversionError {
                    name: "Signature Length",
                });
            }
            let sa = convert_to_arr(s);
            let sign = Signature::from(sa);
            match public_key.verify(msg.as_bytes(), &sign) {
                Err(_) => Err(ValidationError::InvalidSignatureError),
                Ok(()) => Ok(()),
            }
        }
    }
}
