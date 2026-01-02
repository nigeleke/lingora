#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct FocusScope(usize);

impl FocusScope {
    pub const ROOT: Self = FocusScope(0);
}

impl FocusScope {
    pub const fn new(scope: usize) -> Self {
        Self(scope)
    }
}

impl Default for FocusScope {
    fn default() -> Self {
        Self(Default::default())
    }
}
