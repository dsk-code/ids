pub trait Claims: Send + Sync + 'static {
    fn sub(&self) -> String;
}
