//! Cryptographic core for secure vault operations
//!
//! Provides AES-256-GCM encryption and Argon2id key derivation with
//! automatic memory zeroization and constant-time operations.

use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use argon2::{Argon2, ParamsBuilder, Version};
use bincode::{Decode, Encode};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use subtle::ConstantTimeEq;
use thiserror::Error;
use zeroize::{Zeroize, ZeroizeOnDrop};

pub const SALT_SIZE: usize = 32;
pub const NONCE_SIZE: usize = 12;
pub const KEY_SIZE: usize = 32;

#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("Encryption failed")]
    EncryptionFailed,
    #[error("Decryption failed")]
    DecryptionFailed,
    #[error("Key derivation failed: {0}")]
    KeyDerivationFailed(String),
    #[error("Invalid password")]
    InvalidPassword,
    #[error("Invalid key size")]
    InvalidKeySize,
}

/// Secure key material with automatic zeroization
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct SecureKey {
    key: [u8; KEY_SIZE],
}

impl SecureKey {
    pub fn as_bytes(&self) -> &[u8; KEY_SIZE] {
        &self.key
    }

    pub fn from_bytes(bytes: [u8; KEY_SIZE]) -> Self {
        Self { key: bytes }
    }
}

/// Argon2id parameters for key derivation
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct KdfParams {
    pub memory_cost_kb: u32,
    pub time_cost: u32,
    pub parallelism: u32,
}

impl Default for KdfParams {
    fn default() -> Self {
        Self {
            memory_cost_kb: 65536, // 64 MB
            time_cost: 3,
            parallelism: 4,
        }
    }
}

/// Generate a cryptographically secure random salt
pub fn generate_salt() -> [u8; SALT_SIZE] {
    let mut salt = [0u8; SALT_SIZE];
    OsRng.fill_bytes(&mut salt);
    salt
}

/// Generate a cryptographically secure random nonce
pub fn generate_nonce() -> [u8; NONCE_SIZE] {
    let mut nonce = [0u8; NONCE_SIZE];
    OsRng.fill_bytes(&mut nonce);
    nonce
}

/// Derive encryption key from password using Argon2id
pub fn derive_key(
    password: &[u8],
    salt: &[u8; SALT_SIZE],
    params: &KdfParams,
) -> Result<SecureKey, CryptoError> {
    let argon2_params = ParamsBuilder::new()
        .m_cost(params.memory_cost_kb)
        .t_cost(params.time_cost)
        .p_cost(params.parallelism)
        .output_len(KEY_SIZE)
        .build()
        .map_err(|e| CryptoError::KeyDerivationFailed(e.to_string()))?;

    let argon2 = Argon2::new(argon2::Algorithm::Argon2id, Version::V0x13, argon2_params);

    let mut key_bytes = [0u8; KEY_SIZE];
    argon2
        .hash_password_into(password, salt, &mut key_bytes)
        .map_err(|e| CryptoError::KeyDerivationFailed(e.to_string()))?;

    Ok(SecureKey::from_bytes(key_bytes))
}

/// Encrypt data using AES-256-GCM
pub fn encrypt(
    key: &SecureKey,
    nonce: &[u8; NONCE_SIZE],
    plaintext: &[u8],
) -> Result<Vec<u8>, CryptoError> {
    let cipher =
        Aes256Gcm::new_from_slice(key.as_bytes()).map_err(|_| CryptoError::InvalidKeySize)?;

    let nonce_obj = Nonce::from_slice(nonce);

    cipher
        .encrypt(nonce_obj, plaintext)
        .map_err(|_| CryptoError::EncryptionFailed)
}

/// Decrypt data using AES-256-GCM
pub fn decrypt(
    key: &SecureKey,
    nonce: &[u8; NONCE_SIZE],
    ciphertext: &[u8],
) -> Result<Vec<u8>, CryptoError> {
    let cipher =
        Aes256Gcm::new_from_slice(key.as_bytes()).map_err(|_| CryptoError::InvalidKeySize)?;

    let nonce_obj = Nonce::from_slice(nonce);

    cipher
        .decrypt(nonce_obj, ciphertext)
        .map_err(|_| CryptoError::DecryptionFailed)
}

/// Constant-time password verification
pub fn verify_password(
    password: &[u8],
    salt: &[u8; SALT_SIZE],
    expected_key: &SecureKey,
    params: &KdfParams,
) -> Result<bool, CryptoError> {
    let derived_key = derive_key(password, salt, params)?;

    // Constant-time comparison to prevent timing attacks
    let result = derived_key.as_bytes().ct_eq(expected_key.as_bytes());

    Ok(result.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_derivation() {
        let password = b"test_password_123";
        let salt = generate_salt();
        let params = KdfParams::default();

        let key = derive_key(password, &salt, &params).unwrap();
        assert_eq!(key.as_bytes().len(), KEY_SIZE);
    }

    #[test]
    fn test_encryption_decryption() {
        let key = SecureKey::from_bytes([42u8; KEY_SIZE]);
        let nonce = generate_nonce();
        let plaintext = b"Hello, World!";

        let ciphertext = encrypt(&key, &nonce, plaintext).unwrap();
        assert_ne!(ciphertext.as_slice(), plaintext);

        let decrypted = decrypt(&key, &nonce, &ciphertext).unwrap();
        assert_eq!(decrypted.as_slice(), plaintext);
    }

    #[test]
    fn test_tampering_detection() {
        let key = SecureKey::from_bytes([42u8; KEY_SIZE]);
        let nonce = generate_nonce();
        let plaintext = b"Secret data";

        let mut ciphertext = encrypt(&key, &nonce, plaintext).unwrap();

        // Tamper with ciphertext
        ciphertext[0] ^= 1;

        let result = decrypt(&key, &nonce, &ciphertext);
        assert!(result.is_err());
    }

    #[test]
    fn test_password_verification() {
        let password = b"correct_password";
        let salt = generate_salt();
        let params = KdfParams::default();

        let key = derive_key(password, &salt, &params).unwrap();

        assert!(verify_password(password, &salt, &key, &params).unwrap());
        assert!(!verify_password(b"wrong_password", &salt, &key, &params).unwrap());
    }
}
