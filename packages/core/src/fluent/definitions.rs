use std::{
    collections::{HashMap, HashSet},
    env::var,
    rc::Rc,
};

use fluent4rs::{ast::*, prelude::*};

use crate::fluent::QualifiedIdentifier;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum PathSegment {
    Message(String),
    Term(String),
    Attribute(String),
    Variant(String),
    DefaultVariant(String),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Path(Vec<PathSegment>);

impl Path {
    pub fn normalized(&self) -> Self {
        let path = self
            .0
            .clone()
            .into_iter()
            .map(|s| match s {
                PathSegment::DefaultVariant(name) => PathSegment::Variant(name),
                other => other,
            })
            .collect::<Vec<_>>();

        Self(path)
    }
}

impl std::cmp::PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.to_string().partial_cmp(&other.to_string())
    }
}

impl std::cmp::Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.to_string().cmp(&other.to_string())
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let path = self
            .0
            .clone()
            .into_iter()
            .map(|s| match s {
                PathSegment::Message(name)
                | PathSegment::Term(name)
                | PathSegment::Attribute(name)
                | PathSegment::Variant(name)
                | PathSegment::DefaultVariant(name) => name,
            })
            .collect::<Vec<_>>()
            .join(" / ");

        path.fmt(f)
    }
}

#[derive(Debug, Default)]
struct PathStack {
    stack: Vec<Vec<PathSegment>>,
}

impl PathStack {
    fn enter(&mut self) {
        let next = self.stack.last().cloned().unwrap_or_default();
        self.stack.push(next);
    }

    fn exit(&mut self) {
        self.stack.pop();
    }

    fn current(&self) -> &[PathSegment] {
        self.stack.last().map(Vec::as_slice).unwrap_or(&[])
    }

    fn push(&mut self, segment: PathSegment) {
        if let Some(top) = self.stack.last_mut() {
            top.push(segment);
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Signature {
    pub(crate) has_value: bool,
    pub(crate) paths: HashSet<Path>,
}

type EntriesById = HashMap<Path, Vec<Rc<Entry>>>;
type Signatures = HashMap<Path, Signature>;

#[derive(Debug, Default)]
pub struct Definitions {
    current_entry: Option<Rc<Entry>>,
    path_stack: PathStack,
    entry_by_id: EntriesById,
    signatures: Signatures,
}

impl Definitions {
    pub fn duplicate_identifiers(&self) -> Vec<QualifiedIdentifier> {
        let normalize_entries = |entries: &EntriesById| {
            let mut normalized: EntriesById = HashMap::new();

            for (path, entry_vec) in entries {
                normalized
                    .entry(path.normalized())
                    .or_default()
                    .extend(entry_vec.iter().cloned());
            }

            normalized
        };

        normalize_entries(&self.entry_by_id)
            .iter()
            .filter_map(|(k, v)| (v.len() > 1).then_some(QualifiedIdentifier::from(k)))
            .collect::<Vec<_>>()
    }

    pub fn entry_identifiers(&self) -> impl Iterator<Item = QualifiedIdentifier> {
        self.signatures.keys().map(|k| QualifiedIdentifier::from(k))
    }

    fn record_identifier(&mut self, segment: &PathSegment) {
        self.path_stack.push(segment.clone());
        let path = Path(Vec::from(self.path_stack.current()));

        if let Some(entry) = &self.current_entry {
            self.entry_by_id
                .entry(path.clone())
                .or_default()
                .push(entry.clone());

            let root = Path(Vec::from(&self.path_stack.current()[..1]));
            let signature = self.signatures.entry(root).or_default();
            signature.paths.insert(path);
        };
    }

    fn update_signature(&mut self, segment: &PathSegment, has_value: bool) {
        let path = Path(Vec::from([segment.clone()]));
        let signature = self.signatures.entry(path).or_default();
        signature.has_value = has_value;
    }

    pub fn signature(&self, path: &Path) -> Option<&Signature> {
        self.signatures.get(path)
    }
}

impl Visitor for Definitions {
    fn enter(&mut self) {
        self.path_stack.enter();
    }

    fn exit(&mut self) {
        self.path_stack.exit();
    }

    fn visit_entry(&mut self, entry: &Entry) {
        self.current_entry = Some(Rc::new(entry.clone()));
    }

    fn visit_message(&mut self, message: &Message) {
        let segment = PathSegment::Message(message.identifier_name());
        self.record_identifier(&segment);
        self.update_signature(&segment, message.pattern().is_some());
    }

    fn visit_term(&mut self, term: &Term) {
        let segment = PathSegment::Term(term.identifier_name());
        self.record_identifier(&segment);
        self.update_signature(&segment, true);
    }

    fn visit_attribute(&mut self, attribute: &Attribute) {
        let segment = PathSegment::Attribute(attribute.identifier_name());
        self.record_identifier(&segment);
    }

    fn visit_variant(&mut self, variant: &Variant) {
        let segment = PathSegment::Variant(format!("[{}]", variant.variant_key()));
        self.record_identifier(&segment);
    }

    fn visit_default_variant(&mut self, variant: &DefaultVariant) {
        let segment = PathSegment::DefaultVariant(format!("[{}]", variant.variant_key()));
        self.record_identifier(&segment);
    }

    fn visit_message_reference(&mut self, _reference: &MessageReference) {
        // self.record_identifier(&reference.identifier_name());
    }

    fn visit_term_reference(&mut self, _reference: &TermReference) {
        // self.record_identifier(&reference.identifier_name());
    }

    fn visit_attribute_accessor(&mut self, _accessor: &AttributeAccessor) {
        // self.record_identifier(&accessor.identifier_name());
    }

    fn visit_variable_reference(&mut self, _reference: &VariableReference) {
        // self.record_identifier(&reference.identifier_name());
    }

    fn visit_function_reference(&mut self, _reference: &FunctionReference) {
        // self.record_identifier(&reference.identifier_name());
    }
}
