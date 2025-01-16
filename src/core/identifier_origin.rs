#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum IdentifierOrigin {
    Reference,
    Target,
    TargetFallback,
}
