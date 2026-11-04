use std::sync::LazyLock;
use crypto_common::{InvalidLength};
use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;
use tracing::instrument;

type HmacSha256 = Hmac<Sha256>;

pub static HMAC_KEY: LazyLock<Vec<u8>> = 
    LazyLock::new(initialize_hmac_key);

#[instrument]
fn initialize_hmac_key() -> Vec<u8> {
    dotenvy::dotenv().ok();

    let key = std::env::var("HMAC_KEY")
        .expect("HMAC_KEY no está configurada");

    assert_eq!(
    key.len(),
    64,
    "HMAC_KEY debe contener exactamente 64 caracteres hexadecimales");

    hex::decode(key).expect("HMAC_KEY no es hexadecimal válido")
}

#[derive(Debug, thiserror::Error)]
pub enum HMACError {
    #[error("Longitud de la cllave HMAC inválida")]
    InvalidLength(#[from] InvalidLength),
}

#[instrument(err)]
pub fn sign_email(email: &str) -> Result<String, HMACError>{
    let mut mac = HmacSha256::new_from_slice(&HMAC_KEY)?;
    mac.update(email.as_bytes());
    
    let result = mac.finalize();
    Ok(hex::encode(result.into_bytes()))
} 