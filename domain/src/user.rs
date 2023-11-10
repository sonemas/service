#![allow(dead_code)] // TODO: Remove once user is implemented elsewhere.
use std::time::SystemTime;

use svc_std::{
    password_hasher::argon2::Argon2PasswordHasher,
    primitives::{error::Error as StdError, id::Uuid, Email, Password, ValidationError},
    traits::authenticatable::Authenticatable,
};

/// User errors.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    Validation(ValidationError),
    Authentication,

    /// Used to collect errors that normally shouldn't occur.
    Other(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for Error {}

impl From<StdError> for Error {
    fn from(value: StdError) -> Self {
        match value {
            StdError::InvalidPassword => Self::Authentication,
            StdError::Validation(err) => Self::Validation(err),
            StdError::PasswordHashingError(err) => {
                Self::Other(format!("password hashing error: {}", err))
            }
            StdError::RegexError(err) => Self::Other(format!("regex error: {}", err)),
        }
    }
}

/// Entity for user data and logic.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct User {
    id: Uuid,
    email: Email,
    password: Password<Argon2PasswordHasher>,
    created: SystemTime,
    modified: SystemTime,
}

impl User {
    /// Initializes a new user builder.
    pub fn builder() -> UserBuilder<HasId, NoEmail, NoPassword, HasCreated, HasModified> {
        let now = SystemTime::now();

        UserBuilder {
            id: HasId(Uuid::default()),
            email: NoEmail,
            password: NoPassword,
            created: HasCreated(now),
            modified: HasModified(now),
        }
    }
}

impl Authenticatable<Error> for User {
    fn confirm_password(&self, password: &str) -> Result<(), Error> {
        Ok(self.password.confirm(password)?)
    }
}

/// Type states for the user builder.
///
/// Builder state indicating that no id has been set.
#[derive(Debug, PartialEq)]
pub struct NoId;

/// Builder state indicating that an id has been set.
#[derive(Debug, PartialEq)]
pub struct HasId(Uuid);

/// Builder state indicating that no email has been set.
#[derive(Debug, PartialEq)]
pub struct NoEmail;

/// Builder state indicating that an enail has been set.
#[derive(Debug, PartialEq)]
pub struct HasEmail(Email);

/// Builder state indicating that no password has been set.
#[derive(Debug, PartialEq)]
pub struct NoPassword;

/// Builder state indicating that a password has been set.
#[derive(Debug, PartialEq)]
pub struct HasPassword(Password<Argon2PasswordHasher>);

/// Builder state indicating that no creation time has been set.
#[derive(Debug, PartialEq)]
pub struct NoCreated;

/// Builder state indicating that a creation time has been set.
#[derive(Debug, PartialEq)]
pub struct HasCreated(SystemTime);

/// Builder state indicating that no modification time has been set.
#[derive(Debug, PartialEq)]
pub struct NoModified;

/// Builder state indicating that a modification time has been set.
#[derive(Debug, PartialEq)]
pub struct HasModified(SystemTime);

/// Builder for User objects.
#[derive(Debug, PartialEq)]
pub struct UserBuilder<I, E, P, C, M> {
    id: I,
    email: E,
    password: P,
    created: C,
    modified: M,
}

/// Builder functions to set builder properties.
impl<I, E, P, C, M> UserBuilder<I, E, P, C, M> {
    /// Sets the id with the provided uuid.
    ///
    /// Returns a validation error is the provided input is invalid.
    pub fn id_from_str(self, id: &'static str) -> Result<UserBuilder<HasId, E, P, C, M>, Error> {
        let Self {
            email,
            password,
            created,
            modified,
            ..
        } = self;
        Ok(UserBuilder {
            id: HasId(id.try_into()?),
            email,
            password,
            created,
            modified,
        })
    }

    /// Sets the id with a random uuid.
    pub fn id(self) -> Result<UserBuilder<HasId, E, P, C, M>, Error> {
        let Self {
            email,
            password,
            created,
            modified,
            ..
        } = self;
        Ok(UserBuilder {
            id: HasId(Uuid::new()),
            email,
            password,
            created,
            modified,
        })
    }

    /// Sets the email with the provided input.
    ///
    /// Returns a validation error is the provided input is invalid.
    pub fn email(self, email: &'static str) -> Result<UserBuilder<I, HasEmail, P, C, M>, Error> {
        let Self {
            id,
            password,
            created,
            modified,
            ..
        } = self;
        Ok(UserBuilder {
            id,
            email: HasEmail(Email::new(email)?),
            password,
            created,
            modified,
        })
    }

    /// Sets the password with the provided input.
    ///
    /// Returns a validation error is the provided input is invalid.
    pub fn password(
        self,
        password: &'static str,
    ) -> Result<UserBuilder<I, E, HasPassword, C, M>, Error> {
        let Self {
            id,
            email,
            created,
            modified,
            ..
        } = self;
        let password = Password::new(password)?;

        Ok(UserBuilder {
            id,
            email,
            password: HasPassword(password),
            created,
            modified,
        })
    }

    /// Sets the creation time with the provided input.
    pub fn created(self, created: SystemTime) -> UserBuilder<I, E, P, HasCreated, M> {
        let Self {
            id,
            email,
            password,
            modified,
            ..
        } = self;

        UserBuilder {
            id,
            email,
            password,
            created: HasCreated(created),
            modified,
        }
    }

    /// Sets the creation time with the provided input.
    pub fn modified(self, modified: SystemTime) -> UserBuilder<I, E, P, C, HasModified> {
        let Self {
            id,
            email,
            password,
            created,
            ..
        } = self;

        UserBuilder {
            id,
            email,
            password,
            created,
            modified: HasModified(modified),
        }
    }
}

impl UserBuilder<HasId, HasEmail, HasPassword, HasCreated, HasModified> {
    /// Builds the a user instance.
    ///
    /// Can only be used when all states have been set.
    pub fn build(self) -> User {
        let Self {
            id,
            email,
            password,
            created,
            modified,
        } = self;
        User {
            id: id.0,
            email: email.0,
            password: password.0,
            created: created.0,
            modified: modified.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_builder_works() {
        let user = User::builder()
            .email("john.doe@example.com")
            .unwrap()
            .password("mmholAhsbC123*")
            .unwrap()
            .build();
        assert!(user.confirm_password("mmholAhsbC123*").is_ok());
    }

    #[test]
    fn user_validation_works() {
        assert_eq!(
            User::builder().email("blabla"),
            Err(Error::Validation(ValidationError::Email))
        );
        assert_eq!(
            User::builder().id_from_str("blabla"),
            Err(Error::Validation(ValidationError::Id))
        );
        assert_eq!(
            User::builder().password("blabla"),
            Err(Error::Validation(ValidationError::Password))
        );
    }
}
