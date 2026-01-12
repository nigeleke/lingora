mod identifier_integrity;
mod reference_integrity;
mod rule;
mod translation_integrity;
mod valid_syntax;

pub use identifier_integrity::IdentifierIntegrityRule;
pub use reference_integrity::ReferenceIntegrityRule;
pub use rule::AuditRule;
pub use translation_integrity::TranslationIntegrityRule;
pub use valid_syntax::ValidSyntaxRule;

fn emit_ordered<I, F>(iter: I, mut emit: F)
where
    I: IntoIterator,
    I::Item: Ord,
    F: FnMut(I::Item),
{
    use std::collections::BTreeSet;
    for item in BTreeSet::from_iter(iter) {
        emit(item);
    }
}
