/// Type alias for validation results.
///
/// Requires only a generic type for errors.
pub type Result<E> = core::result::Result<(), E>;

/// A trait for validatable objects or fields.
///
/// The example below is simplified, for a production ready example
/// observe the code in `svc_std::primitives::email`.
///
/// ```rust
/// # use crate::svc_std::{traits::validatable::{self, Validatable}};
/// pub struct Email(String);
/// impl Validatable<String> for Email {
///     fn validate(&self) -> validatable::Result<String> {
///         if !self.0.contains('@') || !self.0.contains('.') { return Err("error: invalid email address".to_string())}
///         Ok(())
///     }
/// }
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     assert!(Email("john@example.com".to_string()).validate().is_ok());
///     assert_eq!(Email("invalid email".to_string()).validate(), Err("error: invalid email address".to_string()));
///     Ok(())
/// }
/// ```
pub trait Validatable<E> {
    fn validate(&self) -> Result<E>;
}
