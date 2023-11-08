pub trait Authenticatable<E> {
    fn confirm_password(&self, password: &str) -> Result<(), E>;
}