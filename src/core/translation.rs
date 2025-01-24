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
}

// TODO: Not needed
impl std::fmt::Display for Translation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{:?}", self.entry, self.preceding_comments)
    }
}
