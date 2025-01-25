use fluent4rs::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Translation {
    /// A Message or a Term...
    entry: Entry,
    /// Comment lines that immediately preceed the Message or Term, but which
    /// aren't directly part of the Message or Term description.
    preceding_comments: Vec<CommentLine>,
}

impl Translation {
    pub fn new(entry: &Entry, preceding_comments: &[CommentLine]) -> Self {
        Self {
            entry: entry.clone(),
            preceding_comments: Vec::from(preceding_comments),
        }
    }

    pub fn entry(&self) -> &Entry {
        &self.entry
    }

    pub fn pattern(&self) -> Option<&Pattern> {
        match &self.entry {
            Entry::Message(message) => message.pattern(),
            Entry::Term(term) => Some(term.pattern()),
            Entry::CommentLine(_) => unreachable!(),
        }
    }

    pub fn attributes(&self) -> &[Attribute] {
        match &self.entry {
            Entry::Message(message) => message.attributes(),
            Entry::Term(term) => term.attributes(),
            Entry::CommentLine(_) => unreachable!(),
        }
    }
}
