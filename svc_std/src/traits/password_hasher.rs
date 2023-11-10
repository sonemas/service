/// Type for communicating password hashing errors.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    /// Indicates an error from the hashing algorithm.
    HashingError(String),

    /// Indicates that password validation failed.
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

/// A trait that password hashers should implement.
pub trait PasswordHasher {
    /// Returns the hash for the provided input or `Error::HashingError` if
    /// the hashing algorithm failed.
    fn hash(input: &str) -> Result<String, Error>;

    /// Confirms whether the provided password matches for the provided hash.
    ///
    /// Returns `Error::InvalidPassword` if password validation fails or
    /// `Error::HashingError` in case of hasher errors.
    fn confirm_password(password: &str, hash: &str) -> Result<(), Error>;
}
