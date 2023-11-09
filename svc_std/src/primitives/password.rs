use std::marker::PhantomData;

use crate::traits::{Validatable, password_hasher::{PasswordHasher, argon2::Argon2PasswordHasher}};
use fancy_regex::Regex;

use super::error::Error;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Password<T: PasswordHasher>(String, std::marker::PhantomData<T>);

impl<T: PasswordHasher> TryFrom<&'static str> for Password<T> {
    type Error = Error;

    fn try_from(value: &'static str) -> Result<Self, Self::Error> {
        Ok(Self::new(value)?)
    }
}

impl<T: PasswordHasher> ToString for Password<T> {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl<T: PasswordHasher> Validatable<Error> for Password<T> {
    fn validate(&self) -> crate::traits::validatable::Result<Error> {
        let re = Regex::new(r"^(?=.*\d)(?=.*[a-z])(?=.*[A-Z])(?=.*[#$%/()=¿?*+-])(?=(?:([\w\d])\1?(?!\1\1)))(?!(?=.*(palabra1|palabra2|palabraN))).{8,20}$")?;
        if !re.is_match(&self.0).unwrap_or(false) { return Err(Error::InvalidPassword) }
        Ok(())
    }
}

impl<T: PasswordHasher> Password<T> {
    pub fn new(value: &'static str) -> Result<Self, Error> {
        // TODO: This is ugly... Refactor to something nicer.
        let v = Self(value.to_string(), PhantomData);
        v.validate()?;
        let password_hash = T::hash(value)?;
        Ok(Self(password_hash.to_string(), PhantomData))
    }

    pub fn confirm(&self, password: &str) -> Result<(), Error> {
        T::confirm_password(password, &self.0.clone())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn password_validation_works() {
        assert!(Password::<Argon2PasswordHasher>::new("mmholAhsbC123*").is_ok());
        assert_eq!(Password::<Argon2PasswordHasher>::new("aQ3*"), Err(Error::InvalidPassword));
        assert_eq!(Password::<Argon2PasswordHasher>::new("aaaaaaaaaaaaaaaaaaa"), Err(Error::InvalidPassword));
        assert_eq!(Password::<Argon2PasswordHasher>::new("AAAAAAAAAAAAAAAAAAA"), Err(Error::InvalidPassword));
        assert_eq!(Password::<Argon2PasswordHasher>::new("1111111111111111111"), Err(Error::InvalidPassword));
        assert_eq!(Password::<Argon2PasswordHasher>::new("*******************"), Err(Error::InvalidPassword));
        assert_eq!(Password::<Argon2PasswordHasher>::new("mmholAhsbC123"), Err(Error::InvalidPassword));
        assert_eq!(Password::<Argon2PasswordHasher>::new("mmholAhsbC123*artfgrr"), Err(Error::InvalidPassword));
    }

    #[test]
    fn password_works() {
        let password = Password::<Argon2PasswordHasher>::new("mmholAhsbC123*").unwrap();
        assert!(password.confirm("mmholAhsbC123*").is_ok());
        assert_eq!(password.confirm("blabla"), Err(Error::InvalidPassword));
    }
}