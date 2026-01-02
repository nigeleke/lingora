use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use fluent4rs::{ast::*, prelude::*};

use crate::domain::Identifier;

/// Visits all AST nodes that use Identifiers and (special case) variant key names,
/// persisting the identifier, with its path.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct IdentifierVisitor {
    current_entry: Option<Rc<Entry>>,
    contexts: Vec<Vec<String>>,
    identifiers: HashMap<Identifier, HashSet<Rc<Entry>>>,
}

impl IdentifierVisitor {
    fn current_context(&self) -> Vec<String> {
        self.contexts.last().map_or(Vec::new(), |v| v.clone())
    }

    fn append_current_context(&mut self, context: String) {
        let current_context = self.contexts.last_mut().unwrap();
        current_context.push(context);
    }

    fn insert(&mut self, stem: String) {
        let context = self.current_context();
        let context = context.iter().map(|i| i.as_str()).collect::<Vec<_>>();
        let identifier = Identifier::new(&context, stem.as_str());
        let entry = self.current_entry.as_ref().unwrap();
        self.identifiers
            .entry(identifier)
            .or_default()
            .insert(entry.clone());
        self.append_current_context(stem);
    }

    pub fn identifiers(&self) -> HashSet<Identifier> {
        HashSet::from_iter(self.identifiers.keys().cloned())
    }

    pub fn usages(&self, identifier: &Identifier) -> HashSet<Entry> {
        let usages = self
            .identifiers
            .get(identifier)
            .cloned()
            .unwrap_or_default();
        HashSet::from_iter(usages.into_iter().map(|re| (*re).clone()))
    }
}

impl Visitor for IdentifierVisitor {
    fn enter(&mut self) {
        let current_context = self.current_context();
        self.contexts.push(current_context.clone());
    }

    fn exit(&mut self) {
        self.contexts.pop();
    }

    fn visit_entry(&mut self, entry: &Entry) {
        self.current_entry = Some(Rc::new(entry.clone()));
    }

    fn visit_message(&mut self, message: &Message) {
        self.insert(message.identifier_name());
    }

    fn visit_term(&mut self, term: &Term) {
        self.insert(term.identifier_name());
    }

    fn visit_attribute(&mut self, attribute: &Attribute) {
        self.insert(attribute.identifier_name());
    }

    fn visit_variant(&mut self, variant: &Variant) {
        self.insert(variant.variant_key_name());
    }

    fn visit_default_variant(&mut self, variant: &DefaultVariant) {
        self.insert(variant.variant_key_name());
    }

    fn visit_message_reference(&mut self, reference: &MessageReference) {
        self.insert(reference.identifier_name());
    }

    fn visit_term_reference(&mut self, reference: &TermReference) {
        self.insert(reference.identifier_name());
    }

    fn visit_attribute_accessor(&mut self, accessor: &AttributeAccessor) {
        self.insert(accessor.identifier_name());
    }

    fn visit_variable_reference(&mut self, reference: &VariableReference) {
        self.insert(reference.identifier_name());
    }

    fn visit_function_reference(&mut self, reference: &FunctionReference) {
        self.insert(reference.identifier_name());
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::domain::Identifier;

    #[test]
    fn basic_identifiers_will_be_persisted() {
        let content = r#"
message = A message
-term = A term
"#;
        let ast = Parser::parse(content).unwrap();
        let mut visitor = IdentifierVisitor::default();
        Walker::walk(&ast, &mut visitor);

        let identifiers = visitor.identifiers();
        let expected_identifiers = [
            Identifier::new(&[], "message"),
            Identifier::new(&[], "-term"),
        ];

        assert_eq!(identifiers.len(), expected_identifiers.len());
        for identifier in expected_identifiers {
            assert!(identifiers.contains(&identifier))
        }
    }

    #[test]
    fn attribute_identifiers_will_include_parents() {
        let content = r#"
message = A message
    .placeholder = A message placeholder
    .aria-label = A message aria label
-term = A term
    .placeholder = A term placeholder
    .aria-label = A term aria label
"#;
        let ast = Parser::parse(content).unwrap();
        let mut visitor = IdentifierVisitor::default();
        Walker::walk(&ast, &mut visitor);

        let identifiers = visitor.identifiers();

        let expected_identifiers = [
            Identifier::new(&[], "message"),
            Identifier::new(&["message"], ".placeholder"),
            Identifier::new(&["message"], ".aria-label"),
            Identifier::new(&[], "-term"),
            Identifier::new(&["-term"], ".placeholder"),
            Identifier::new(&["-term"], ".aria-label"),
        ];

        assert_eq!(identifiers.len(), expected_identifiers.len());
        for identifier in expected_identifiers {
            assert!(identifiers.contains(&identifier))
        }
    }

    #[test]
    fn attribute_identifiers_will_include_grandparents() {
        let content = r#"
message = A message { $name }
    .placeholder = A message placeholder { $name }
    .aria-label = A message aria label { $name }
-term = A term { $name }
    .placeholder = A term placeholder { $name }
    .aria-label = A term aria label { $name }
"#;
        let ast = Parser::parse(content).unwrap();
        let mut visitor = IdentifierVisitor::default();
        Walker::walk(&ast, &mut visitor);

        let identifiers = visitor.identifiers();

        let expected_identifiers = [
            Identifier::new(&[], "message"),
            Identifier::new(&["message"], "$name"),
            Identifier::new(&["message"], ".placeholder"),
            Identifier::new(&["message", ".placeholder"], "$name"),
            Identifier::new(&["message"], ".aria-label"),
            Identifier::new(&["message", ".aria-label"], "$name"),
            Identifier::new(&[], "-term"),
            Identifier::new(&["-term"], "$name"),
            Identifier::new(&["-term"], ".placeholder"),
            Identifier::new(&["-term", ".placeholder"], "$name"),
            Identifier::new(&["-term"], ".aria-label"),
            Identifier::new(&["-term", ".aria-label"], "$name"),
        ];

        assert_eq!(identifiers.len(), expected_identifiers.len());
        for identifier in expected_identifiers {
            assert!(identifiers.contains(&identifier))
        }
    }
}
