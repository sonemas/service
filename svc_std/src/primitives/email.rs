use super::error::{Error, ValidationError};
use crate::traits::validatable::Validatable;
use fancy_regex::Regex;

/// A validatable email field.
///
/// ```rust
/// # use crate::svc_std::{traits::Validatable, primitives::{Email, Error, ValidationError}};
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let john_email = Email::new("john.doe@example.com")?;
///     assert!(john_email.validate().is_ok());
///     assert_eq!(Email::new("not an email"), Err(Error::Validation(ValidationError::Email)));
///
///     let jane_email: Email = "jane.doe@example.com".try_into()?;
/// #    Ok(())
/// # }
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Email(String);

impl Validatable<Error> for Email {
    fn validate(&self) -> crate::traits::validatable::Result<Error> {
        let email_regex = Regex::new(
            r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
        )?;
        if !email_regex.is_match(&self.0).unwrap_or(false) {
            return Err(ValidationError::Email.into());
        }
        Ok(())
    }
}

impl TryFrom<&str> for Email {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Email::new(value)
    }
}

impl std::fmt::Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Email {
    /// Initializes a new email instance.
    ///
    /// Returns a validation error if validation of the provided value fails.
    pub fn new(value: &str) -> Result<Self, Error> {
        let v = Self(value.to_string());
        v.validate()?;
        Ok(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn email_validation_works() {
        assert!(Email::new("john.doe@example.com").is_ok());
        assert_eq!(
            Email::new("a"),
            Err(Error::Validation(ValidationError::Email))
        );
        assert_eq!(
            Email::new("a@"),
            Err(Error::Validation(ValidationError::Email))
        );
        assert_eq!(
            Email::new("example.com"),
            Err(Error::Validation(ValidationError::Email))
        );
        assert_eq!(
            Email::new("a@.com"),
            Err(Error::Validation(ValidationError::Email))
        );
    }
}
