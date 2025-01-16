//use super::language::Language;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PrimaryLanguage(unic_langid::subtags::Language);

impl From<unic_langid::subtags::Language> for PrimaryLanguage {
    fn from(value: unic_langid::subtags::Language) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for PrimaryLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
