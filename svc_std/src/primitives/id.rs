use crate::traits::validatable::Validatable;
use uuid::Uuid as CoreUuid;

use super::{error::Error, ValidationError};

/// A validatable uuid field.
///
/// ```rust
/// # use crate::svc_std::{traits::Validatable, primitives::{Uuid, Error, ValidationError}};
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let uuid = Uuid::new();
///     assert!(uuid.validate().is_ok());
///     println!("Uuid: {uuid}");
///
///     let uuid: Uuid = "07a25b85-f1bb-4143-8e2e-5d8b4fb32f26".try_into()?;
///     assert_eq!(Uuid::try_from("234"), Err(Error::Validation(ValidationError::Id)));
/// #    Ok(())
/// # }
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Uuid(String);

impl Validatable<Error> for Uuid {
    fn validate(&self) -> crate::traits::validatable::Result<Error> {
        match CoreUuid::parse_str(&self.0) {
            Err(_) => Err(ValidationError::Id.into()),
            Ok(_) => Ok(()),
        }
    }
}

impl TryFrom<&str> for Uuid {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let id = CoreUuid::parse_str(value)
            .map_err(|_| ValidationError::Id)?
            .to_string();
        Ok(Self(id))
    }
}

impl std::fmt::Display for Uuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for Uuid {
    fn default() -> Self {
        Self(CoreUuid::new_v4().to_string())
    }
}

impl Uuid {
    /// Initializes a new uuid instance with a random v4 uuid.
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn email_validation_works() {
        assert!(Uuid::new().validate().is_ok());
        assert!(Uuid::try_from("ebf8a4f3-b481-474c-ae29-c71e975e1055").is_ok());
        assert_eq!(
            Uuid::try_from("123"),
            Err(Error::Validation(ValidationError::Id))
        );
    }
}
