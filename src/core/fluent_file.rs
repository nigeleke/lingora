use super::error::Error;
use super::identifier::Identifier;
use super::locale::Locale;

use fluent::FluentResource;
use fluent_syntax::ast::Entry;

use std::collections::HashSet;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum FluentFile {
    Editable(Locale, PathBuf),
    Locked(Error),
}

impl FluentFile {
    pub fn try_identifiers(&self) -> Result<HashSet<Identifier>, Error> {
        let insert_message_id = |mut identifiers: HashSet<Identifier>, entry: &Entry<_>| {
            match &entry {
                Entry::Message(message) => identifiers.insert(Identifier::from(message.id.name)),
                _ => unimplemented!(),
            };
            identifiers
        };

        let identifiers = match self {
            FluentFile::Editable(_, path) => {
                let content = std::fs::read_to_string(path)
                    .map_err(|e| Error::FluentFileAccessFailed(e.to_string()))?;
                let resource = FluentResource::try_new(content)
                    .map_err(|e| Error::FluentFileAccessFailed(format!("{:#?}", e)))?;
                resource.entries().fold(HashSet::new(), insert_message_id)
            }
            FluentFile::Locked(_) => HashSet::new(),
        };

        Ok(identifiers
            .into_iter()
            .map(Identifier::from)
            .collect::<HashSet<_>>())
    }
}

impl From<&PathBuf> for FluentFile {
    fn from(value: &PathBuf) -> Self {
        let validated = Locale::try_from(value)
            .map(|l| FluentFile::Editable(l, value.to_owned()))
            .map_err(FluentFile::Locked);
        match validated {
            Ok(vl) => vl,
            Err(vl) => vl,
        }
    }
}
