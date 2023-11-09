use crate::traits::validatable::Validatable;
use uuid::Uuid as CoreUuid;

use super::error::Error;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Uuid(String);

impl TryFrom<&str> for Uuid {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let id = CoreUuid::parse_str(value)?;
        Ok(Self(id.to_string()))
    }
}

impl Validatable<Error> for Uuid {
    fn validate(&self) -> crate::traits::validatable::Result<Error> {
        match CoreUuid::parse_str(&self.0) {
            Err(_) => Err(Error::InvalidID),
            Ok(_) => Ok(())
        }
    }
}

impl Default for Uuid {
    fn default() -> Self {
        Self(CoreUuid::new_v4().to_string())
    }
}

impl Uuid {
    pub fn new() -> Self {
        Self(Default::default())
    }
}