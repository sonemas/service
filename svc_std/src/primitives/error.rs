use fancy_regex::Error as RegexError;

use crate::traits::password_hasher;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ValidationError {
    Id,
    Email,
    Password,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for ValidationError {}

impl From<uuid::Error> for ValidationError {
    fn from(_: uuid::Error) -> Self {
        Self::Id
    }
}

/// Primitives' error enum.
///
/// Errors could be a validation or technical errors.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    /// Indicates a validation error.
    Validation(ValidationError),

    /// Indicates that the validity of a password couldn't be confirmed.
    InvalidPassword,

    /// Technical error indicating that a password hasher wasn't able to hash a password.
    PasswordHashingError(password_hasher::Error),

    /// Technical error indicating a problem with a regular expression.
    /// In most cases this error indicates that a regular expression couldn't be compiled.
    RegexError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for Error {}

impl From<RegexError> for Error {
    fn from(value: RegexError) -> Self {
        Self::RegexError(value.to_string())
    }
}

impl From<ValidationError> for Error {
    fn from(value: ValidationError) -> Self {
        Self::Validation(value)
    }
}

impl From<password_hasher::Error> for Error {
    fn from(value: password_hasher::Error) -> Self {
        match value {
            password_hasher::Error::InvalidPassword => Self::InvalidPassword,
            _ => Self::PasswordHashingError(value),
        }
    }
}
