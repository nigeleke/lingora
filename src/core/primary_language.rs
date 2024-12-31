use super::language::Language;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PrimaryLanguage(String);

impl From<&Language> for PrimaryLanguage {
    fn from(value: &Language) -> Self {
        Self(value.primary_language().into())
    }
}

impl std::fmt::Display for PrimaryLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
