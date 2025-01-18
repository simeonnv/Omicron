
use argon2::{
    password_hash::{
        rand_core::OsRng, PasswordHasher, SaltString, Error
    },
    Argon2
};


pub async fn hash(input: &String) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    
    let password_hash = argon2.hash_password(input.as_bytes(), &salt)?.to_string();
    
    Ok(password_hash)
}