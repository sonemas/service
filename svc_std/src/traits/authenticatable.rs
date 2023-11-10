/// Type alias for authentication results.
///
/// Requires only a generic type for errors.
pub type Result<E> = core::result::Result<(), E>;

/// A trait for password authenticatable objects.
///
/// This trait is particularly useful in combination with hashed passwords,
/// for example as `svc_std::primitives::password::Password` does.
///
/// ```rust
/// # use crate::svc_std::traits::{authenticatable, Authenticatable};
/// struct User {
///     username: &'static str,
///     password: &'static str,
/// }
/// impl Authenticatable<&str> for User {
///     fn confirm_password(&self, password: &str) -> authenticatable::Result<&'static str> {
///         // Reminder: Working with litereal passwords is bad!
///         // Use password hashing in production environments.
///         if password != self.password { return Err("invalid password") }
///         Ok(())
///     }
/// }
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let user = User{
///         username: "test_user",
///         password: "testtest",
///     };
///
///     assert!(user.confirm_password("testtest").is_ok());
///     assert_eq!(user.confirm_password("blabla"), Err("invalid password"));
///
///     Ok(())
/// }
/// ```
pub trait Authenticatable<E> {
    fn confirm_password(&self, password: &str) -> Result<E>;
}
