use anyhow::anyhow;
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
pub async fn hash_password(password: String) -> anyhow::Result<String> {
    let password_hash:anyhow::Result<String>= tokio::spawn(async move {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow!(e))?
            .to_string();
        Ok(password_hash)
    }).await.map_err(|e|anyhow!(e))?;
    password_hash
}
pub async fn verify(password: String, hashed: String) -> anyhow::Result<()> {
    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(&hashed).map_err(|e| anyhow!(e))?;
    argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|e| anyhow!(e))?;
    Ok(())
}