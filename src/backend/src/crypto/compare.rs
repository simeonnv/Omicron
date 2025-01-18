use argon2::{
    password_hash::{
        Error, PasswordHash, PasswordVerifier
    },
    Argon2
};

pub async fn compare(input: &String, hash: &String) -> Result<bool, Error> {
    let parsed_hash = PasswordHash::new(hash)?;

    let is_correct = Argon2::default().verify_password(input.as_bytes(), &parsed_hash).is_ok();

    Ok(is_correct)
}