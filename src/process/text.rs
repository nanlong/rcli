use std::io::Read;

use crate::{decode_hex, encode_hex, TextSignFormat};
use anyhow::Result;
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit},
    ChaCha20Poly1305, Nonce,
};
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

pub fn process_encrypt(input: &mut dyn Read, key: &str) -> Result<String> {
    let mut buf = String::new();
    input.read_to_string(&mut buf)?;
    let buf = buf.trim();

    let cipher = ChaCha20Poly1305::new_from_slice(decode_hex(key)?.as_slice())?;
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);

    let ciphertext = cipher
        .encrypt(&nonce, buf.as_bytes())
        .map_err(|_| anyhow::anyhow!("encrypt error"))?;

    let mut output = Vec::new();
    output.extend_from_slice(nonce.as_slice());
    output.extend_from_slice(ciphertext.as_slice());

    Ok(encode_hex(output.as_slice()))
}

pub fn process_decrypt(input: &mut dyn Read, key: &str) -> Result<String> {
    let mut buf = String::new();
    input.read_to_string(&mut buf)?;
    let buf = decode_hex(buf.trim())?;

    let cipher = ChaCha20Poly1305::new_from_slice(decode_hex(key)?.as_slice())?;
    let nonce = Nonce::from_slice(&buf[0..12]);

    let plaintext = cipher
        .decrypt(nonce, &buf[12..])
        .map_err(|_| anyhow::anyhow!("decrypt error"))?;

    Ok(String::from_utf8(plaintext)?)
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

    #[test]
    fn test_encrypt() {
        let input = &mut "你好，世界！".as_bytes();
        let key = "d15b212054ab60da12d67534d79d06f432bc1d7be2b5902297189639078c4a38";
        let encrypted = process_encrypt(input, key).unwrap();
        let descrypted = process_decrypt(&mut encrypted.as_bytes(), key).unwrap();
        assert_eq!(descrypted, "你好，世界！");
    }

    #[test]
    fn test_decrypt() {
        let input = &mut "2a49e9b2deb0f9c8cd440699f6e22757249a5f924656fbc8f420ed7978df53d89d51964ae6a76a1d647beff3be46".as_bytes();
        let key = "d15b212054ab60da12d67534d79d06f432bc1d7be2b5902297189639078c4a38";
        let resp = process_decrypt(input, key).unwrap();
        assert_eq!(resp, "你好，世界！");
    }
}
