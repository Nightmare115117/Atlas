use argon2::{
    Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version,
    password_hash,
};
use tracing::instrument;

#[derive(Debug, thiserror::Error)]
pub enum Argon2IdError {
    #[error("Error durante la operación Argon2id: {0}")]
    Argon2(#[from] argon2::Error),

    #[error("Error al hashear o verificar contraseña")]
    PasswordHash(#[from] password_hash::Error),

    #[error("Error al convertir al objeto PasswordHash")]
    Parse(#[from] password_hash::phc::Error),
}

#[instrument(err)]
fn config_hash() -> Result<Argon2<'static>, Argon2IdError> {
    let param = Params::new(19456, 2, 1, Some(32))?;
    let algorithm = Algorithm::Argon2id;
    let version = Version::V0x13;

    Ok(Argon2::new(algorithm, version, param))
}

#[instrument(err)]
pub fn hash_password(pass: &str) -> Result<String, Argon2IdError> {
    let hash = config_hash()?;

    let pass_hash = PasswordHasher::hash_password(&hash, pass.as_bytes())?;
    Ok(pass_hash.to_string())
}

#[instrument(err)]
pub fn verify_password(pass: &str, hash_pass: &str) -> Result<bool, Argon2IdError> {
    let hash = config_hash()?;
    let password_hash = PasswordHash::new(hash_pass)?;

    match PasswordVerifier::verify_password(&hash, pass.as_bytes(), &password_hash) {
        Ok(_) => Ok(true),
        Err(error) => Err(Argon2IdError::PasswordHash(error)),
    }
}
