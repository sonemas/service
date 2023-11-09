use svc_std::{primitives::{Email, error::Error as ValidationError, id::Uuid, Password}, traits::{authenticatable::Authenticatable, password_hasher::{PasswordHasher, argon2::Argon2PasswordHasher}}};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    HashingError(String),
    ValidationError(ValidationError),
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
    password: Password<Argon2PasswordHasher>,
}

impl User {
    pub fn builder() -> UserBuilder<HasId, NoEmail, NoPassword> {
        UserBuilder { 
            id: HasId(Uuid::default()), 
            email: NoEmail, 
            password: NoPassword, 
        }
    }
}

impl Authenticatable<Error> for User {
    fn confirm_password(&self, password: &str) -> Result<(), Error> {
        Ok(self.password.confirm(password)?)
    }
}

#[derive(Debug, PartialEq)]
pub struct NoId;
#[derive(Debug, PartialEq)]
pub struct HasId(Uuid);

#[derive(Debug, PartialEq)]
pub struct NoEmail;
#[derive(Debug, PartialEq)]
pub struct HasEmail(Email);

#[derive(Debug, PartialEq)]
pub struct NoPassword;
#[derive(Debug, PartialEq)]
pub struct HasPassword(Password<Argon2PasswordHasher>);

#[derive(Debug, PartialEq)]
pub struct UserBuilder<I, E, P> {
    id: I,
    email: E,
    password: P,
}

impl<I, E, P> UserBuilder<I, E, P> {
    pub fn id_from_str(self, id: &'static str) -> Result<UserBuilder<HasId, E, P>, Error> {
        let Self { email, password, .. } = self;
        Ok(UserBuilder { 
            id: HasId(id.try_into()?), 
            email,
            password 
        })
    }

    pub fn id(self) -> Result<UserBuilder<HasId, E, P>, Error> {
        let Self { email, password, .. } = self;
        Ok(UserBuilder { 
            id: HasId(Uuid::new()), 
            email,
            password 
        })
    }
}

impl<I, P> UserBuilder<I, NoEmail, P> {
    pub fn email(self, email: &'static str) -> Result<UserBuilder<I, HasEmail, P>, Error> {
        let Self { id, password, .. } = self;
        Ok(UserBuilder { 
            id, 
            email: HasEmail(Email::new(email)?),
            password 
        })
    }
}

impl<I, E> UserBuilder<I, E, NoPassword> {
    pub fn password(self, password: &'static str) -> Result<UserBuilder<I, E, HasPassword>, Error> {
        let Self { id, email, .. } = self;
        let password = Password::new(password)?;

        Ok(UserBuilder { 
            id, 
            email: email,
            password: HasPassword(password) 
        })
    }
}

impl UserBuilder<HasId, HasEmail, HasPassword> {
    pub fn build(self) -> User {
        let Self { id, email, password } = self;
        User{
            id: id.0, 
            email: email.0, 
            password: password.0
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::user;

    use super::*;

    #[test]
    fn user_works() {
        let user = User::builder()
            .email("john.doe@example.com").unwrap()
            .password("testtest").unwrap()
            .build();
        assert!(user.confirm_password("testtest").is_ok());
    }

    #[test]
    fn user_validation_works() {
        assert_eq!(User::builder().email("blabla"), Err(user::Error::ValidationError(ValidationError::InvalidEmailAddress)));
        assert_eq!(User::builder().id_from_str("blabla"), Err(user::Error::ValidationError(ValidationError::InvalidID)));
    }
}