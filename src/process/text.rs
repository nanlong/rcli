use std::io::Read;

use crate::{decode_hex, encode_hex, TextSignFormat};
use anyhow::Result;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier};
use rand::rngs::OsRng;

pub struct Blake3 {
    key: [u8; 32],
}

impl Blake3 {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    pub fn try_new(key: &str) -> Result<Self> {
        let key = decode_hex(key)?.as_slice().try_into()?;
        Ok(Self::new(key))
    }

    pub fn generate_key() -> Result<String> {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        Ok(encode_hex(&signing_key.to_bytes()))
    }
}

pub struct Ed25519 {
    signing_key: SigningKey,
}

impl Ed25519 {
    pub fn new(key: [u8; 32]) -> Self {
        Self {
            signing_key: SigningKey::from_bytes(&key),
        }
    }

    pub fn try_new(key: &str) -> Result<Self> {
        let key = decode_hex(key)?.as_slice().try_into()?;
        let signing_key = SigningKey::from_keypair_bytes(&key)?;
        Ok(Self { signing_key })
    }

    pub fn generate_key() -> Result<String> {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        Ok(encode_hex(&signing_key.to_keypair_bytes()))
    }
}

pub trait TextSigner {
    fn sign(&self, msg: &str) -> String;
}

pub trait TextVerifier: TextSigner {
    fn verify(&self, msg: &str, signature: &str) -> Result<bool>;
}

impl TextSigner for Blake3 {
    fn sign(&self, msg: &str) -> String {
        let signed = blake3::keyed_hash(&self.key, msg.as_bytes());
        encode_hex(signed.as_bytes())
    }
}

impl TextVerifier for Blake3 {
    fn verify(&self, msg: &str, signature: &str) -> Result<bool> {
        Ok(self.sign(msg) == signature)
    }
}

impl TextSigner for Ed25519 {
    fn sign(&self, msg: &str) -> String {
        let signed = self.signing_key.sign(msg.as_bytes());
        encode_hex(signed.to_bytes().as_slice())
    }
}

impl TextVerifier for Ed25519 {
    fn verify(&self, msg: &str, signature: &str) -> Result<bool> {
        let signature = decode_hex(signature)?;
        let signature = Signature::from_bytes(signature.as_slice().try_into()?);
        Ok(self
            .signing_key
            .verifying_key()
            .verify(msg.as_bytes(), &signature)
            .is_ok())
    }
}

pub fn process_sign(input: &mut dyn Read, key: &str, format: TextSignFormat) -> Result<String> {
    let mut buf = String::new();
    input.read_to_string(&mut buf)?;
    let buf = buf.trim();

    match format {
        TextSignFormat::Blake3 => {
            let blake3 = Blake3::try_new(key)?;
            Ok(blake3.sign(buf))
        }
        TextSignFormat::Ed25519 => {
            let ed25519 = Ed25519::try_new(key)?;
            Ok(ed25519.sign(buf))
        }
    }
}

pub fn process_verify(
    input: &mut dyn Read,
    key: &str,
    sig: &str,
    format: TextSignFormat,
) -> Result<bool> {
    let mut buf = String::new();
    input.read_to_string(&mut buf)?;
    let buf = buf.trim();

    match format {
        TextSignFormat::Blake3 => {
            let blake3 = Blake3::try_new(key)?;
            Ok(blake3.verify(buf, sig)?)
        }
        TextSignFormat::Ed25519 => {
            let ed25519 = Ed25519::try_new(key)?;
            Ok(ed25519.verify(buf, sig)?)
        }
    }
}

pub fn process_generate(format: TextSignFormat) -> String {
    match format {
        TextSignFormat::Blake3 => Blake3::generate_key().unwrap(),
        TextSignFormat::Ed25519 => Ed25519::generate_key().unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blake3_generate_key() {
        let key = Blake3::generate_key();
        assert!(key.is_ok());
    }

    #[test]
    fn test_blake3_try_new() {
        let key = Blake3::generate_key().unwrap();
        let blake3 = Blake3::try_new(&key);
        assert!(blake3.is_ok());
    }

    #[test]
    fn test_blake3_sign_and_verify() {
        let key = Blake3::generate_key().unwrap();
        let blake3 = Blake3::try_new(&key).unwrap();
        let msg = "hello world";
        let signature = blake3.sign(msg);
        assert!(blake3.verify(msg, &signature).unwrap());
    }

    #[test]
    fn test_ed25519_generate_key() {
        let key = Ed25519::generate_key();
        assert!(key.is_ok());
    }

    #[test]
    fn test_ed25519_try_new() {
        let key = Ed25519::generate_key().unwrap();
        let edd25519 = Ed25519::try_new(&key);
        assert!(edd25519.is_ok());
    }

    #[test]
    fn test_ed25519_sign_and_verify() {
        let key = Ed25519::generate_key().unwrap();
        let edd25519 = Ed25519::try_new(&key).unwrap();
        let msg = "hello world";
        let signature = edd25519.sign(msg);
        assert!(edd25519.verify(msg, &signature).unwrap());
    }
}
