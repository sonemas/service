use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher as CorePasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use super::{PasswordHasher, Error};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Argon2PasswordHasher;

impl PasswordHasher for Argon2PasswordHasher {
    fn hash(input: &str) -> Result<String, Error> {
        let salt = SaltString::generate(&mut OsRng);
        Ok(Argon2::default().hash_password(input.as_bytes(), &salt)?.to_string())
    }

    fn confirm_password(password: &str, hash: &str) ->  Result<(), Error> {
        let parsed_hash = PasswordHash::new(&hash)?;
        Argon2::default().verify_password(password.as_bytes(), &parsed_hash)?;
        Ok(())
    }
}