use argon2::{
    Argon2,
    password_hash::{Error, PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

pub fn hash_password(password: &str) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default().hash_password(password.as_bytes(), &salt)?;
    Ok(hash.to_string())
}

pub fn verify_password(plain: &str, hashed: &str) -> Result<bool, Error> {
    let parsed = PasswordHash::new(hashed)?;
    Ok(Argon2::default().verify_password(plain.as_bytes(), &parsed).is_ok())
}
