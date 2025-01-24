use super::error::Error;
use super::identifier::Identifier;
use super::locale::Locale;
use super::translation::Translation;

use fluent4rs::prelude::{Entry, Parser};

use std::collections::HashSet;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum FluentFile {
    Editable(Locale, PathBuf),
    Locked(Error),
}

impl FluentFile {
    pub fn try_identifiers(&self) -> Result<HashSet<Identifier>, Error> {
        let insert_message_id = |mut identifiers: HashSet<Identifier>, entry: Entry| {
            let name = match entry {
                Entry::Message(message) => Some(message.identifier_name()),
                Entry::Term(term) => Some(term.identifier_name()),
                _ => None,
            };
            if let Some(name) = name {
                identifiers.insert(Identifier::from(name.as_str()));
            }
            identifiers
        };

        let identifiers = match self {
            FluentFile::Editable(_, path) => {
                let content = std::fs::read_to_string(path)
                    .map_err(|e| Error::FluentFileAccessFailed(e.to_string()))?;
                let resource = Parser::parse(&content)
                    .map_err(|e| Error::FluentFileAccessFailed(format!("{:#?}", e)))?;
                resource
                    .entries()
                    .into_iter()
                    .fold(HashSet::new(), |acc, e| insert_message_id(acc, e.clone()))
            }
            FluentFile::Locked(_) => HashSet::new(),
        };

        Ok(identifiers)
    }

    pub fn translation(&self, identifier: &Identifier) -> Option<Translation> {
        let identifier_name = identifier.to_string();
        match self {
            FluentFile::Editable(_, path) => {
                let Ok(content) = std::fs::read_to_string(path) else {
                    return None;
                };
                let Ok(resource) = Parser::parse(&content) else {
                    return None;
                };
                let mut preceeding = Vec::new();
                let mut result = None;
                for entry in resource.entries() {
                    match entry {
                        Entry::Message(message) => {
                            if message.identifier_name() == identifier_name {
                                result = Some(Translation::new(entry, &preceeding));
                                break;
                            } else {
                                preceeding.clear();
                            }
                        }
                        Entry::Term(term) => {
                            if term.identifier_name() == identifier_name {
                                result = Some(Translation::new(entry, &preceeding));
                                break;
                            } else {
                                preceeding.clear();
                            }
                        }
                        Entry::CommentLine(comment) => preceeding.push(comment.clone()),
                    }
                }
                result
            }
            FluentFile::Locked(_) => None,
        }
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
