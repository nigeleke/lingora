mod base_files_provided;
mod base_files_unique;
mod identifier_integrity;
mod reference_integrity;
mod rule;
mod translation_integrity;
mod valid_syntax;
mod variants_have_base;

pub use base_files_provided::BaseFilesProvidedRule;
pub use base_files_unique::BaseFilesUniqueRule;
pub use identifier_integrity::IdentifierIntegrityRule;
pub use reference_integrity::ReferenceIntegrityRule;
pub use rule::AuditRule;
pub use translation_integrity::TranslationIntegrityRule;
pub use valid_syntax::ValidSyntaxRule;
pub use variants_have_base::VariantsHaveBaseRule;

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

#[macro_export]
macro_rules! assert_issue {
    (
        $issues:expr,
        $kind:expr
    ) => {
        assert!(
            $issues.iter().any(|issue| { issue.kind() == &$kind }),
            "expected issue of kind {:?}",
            $kind
        );
    };

    (
        $issues:expr,
        $kind:expr,
        $ident:expr
    ) => {
        assert!(
            $issues.iter().any(|issue| {
                issue.kind() == &$kind
                    && issue
                        .identifier()
                        .map(|id| id.to_meta_string() == $ident)
                        .unwrap_or(false)
            }),
            "expected issue of kind {:?} with identifier {:?}",
            $kind,
            $ident
        );
    };

    (
        not,
        $issues:expr,
        $kind:expr
    ) => {
        assert!(
            !$issues.iter().any(|issue| { issue.kind() == &$kind }),
            "unexpected issue of kind {:?}",
            $kind
        );
    };

    (
        not,
        $issues:expr,
        $kind:expr,
        $ident:expr
    ) => {
        assert!(
            !$issues.iter().any(|issue| {
                issue.kind() == &$kind
                    && issue
                        .identifier()
                        .map(|id| id.to_meta_string() == $ident)
                        .unwrap_or(false)
            }),
            "unexpected issue of kind {:?} with identifier {:?}",
            $kind,
            $ident
        );
    };
}
