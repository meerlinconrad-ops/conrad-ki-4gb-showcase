//! OpenSSL-kompatible AES-256-CBC Verschluesselung fuer n8n Credentials.
//!
//! Format: `base64("Salted__" || 8-Byte Salt || Ciphertext)`, Schluesselableitung via
//! OpenSSL `EVP_BytesToKey(MD5, count=1)` -> 32-Byte AES-256 Key + 16-Byte IV.
//! 1:1 drahtkompatibel zu `n8n-core` (`Cipher.ts`) und `openssl enc -aes-256-cbc -md md5`.

use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use base64::{engine::general_purpose::STANDARD as B64, Engine as _};
use md5::{Digest, Md5};
use rand::RngCore;
use serde_json::Value;
use thiserror::Error;

type Aes256CbcEnc = cbc::Encryptor<aes::Aes256>;
type Aes256CbcDec = cbc::Decryptor<aes::Aes256>;

const SALTED_MAGIC: &[u8; 8] = b"Salted__";
const DEFAULT_N8N_KEY: &str = "conrad-ki-n8n-default-key";

#[derive(Debug, Error)]
pub enum CipherError {
    #[error("Ungueltiges Base64-Format in verschluesselten Anmeldedaten")]
    InvalidBase64,
    #[error("Verschluesselte Anmeldedaten sind zu kurz oder ohne Salted__-Header")]
    InvalidHeader,
    #[error("Entschluesselung fehlgeschlagen (falscher N8N_ENCRYPTION_KEY oder korruptes Padding)")]
    DecryptFailed,
    #[error("JSON-Fehler in Anmeldedaten: {0}")]
    Json(#[from] serde_json::Error),
}

/// Liest den aktiven `N8N_ENCRYPTION_KEY` aus der Umgebung oder nutzt einen deterministischen Fallback.
pub fn encryption_key() -> String {
    std::env::var("N8N_ENCRYPTION_KEY")
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| DEFAULT_N8N_KEY.to_string())
}

/// OpenSSL EVP_BytesToKey mit MD5 (1 Iteration) fuer AES-256-CBC (32-Byte Key, 16-Byte IV).
fn evp_bytes_to_key(passphrase: &[u8], salt: &[u8; 8]) -> ([u8; 32], [u8; 16]) {
    let mut blocks = Vec::<u8>::with_capacity(48);
    let mut prev: Vec<u8> = Vec::new();
    while blocks.len() < 48 {
        let mut hasher = Md5::new();
        if !prev.is_empty() {
            hasher.update(&prev);
        }
        hasher.update(passphrase);
        hasher.update(salt);
        let d = hasher.finalize();
        blocks.extend_from_slice(&d);
        prev = d.to_vec();
    }
    let mut key = [0u8; 32];
    let mut iv = [0u8; 16];
    key.copy_from_slice(&blocks[0..32]);
    iv.copy_from_slice(&blocks[32..48]);
    (key, iv)
}

pub fn encrypt(passphrase: &str, plaintext: &str) -> String {
    let mut salt = [0u8; 8];
    rand::thread_rng().fill_bytes(&mut salt);
    let (key, iv) = evp_bytes_to_key(passphrase.as_bytes(), &salt);

    let pt = plaintext.as_bytes();
    let mut buf = vec![0u8; pt.len() + 16];
    buf[..pt.len()].copy_from_slice(pt);
    let ct = Aes256CbcEnc::new(&key.into(), &iv.into())
        .encrypt_padded_mut::<Pkcs7>(&mut buf, pt.len())
        .expect("AES-256-CBC buffer sized with +16 bytes cannot fail");

    let mut out = Vec::with_capacity(16 + ct.len());
    out.extend_from_slice(SALTED_MAGIC);
    out.extend_from_slice(&salt);
    out.extend_from_slice(ct);
    B64.encode(out)
}

pub fn decrypt(passphrase: &str, encoded: &str) -> Result<String, CipherError> {
    let raw = B64.decode(encoded.trim()).map_err(|_| CipherError::InvalidBase64)?;
    if raw.len() < 16 || &raw[0..8] != SALTED_MAGIC {
        return Err(CipherError::InvalidHeader);
    }
    let mut salt = [0u8; 8];
    salt.copy_from_slice(&raw[8..16]);
    let (key, iv) = evp_bytes_to_key(passphrase.as_bytes(), &salt);

    let mut buf = raw[16..].to_vec();
    let pt = Aes256CbcDec::new(&key.into(), &iv.into())
        .decrypt_padded_mut::<Pkcs7>(&mut buf)
        .map_err(|_| CipherError::DecryptFailed)?;
    String::from_utf8(pt.to_vec()).map_err(|_| CipherError::DecryptFailed)
}

pub fn encrypt_json(passphrase: &str, value: &Value) -> Result<String, CipherError> {
    let s = serde_json::to_string(value)?;
    Ok(encrypt(passphrase, &s))
}

pub fn decrypt_json(passphrase: &str, encoded: &str) -> Result<Value, CipherError> {
    let s = decrypt(passphrase, encoded)?;
    Ok(serde_json::from_str(&s)?)
}

/// Akzeptiert optional auch unverschluesseltes Legacy-JSON beim Lesen, speichert aber immer verschluesselt.
pub fn decrypt_json_lenient(passphrase: &str, stored: &str) -> Result<Value, CipherError> {
    let trimmed = stored.trim();
    if trimmed.is_empty() {
        return Ok(serde_json::json!({}));
    }
    if trimmed.starts_with('{') || trimmed.starts_with('[') {
        if let Ok(v) = serde_json::from_str::<Value>(trimmed) {
            return Ok(v);
        }
    }
    decrypt_json(passphrase, trimmed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_salted_openssl_format() {
        let secret = serde_json::json!({"apiKey": "sk-test-12345", "baseUrl": "https://api.example.com"});
        let enc = encrypt_json("my-secret-key", &secret).unwrap();
        let raw = B64.decode(&enc).unwrap();
        assert_eq!(&raw[0..8], b"Salted__");
        let dec = decrypt_json("my-secret-key", &enc).unwrap();
        assert_eq!(dec, secret);
        assert!(decrypt_json("wrong-key", &enc).is_err());
    }
}
