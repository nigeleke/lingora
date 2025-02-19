use std::collections::HashSet;

use fluent4rs::prelude::{Entry, Message, MessageReference, Term, TermReference, Visitor};

use crate::domain::Identifier;

pub struct UsageVisitor<'a> {
    identifier: &'a Identifier,
    most_recent_entry: Option<Entry>,
    entries: HashSet<Entry>,
}

impl<'a> UsageVisitor<'a> {
    pub fn new(identifier: &'a Identifier) -> Self {
        Self {
            identifier,
            most_recent_entry: None,
            entries: HashSet::new(),
        }
    }

    fn add_entry(&mut self, identifier: &Identifier) {
        if self.identifier == identifier {
            self.entries
                .insert(self.most_recent_entry.as_ref().unwrap().clone());
        }
    }

    pub fn entries(&self) -> HashSet<Entry> {
        self.entries.clone()
    }
}

impl<'a> Visitor for UsageVisitor<'a> {
    fn visit_entry(&mut self, entry: &Entry) {
        self.most_recent_entry = Some(entry.clone())
    }

    fn visit_message(&mut self, message: &Message) {
        self.add_entry(&Identifier::from(message));
    }

    fn visit_message_reference(&mut self, reference: &MessageReference) {
        self.add_entry(&Identifier::from(reference));
    }

    fn visit_term(&mut self, term: &Term) {
        self.add_entry(&Identifier::from(term));
    }

    fn visit_term_reference(&mut self, reference: &TermReference) {
        self.add_entry(&Identifier::from(reference));
    }
}
