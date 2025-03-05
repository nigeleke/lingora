use std::collections::HashSet;

use fluent4rs::prelude::{Attribute, Message, MessageReference, Term, TermReference, Visitor};

use crate::domain::Identifier;

#[derive(Default)]
pub struct IdentifierVisitor(HashSet<Identifier>);

impl IdentifierVisitor {
    pub fn identifiers(&self) -> HashSet<Identifier> {
        self.0.clone()
    }

    pub fn add_attribute_identifiers(&mut self, parent: &Identifier, attrbutes: &[Attribute]) {
        for attribute in attrbutes {
            self.0
                .insert(Identifier::from_parent_and_attribute(parent, attribute));
        }
    }
}

impl Visitor for IdentifierVisitor {
    fn visit_message(&mut self, message: &Message) {
        let identifier = Identifier::from(message);
        self.add_attribute_identifiers(&identifier, message.attributes());
        self.0.insert(identifier);
    }

    fn visit_message_reference(&mut self, reference: &MessageReference) {
        self.0.insert(Identifier::from(reference));
    }

    fn visit_term(&mut self, term: &Term) {
        let identifier = Identifier::from(term);
        self.add_attribute_identifiers(&identifier, term.attributes());
        self.0.insert(Identifier::from(term));
    }

    fn visit_term_reference(&mut self, reference: &TermReference) {
        self.0.insert(Identifier::from(reference));
    }
}
