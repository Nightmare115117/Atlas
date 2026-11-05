pub mod aes;
pub mod hmac;
pub mod argon2id;

pub use aes::{encrypt_email, decrypt_email, EncryptionError};
pub use hmac::{sign_email, HMACError};
pub use argon2id::{Argon2IdError, hash_password, verify_password};