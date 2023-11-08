use regex::Error as RegexError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    InvalidEmailAddress,
    InvalidID,
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

impl From<uuid::Error> for Error {
    fn from(_: uuid::Error) -> Self {
        Self::InvalidID
    }
}