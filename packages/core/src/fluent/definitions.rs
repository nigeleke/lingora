use std::{
    collections::{HashMap, HashSet},
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
    Variable(String),
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

impl From<&[PathSegment]> for Path {
    fn from(value: &[PathSegment]) -> Self {
        Self(Vec::from(value))
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
                | PathSegment::DefaultVariant(name)
                | PathSegment::Variable(name) => name,
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

    fn root(&self) -> &[PathSegment] {
        self.stack
            .iter()
            .find(|v| !v.is_empty())
            .map(Vec::as_slice)
            .unwrap_or(&[])
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
type References = Vec<Path>;

#[derive(Debug, Default)]
pub struct Definitions {
    current_entry: Option<Rc<Entry>>,
    path_stack: PathStack,
    entry_by_id: EntriesById,
    signatures: Signatures,
    reference_stack: PathStack,
    references: References,
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
        let path = Path::from(self.path_stack.current());

        if let Some(entry) = &self.current_entry {
            self.entry_by_id
                .entry(path.clone())
                .or_default()
                .push(entry.clone());

            self.append_signature_path(path);
        };
    }

    fn update_signature_has_value(&mut self, has_value: bool) {
        let root = Path::from(self.path_stack.root());
        let signature = self.signatures.entry(root).or_default();
        signature.has_value = has_value;
    }

    fn append_signature_path(&mut self, path: Path) {
        let root = Path::from(self.path_stack.root());
        let signature = self.signatures.entry(root).or_default();
        signature.paths.insert(path);
    }

    pub fn signature(&self, path: &Path) -> Option<&Signature> {
        self.signatures.get(path)
    }

    pub fn identifiers(&self) -> impl Iterator<Item = QualifiedIdentifier> {
        self.entry_by_id
            .keys()
            .map(|k| QualifiedIdentifier::from(k))
    }

    pub fn references(&self) -> impl Iterator<Item = QualifiedIdentifier> {
        self.references.iter().map(|k| QualifiedIdentifier::from(k))
    }
}

impl Visitor for Definitions {
    fn enter(&mut self) {
        self.path_stack.enter();
        self.reference_stack.enter();
    }

    fn exit(&mut self) {
        self.path_stack.exit();
        self.reference_stack.exit();
    }

    fn visit_entry(&mut self, entry: &Entry) {
        self.current_entry = Some(Rc::new(entry.clone()));
    }

    fn visit_message(&mut self, message: &Message) {
        let segment = PathSegment::Message(message.identifier_name());
        self.record_identifier(&segment);
        self.update_signature_has_value(message.pattern().is_some());
    }

    fn visit_term(&mut self, term: &Term) {
        let segment = PathSegment::Term(term.identifier_name());
        self.record_identifier(&segment);
        self.update_signature_has_value(true);
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

    fn visit_message_reference(&mut self, reference: &MessageReference) {
        let reference = reference.identifier_name();
        let segment = PathSegment::Message(reference);
        self.reference_stack.push(segment);
        self.references
            .push(Path::from(self.reference_stack.current()));
    }

    fn visit_term_reference(&mut self, reference: &TermReference) {
        let reference = reference.identifier_name();
        let segment = PathSegment::Term(reference);
        self.reference_stack.push(segment);
        self.references
            .push(Path::from(self.reference_stack.current()));
    }

    fn visit_attribute_accessor(&mut self, accessor: &AttributeAccessor) {
        let accessor = accessor.identifier_name();
        let segment = PathSegment::Attribute(accessor);
        self.reference_stack.push(segment);
        self.references
            .push(Path::from(self.reference_stack.current()));
    }

    fn visit_variable_reference(&mut self, reference: &VariableReference) {
        let mut path = Vec::from(self.path_stack.root());
        path.push(PathSegment::Variable(reference.identifier_name()));
        self.append_signature_path(Path::from(path.as_slice()));
    }
}
