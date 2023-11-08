pub type Result<E> = core::result::Result<(), E>;

pub trait Validatable<E> {
    fn validate(&self) -> Result<E>;
}