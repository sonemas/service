use crate::traits::password_hasher::PasswordHasher;
use fancy_regex::Regex;
use std::marker::PhantomData;

use super::{error::Error, ValidationError};

/// A password field with built-in validation and hashing.
///
/// Validation is done when initializing a new instance with new. Only stores the password hash.
/// A password is considered valid when it has:
/// - a length between 8 and 20 characters
/// - a combination of lowercase, uppercase, digits and symbols
/// ```rust
/// # use crate::svc_std::{traits::Validatable, password_hasher::argon2::Argon2PasswordHasher, primitives::{Password, Error, ValidationError}};
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let password: Password<Argon2PasswordHasher> = Password::new("mmholAhsbC123*")?;
///     assert!(password.confirm("mmholAhsbC123*").is_ok());
///     assert_eq!(password.confirm("blabla"), Err(Error::InvalidPassword));
///
///     let password: Password<Argon2PasswordHasher> = "mmholAhsbC123*".try_into()?;
///     assert_eq!(Password::<Argon2PasswordHasher>::try_from("aaa"), Err(Error::Validation(ValidationError::Password)));
/// #    Ok(())
/// # }
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Password<T: PasswordHasher>(String, std::marker::PhantomData<T>);

impl<T: PasswordHasher> TryFrom<&'static str> for Password<T> {
    type Error = Error;

    fn try_from(value: &'static str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl<T: PasswordHasher> ToString for Password<T> {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl<T: PasswordHasher> Password<T> {
    /// Initializes a new password instance.
    ///
    /// Returns a validation error if validation of the provided value fails.
    pub fn new(value: &'static str) -> Result<Self, Error> {
        Self::validate_value(value)?;
        let password_hash = T::hash(value)?;
        Ok(Self(password_hash.to_string(), PhantomData))
    }

    fn validate_value(value: &str) -> Result<(), Error> {
        let re = Regex::new(
            r"^(?=.*\d)(?=.*[a-z])(?=.*[A-Z])(?=.*[#$%/()=¿?*+-])(?=(?:([\w\d])\1?(?!\1\1)))(?!(?=.*(palabra1|palabra2|palabraN))).{8,20}$",
        )?;
        if !re.is_match(value).unwrap_or(false) {
            return Err(ValidationError::Password.into());
        }
        Ok(())
    }

    /// Confirms whehter the provided password matches the stored password hash.
    ///
    /// Returns `Error::InvalidPassword` if the provided password is invalid.
    pub fn confirm(&self, password: &str) -> Result<(), Error> {
        T::confirm_password(password, &self.0.clone())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{password_hasher::argon2::Argon2PasswordHasher, primitives::ValidationError};

    use super::*;

    #[test]
    fn password_validation_works() {
        // Ensure that a valid password passes.
        assert!(Password::<Argon2PasswordHasher>::new("mmholAhsbC123*").is_ok());

        // Ensure that a password shorter than 8 characters fails validation.
        assert_eq!(
            Password::<Argon2PasswordHasher>::new("aQ3*"),
            Err(Error::Validation(ValidationError::Password))
        );

        // Ensure that a password longer than 20 characters fails validation.
        assert_eq!(
            Password::<Argon2PasswordHasher>::new("mmholAhsbC123*artfgrr"),
            Err(Error::Validation(ValidationError::Password))
        );

        // Ensure that repetitive passwords fail validation.
        assert_eq!(
            Password::<Argon2PasswordHasher>::new("aaaaaaaaaaaaaaaaaaa"),
            Err(Error::Validation(ValidationError::Password))
        );
        assert_eq!(
            Password::<Argon2PasswordHasher>::new("AAAAAAAAAAAAAAAAAAA"),
            Err(Error::Validation(ValidationError::Password))
        );
        assert_eq!(
            Password::<Argon2PasswordHasher>::new("1111111111111111111"),
            Err(Error::Validation(ValidationError::Password))
        );
        assert_eq!(
            Password::<Argon2PasswordHasher>::new("*******************"),
            Err(Error::Validation(ValidationError::Password))
        );

        // Ensure that a password without at least one symbol fails validation.
        assert_eq!(
            Password::<Argon2PasswordHasher>::new("mmholAhsbC123"),
            Err(Error::Validation(ValidationError::Password))
        );

        // Ensure that a password without at least one capital letter fails validation.
        assert_eq!(
            Password::<Argon2PasswordHasher>::new("mmholahsbc123*"),
            Err(Error::Validation(ValidationError::Password))
        );

        // Ensure that a password without at least one lowercase letter fails validation.
        assert_eq!(
            Password::<Argon2PasswordHasher>::new("MMHOLAHSBC123*"),
            Err(Error::Validation(ValidationError::Password))
        );

        // Ensure that a password without at least one digit fails validation.
        assert_eq!(
            Password::<Argon2PasswordHasher>::new("mmholAhsbCaaa*"),
            Err(Error::Validation(ValidationError::Password))
        );
    }

    #[test]
    fn password_confirmation_works() {
        let password = Password::<Argon2PasswordHasher>::new("mmholAhsbC123*").unwrap();
        assert!(password.confirm("mmholAhsbC123*").is_ok());
        assert_eq!(password.confirm("blabla"), Err(Error::InvalidPassword));
    }
}
