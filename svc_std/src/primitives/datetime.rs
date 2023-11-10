use std::time::SystemTime;

/// A datetime field based on SystemTime.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct DateTime(SystemTime);

impl std::default::Default for DateTime {
    fn default() -> Self {
        Self(SystemTime::now())
    }
}

impl std::ops::Deref for DateTime {
    type Target = SystemTime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::convert::AsRef<SystemTime> for DateTime {
    fn as_ref(&self) -> &SystemTime {
        &self.0
    }
}

impl DateTime {
    pub fn now() -> Self {
        Self::default()
    }
}
