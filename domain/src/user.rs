use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
        Error as ArgonError,
    },
    Argon2
};
use svc_std::{primitives::{Email, validation_error::Error as ValidationError, id::Uuid}, traits::{authenticatable::Authenticatable, validatable::Validatable}};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    HashingError(String),
    ValidationError(ValidationError),
}

impl From<ArgonError> for Error {
    fn from(value: ArgonError) -> Self {
        Self::HashingError(value.to_string())
    }
}

impl From<ValidationError> for Error {
    fn from(value: ValidationError) -> Self {
        Self::ValidationError(value)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for Error {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct User {
    id: Uuid,
    email: Email,
    password_hash: String,
}

impl User {
    fn builder() -> UserBuilder<HasId, NoEmail, NoPasswordHash> {
        UserBuilder { 
            id: HasId(Uuid::default()), 
            email: NoEmail, 
            password_hash: NoPasswordHash, 
        }
    }
}

impl Authenticatable<Error> for User {
    fn confirm_password(&self, password: &str) -> Result<(), Error> {
        let parsed_hash = PasswordHash::new(&self.password_hash)?;
        Argon2::default().verify_password(password.as_bytes(), &parsed_hash)?;
        Ok(())
    }
}

impl Validatable<Error> for User {
    fn validate(&self) -> svc_std::traits::validatable::Result<Error> {
        self.id.validate()?;
        self.email.validate()?;
        Ok(())
    }
}

pub struct NoId;
pub struct HasId(Uuid);

pub struct NoEmail;
pub struct HasEmail(Email);

pub struct NoPasswordHash;
pub struct HasPasswordHash(String);

pub struct UserBuilder<I, E, P> {
    id: I,
    email: E,
    password_hash: P,
}

impl<I, E, P> UserBuilder<I, E, P> {
    fn id_from_str(self, id: &'static str) -> Result<UserBuilder<HasId, E, P>, Error> {
        let Self { email, password_hash, .. } = self;
        Ok(UserBuilder { 
            id: HasId(id.try_into()?), 
            email,
            password_hash 
        })
    }

    fn id(self) -> Result<UserBuilder<HasId, E, P>, Error> {
        let Self { email, password_hash, .. } = self;
        Ok(UserBuilder { 
            id: HasId(Uuid::new()), 
            email,
            password_hash 
        })
    }
}

impl<I, P> UserBuilder<I, NoEmail, P> {
    fn email(self, email: &'static str) -> Result<UserBuilder<I, HasEmail, P>, Error> {
        let Self { id, password_hash, .. } = self;
        Ok(UserBuilder { 
            id, 
            email: HasEmail(Email::new(email)?),
            password_hash 
        })
    }
}

impl<I, E> UserBuilder<I, E, NoPasswordHash> {
    fn password(self, password: &'static str) -> Result<UserBuilder<I, E, HasPasswordHash>, Error> {
        let Self { id, email, .. } = self;
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(password.as_bytes(), &salt)?.to_string();

        Ok(UserBuilder { 
            id, 
            email: email,
            password_hash: HasPasswordHash(password_hash) 
        })
    }
}

impl UserBuilder<HasId, HasEmail, HasPasswordHash> {
    fn build(self) -> User {
        let Self { id, email, password_hash } = self;
        User{
            id: id.0, 
            email: email.0, 
            password_hash: password_hash.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_works() {
        let user = User::builder()
            .email("john.doe@example.com").unwrap()
            .password("testtest").unwrap()
            .build();
        assert!(user.confirm_password("testtest").is_ok());
    }
}