use super::language::Language;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PrimaryLanguage(String);

impl From<&Language> for PrimaryLanguage {
    fn from(value: &Language) -> Self {
        Self(value.primary_language().into())
    }
}
