use regex::Regex;

use crate::fluent::{Path, PathSegment, QualifiedIdentifier};

pub fn identifier(s: &str) -> QualifiedIdentifier {
    let segment_from_capture = |capture: regex::Captures| {
        if let Some(m) = capture.name("message") {
            PathSegment::Message(m.as_str().into())
        } else if let Some(m) = capture.name("term") {
            PathSegment::Term(m.as_str().into())
        } else if let Some(m) = capture.name("attribute") {
            PathSegment::Attribute(m.as_str().into())
        } else if let Some(m) = capture.name("default_variant") {
            PathSegment::DefaultVariant(m.as_str()[1..].into())
        } else if let Some(m) = capture.name("variant") {
            PathSegment::Variant(m.as_str().into())
        } else if let Some(m) = capture.name("variable") {
            PathSegment::Variable(m.as_str().into())
        } else {
            unreachable!()
        }
    };

    // Note: This regex is for testing only.
    //       The QualifierIdentifier::from_str decodes dioxus-i18n usage only,
    //       which is more restrictive
    let regex = Regex::new(
        r"(?x)
            (?:
                (?P<term>-[\w-]+) |
                (?P<variable>\$[\w-]+) |
                (?P<default_variant>\[\*[\w-]+\]) |
                (?P<variant>\[[\w-]+\]) |
                (?P<attribute>\.[\w-]+) |
                (?P<message>[\w-]+)
            )",
    )
    .expect("required valid regex for identifier");

    let segments = regex
        .captures_iter(s)
        .map(segment_from_capture)
        .collect::<Vec<PathSegment>>();

    let path = Path::from(segments.as_slice());

    QualifiedIdentifier::from(&path)
}
