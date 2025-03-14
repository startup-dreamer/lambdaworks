#[derive(Debug)]
pub enum FieldError {
    DivisionByZero,
    /// Returns order of the calculated root of unity
    RootOfUnityError(u64),
}
