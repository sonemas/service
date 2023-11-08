use regex::Regex;

use crate::traits::validatable::Validatable;

use super::validation_error::Error;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Email(String);

impl Validatable<Error> for Email {
    fn validate(&self) -> crate::traits::validatable::Result<Error> {
        let email_regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})")?;
        if !email_regex.is_match(&self.0) { return Err(Error::InvalidEmailAddress) }
        Ok(())
    }
}

impl Email {
    pub fn new(email: &'static str) -> Result<Self, Error> {
        let v = Self(email.to_string());
        v.validate()?;
        Ok(v)
    }
}