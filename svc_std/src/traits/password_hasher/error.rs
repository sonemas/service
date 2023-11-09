use argon2::password_hash::Error as ArgonError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    HashingError(String),
    InvalidPassword,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HashingError(msg) => write!(f, "hashing failed: {msg}"),
            Self::InvalidPassword => write!(f, "invalid password"),
        }
    }
}
impl std::error::Error for Error {}

impl From<ArgonError> for Error {
    fn from(value: ArgonError) -> Self {
        match value {
            ArgonError::Password => Self::InvalidPassword,
            _ => Self::HashingError(value.to_string())
        }
    }
}