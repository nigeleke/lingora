use crate::domain::Identifier;

use fluent4rs::prelude::{Message, MessageReference, Term, TermReference, Visitor};

use std::collections::HashSet;

#[derive(Default)]
pub struct IdentifierVisitor(HashSet<Identifier>);

impl IdentifierVisitor {
    pub fn identifiers(&self) -> HashSet<Identifier> {
        self.0.clone()
    }
}

impl Visitor for IdentifierVisitor {
    fn visit_message(&mut self, message: &Message) {
        self.0.insert(Identifier::from(message));
    }

    fn visit_message_reference(&mut self, reference: &MessageReference) {
        self.0.insert(Identifier::from(reference));
    }

    fn visit_term(&mut self, term: &Term) {
        self.0.insert(Identifier::from(term));
    }

    fn visit_term_reference(&mut self, reference: &TermReference) {
        self.0.insert(Identifier::from(reference));
    }
}
