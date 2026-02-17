//! Encryption utilities for storing secrets at rest.
//!
//! Uses AES-256-GCM for authenticated encryption. The encryption key should be
//! stored in the ENCRYPTION_KEY environment variable as a 32-byte base64-encoded string.

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use rand::Rng;

/// Encrypts a plaintext value using AES-256-GCM.
///
/// Returns the nonce (12 bytes) prepended to the ciphertext.
/// The result is suitable for storing in a BLOB column.
pub fn encrypt(plaintext: &str, key: &[u8; 32]) -> Result<Vec<u8>, CryptoError> {
    let cipher = Aes256Gcm::new_from_slice(key).map_err(|_| CryptoError::InvalidKey)?;

    // Generate a random 12-byte nonce
    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext.as_bytes())
        .map_err(|_| CryptoError::EncryptionFailed)?;

    // Prepend nonce to ciphertext
    let mut result = Vec::with_capacity(12 + ciphertext.len());
    result.extend_from_slice(&nonce_bytes);
    result.extend_from_slice(&ciphertext);

    Ok(result)
}

/// Decrypts a value that was encrypted with `encrypt`.
///
/// Expects the input to be nonce (12 bytes) + ciphertext.
pub fn decrypt(encrypted: &[u8], key: &[u8; 32]) -> Result<String, CryptoError> {
    if encrypted.len() < 12 {
        return Err(CryptoError::InvalidCiphertext);
    }

    let cipher = Aes256Gcm::new_from_slice(key).map_err(|_| CryptoError::InvalidKey)?;

    let nonce = Nonce::from_slice(&encrypted[..12]);
    let ciphertext = &encrypted[12..];

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| CryptoError::DecryptionFailed)?;

    String::from_utf8(plaintext).map_err(|_| CryptoError::InvalidUtf8)
}

/// Parses a base64-encoded encryption key from an environment variable.
pub fn parse_key(base64_key: &str) -> Result<[u8; 32], CryptoError> {
    let decoded = BASE64
        .decode(base64_key)
        .map_err(|_| CryptoError::InvalidKey)?;

    if decoded.len() != 32 {
        return Err(CryptoError::InvalidKey);
    }

    let mut key = [0u8; 32];
    key.copy_from_slice(&decoded);
    Ok(key)
}

/// Generates a new random encryption key and returns it as base64.
/// Useful for initial setup.
#[allow(dead_code)]
pub fn generate_key() -> String {
    let mut key = [0u8; 32];
    rand::thread_rng().fill(&mut key);
    BASE64.encode(key)
}

#[derive(Debug)]
pub enum CryptoError {
    InvalidKey,
    EncryptionFailed,
    DecryptionFailed,
    InvalidCiphertext,
    InvalidUtf8,
}

impl std::fmt::Display for CryptoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CryptoError::InvalidKey => write!(f, "Invalid encryption key"),
            CryptoError::EncryptionFailed => write!(f, "Encryption failed"),
            CryptoError::DecryptionFailed => write!(f, "Decryption failed"),
            CryptoError::InvalidCiphertext => write!(f, "Invalid ciphertext"),
            CryptoError::InvalidUtf8 => write!(f, "Invalid UTF-8 in decrypted data"),
        }
    }
}

impl std::error::Error for CryptoError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let key = [0u8; 32]; // Test key (don't use all zeros in production!)
        let plaintext = "my-secret-api-key-12345";

        let encrypted = encrypt(plaintext, &key).unwrap();
        let decrypted = decrypt(&encrypted, &key).unwrap();

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_different_nonces() {
        let key = [0u8; 32];
        let plaintext = "test";

        let encrypted1 = encrypt(plaintext, &key).unwrap();
        let encrypted2 = encrypt(plaintext, &key).unwrap();

        // Same plaintext should produce different ciphertext due to random nonce
        assert_ne!(encrypted1, encrypted2);

        // But both should decrypt to the same value
        assert_eq!(decrypt(&encrypted1, &key).unwrap(), plaintext);
        assert_eq!(decrypt(&encrypted2, &key).unwrap(), plaintext);
    }

    #[test]
    fn test_wrong_key_fails() {
        let key1 = [0u8; 32];
        let key2 = [1u8; 32];
        let plaintext = "secret";

        let encrypted = encrypt(plaintext, &key1).unwrap();
        let result = decrypt(&encrypted, &key2);

        assert!(result.is_err());
    }

    #[test]
    fn test_generate_and_parse_key() {
        let base64_key = generate_key();
        let key = parse_key(&base64_key).unwrap();

        // Should be 32 bytes
        assert_eq!(key.len(), 32);

        // Roundtrip test with the generated key
        let plaintext = "test-value";
        let encrypted = encrypt(plaintext, &key).unwrap();
        let decrypted = decrypt(&encrypted, &key).unwrap();
        assert_eq!(decrypted, plaintext);
    }
}
