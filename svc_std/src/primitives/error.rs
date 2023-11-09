use fancy_regex::Error as RegexError;

use crate::traits::password_hasher::password_hasher;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    InvalidEmailAddress,
    InvalidID,
    InvalidPassword,
    PasswordHashingError(password_hasher::Error),
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

impl From<password_hasher::Error> for Error{
    fn from(value: password_hasher::Error) -> Self {
        match value {
            password_hasher::Error::InvalidPassword => Self::InvalidPassword,
            _ => Self::PasswordHashingError(value)
        }
    }
}

impl From<uuid::Error> for Error {
    fn from(_: uuid::Error) -> Self {
        Self::InvalidID
    }
}