use std::{sync::LazyLock, convert::{TryFrom, Infallible},array::TryFromSliceError};
use aes_gcm::{aead::{Aead, KeyInit}, Aes256Gcm, Nonce};
use crypto_common::InvalidLength;
use hex::FromHexError;
use rand::TryRng;


pub static AES_KEY: LazyLock<Vec<u8>> = LazyLock::new(|| {
    dotenvy::dotenv().ok();

    let key = std::env::var("AES_KEY")
        .expect("AES_KEY no está configurada");

    assert_eq!(
    key.len(),
    64,
    "AES_KEY debe contener exactamente 64 caracteres hexadecimales");

    hex::decode(key).expect("AES_KEY no es hexadecimal válido")
});

#[derive(Debug, thiserror::Error)]
pub enum EncryptionError{
    #[error("La clave no tiene el tamaño esperado")]
    InvalidLength(#[from] InvalidLength),

    #[error("El texto de descifrar no tiene la longitud esperada")]
    InvalidLengthError,

    #[error("No se pudo cifrar")]
    CryptoOperationFailed(#[from] aes_gcm::Error),

    #[error("Los datos descifrados no son UTF-8 valido")]
    InvalidUtf8(#[from] std::string::FromUtf8Error),

    #[error("No se pudo rellenar el arreglo de bytes")]
    Infalible(#[from] Infallible),

    #[error("Error al convertir el nonce")]
    NonceConversionError(#[from]TryFromSliceError),

    #[error("Error al decodificar el hexadecimal")]
    FromHexError(#[from]FromHexError),
}

pub fn encrypt_email(email: &str) -> Result<String, EncryptionError> {

    let mut nonce_bytes = [0u8; 12];
    let mut rand = rand::rng();
    
    rand.try_fill_bytes(&mut nonce_bytes)?;
    let crypto = Aes256Gcm::new_from_slice(&AES_KEY)?;

    let nonce = Nonce::try_from(&nonce_bytes[..])?;

    let result = crypto.encrypt(&nonce, email.as_bytes())?;
    let mut cyphertext = Vec::with_capacity(nonce_bytes.len() + result.len());

    cyphertext.extend_from_slice(&nonce_bytes);
    cyphertext.extend_from_slice(&result);

    Ok(hex::encode(cyphertext))
}

pub fn decrypt_email(ciphertext: &str) -> Result<String, EncryptionError> {
    let cyphertext = hex::decode(ciphertext)?;
    let crypto = Aes256Gcm::new_from_slice(&AES_KEY)?;

    if cyphertext.len() < 28 {
        return Err(EncryptionError::InvalidLengthError);
    }

    let (nonce_bytes, encrypted_data) = cyphertext.split_at(12);
    let nonce = Nonce::try_from(nonce_bytes)?;

    let reslt = crypto.decrypt(&nonce, encrypted_data)?;

    Ok(String::from_utf8(reslt)?)

}