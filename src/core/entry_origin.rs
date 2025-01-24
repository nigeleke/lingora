#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EntryOrigin {
    Reference,
    Target,
    TargetFallback,
}
