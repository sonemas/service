use crate::traits::{PasswordHasher, Authenticatable};

use super::{Email, Password, Error};

pub trait Config {
    type Id: Default + PartialEq;
    type PasswordHasher: PasswordHasher;
    type DateTime: Clone + Copy + Default + Eq + PartialEq;
}

/// Entity for user data and logic.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct User<T: Config> {
    id: T::Id,
    email: Email,
    password: Password<T::PasswordHasher>,
    created: T::DateTime,
    modified: T::DateTime,
}

impl<T: Config> User<T> {
    /// Initializes a new user builder.
    pub fn builder() -> UserBuilder<T, HasId<T>, NoEmail, NoPassword, HasCreated<T>, HasModified<T>> {
        let now = T::DateTime::default();

        UserBuilder {
            id: HasId(T::Id::default()),
            email: NoEmail,
            password: NoPassword,
            created: HasCreated(now),
            modified: HasModified(now),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<T: Config> Authenticatable<Error> for User<T> {
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
pub struct HasId<T: Config>(T::Id);

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
pub struct HasPassword<T: Config>(Password<T::PasswordHasher>);

/// Builder state indicating that no creation time has been set.
#[derive(Debug, PartialEq)]
pub struct NoCreated;

/// Builder state indicating that a creation time has been set.
#[derive(Debug, PartialEq)]
pub struct HasCreated<T: Config>(T::DateTime);

/// Builder state indicating that no modification time has been set.
#[derive(Debug, PartialEq)]
pub struct NoModified;

/// Builder state indicating that a modification time has been set.
#[derive(Debug, PartialEq)]
pub struct HasModified<T: Config>(T::DateTime);

/// Builder for User objects.
#[derive(Debug, PartialEq)]
pub struct UserBuilder<T:Config, I, E, P, C, M> {
    id: I,
    email: E,
    password: P,
    created: C,
    modified: M,
    phantom: std::marker::PhantomData<T>,
}

/// Builder functions to set builder properties.
impl<T: Config, I, E, P, C, M> UserBuilder<T, I, E, P, C, M> {
    /// Sets the id with the provided uuid.
    ///
    /// Returns a validation error is the provided input is invalid.
    pub fn id(self, id: T::Id) -> UserBuilder<T, HasId<T>, E, P, C, M> {
        let Self {
            email,
            password,
            created,
            modified,
            phantom,
            ..
        } = self;
        UserBuilder {
            id: HasId(id),
            email,
            password,
            created,
            modified,
            phantom,
        }
    }

    /// Sets the email with the provided input.
    ///
    /// Returns a validation error is the provided input is invalid.
    pub fn email(self, email: &'static str) -> Result<UserBuilder<T, I, HasEmail, P, C, M>, Error> {
        let Self {
            id,
            password,
            created,
            modified,
            phantom,
            ..
        } = self;
        Ok(UserBuilder {
            id,
            email: HasEmail(Email::new(email)?),
            password,
            created,
            modified,
            phantom,
        })
    }

    /// Sets the password with the provided input.
    ///
    /// Returns a validation error is the provided input is invalid.
    pub fn password(
        self,
        password: &'static str,
    ) -> Result<UserBuilder<T, I, E, HasPassword<T>, C, M>, Error> {
        let Self {
            id,
            email,
            created,
            modified,
            phantom,
            ..
        } = self;
        let password = Password::new(password)?;

        Ok(UserBuilder {
            id,
            email,
            password: HasPassword(password),
            created,
            modified,
            phantom,
        })
    }

    /// Sets the creation time with the provided input.
    pub fn created(self, created: T::DateTime) -> UserBuilder<T, I, E, P, HasCreated<T>, M> {
        let Self {
            id,
            email,
            password,
            modified,
            phantom,
            ..
        } = self;

        UserBuilder {
            id,
            email,
            password,
            created: HasCreated(created),
            modified,
            phantom,
        }
    }

    /// Sets the creation time with the provided input.
    pub fn modified(self, modified: T::DateTime) -> UserBuilder<T, I, E, P, C, HasModified<T>> {
        let Self {
            id,
            email,
            password,
            created,
            phantom,
            ..
        } = self;

        UserBuilder {
            id,
            email,
            password,
            created,
            modified: HasModified(modified),
            phantom,
        }
    }
}

impl<T: Config> UserBuilder<T, HasId<T>, HasEmail, HasPassword<T>, HasCreated<T>, HasModified<T>> {
    /// Builds the a user instance.
    ///
    /// Can only be used when all states have been set.
    pub fn build(self) -> User<T> {
        let Self {
            id,
            email,
            password,
            created,
            modified,
            ..
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
    use crate::{primitives::{Uuid, DateTime}, password_hasher::argon2::Argon2PasswordHasher};
    use super::*;

    struct App;
    impl Config for App {
        type Id = Uuid;
        type PasswordHasher = Argon2PasswordHasher;
        type DateTime = DateTime;
    }

    #[test]
    fn user_builder_works() {
        let user = User::<App>::builder()
            .email("john.doe@example.com")
            .unwrap()
            .password("mmholAhsbC123*")
            .unwrap()
            .build();
        assert!(user.confirm_password("mmholAhsbC123*").is_ok());
    }
}
