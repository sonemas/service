pub use super::Error;

pub trait PasswordHasher {
    fn hash(input: &str) -> Result<String, Error>;
    fn confirm_password(password: &str, hash: &str) ->  Result<(), Error>;
}