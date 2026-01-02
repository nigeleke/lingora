use super::{FocusScope, key::FocusKey};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FocusId {
    pub(super) key: FocusKey,
    pub(super) scope: FocusScope,
}
