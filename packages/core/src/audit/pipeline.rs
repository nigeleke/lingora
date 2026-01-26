use std::{
    collections::{HashMap, HashSet},
    marker::PhantomData,
};

use crate::{
    audit::{
        AuditIssue, AuditResult, Workspace,
        result::{AuditedDocument, DocumentRole},
    },
    domain::{HasLocale, LanguageRoot, Locale},
    error::LingoraError,
    fluent::{FluentDocument, FluentFile, ParsedFluentFile},
};

pub(super) struct Empty;

pub(super) struct Parsed {
    files: Vec<ParsedFluentFile>,
}

pub(super) struct DocumentsCollected {
    documents: Vec<FluentDocument>,
}

pub(super) struct DocumentsClassified {
    canonical: Option<FluentDocument>,
    primaries: Vec<FluentDocument>,
    variants: Vec<FluentDocument>,
    orphans: Vec<FluentDocument>,
}

pub(super) struct Audited {
    canonical: Option<FluentDocument>,
    primaries: Vec<FluentDocument>,
    variants: Vec<FluentDocument>,
    orphans: Vec<FluentDocument>,
}

pub struct Pipeline<S> {
    state: S,
    issues: Vec<AuditIssue>,
    _state: PhantomData<S>,
}

impl Default for Pipeline<Empty> {
    fn default() -> Self {
        Self {
            state: Empty,
            issues: Vec::default(),
            _state: Default::default(),
        }
    }
}

impl Pipeline<Empty> {
    pub fn parse_fluent_files(
        mut self,
        files: &[FluentFile],
    ) -> Result<Pipeline<Parsed>, LingoraError> {
        let files = files.into_iter().try_fold(Vec::new(), |mut acc, file| {
            let file = ParsedFluentFile::try_from(file)?;
            acc.push(file);
            Ok::<_, LingoraError>(acc)
        })?;

        self.emit_parse_error_issues(&files);

        let state = Parsed { files };

        Ok(Pipeline::<_> {
            state,
            issues: self.issues,
            _state: Default::default(),
        })
    }

    fn emit_parse_error_issues(&mut self, files: &[ParsedFluentFile]) {
        files
            .iter()
            .filter(|f| f.resource().is_none())
            .for_each(|f| self.issues.push(AuditIssue::parse_error(f)));
    }
}

impl Pipeline<Parsed> {
    pub fn collect_documents_by_locale(self) -> Pipeline<DocumentsCollected> {
        let locales = self
            .state
            .files
            .iter()
            .map(|f| f.locale())
            .collect::<HashSet<_>>();

        let documents = locales
            .into_iter()
            .map(|locale| FluentDocument::from_parsed_files(locale, &self.state.files))
            .collect();

        let state = DocumentsCollected { documents };

        Pipeline::<_> {
            state,
            issues: self.issues,
            _state: Default::default(),
        }
    }
}

impl Pipeline<DocumentsCollected> {
    pub fn classify_documents(
        mut self,
        canonical_locale: &Locale,
        primary_locales: &[Locale],
    ) -> Pipeline<DocumentsClassified> {
        self.emit_missing_bases(canonical_locale, primary_locales);

        let canonical = self
            .state
            .documents
            .iter()
            .find(|d| d.locale() == canonical_locale)
            .cloned();

        let primaries = Vec::from_iter(
            self.state
                .documents
                .iter()
                .filter(|d| primary_locales.contains(d.locale()))
                .cloned(),
        );

        let base_language_roots = canonical
            .iter()
            .chain(primaries.iter())
            .map(|d| d.language_root())
            .collect::<HashSet<_>>();

        let (variants, orphans): (Vec<FluentDocument>, Vec<FluentDocument>) = self
            .state
            .documents
            .iter()
            .filter(|document| {
                let is_canonical = Some(*document) == canonical.as_ref();
                let is_primary = primaries.contains(document);
                !(is_canonical || is_primary)
            })
            .map(|d| d.clone())
            .partition(|document| {
                let root = document.language_root();
                base_language_roots.contains(&root)
            });

        self.emit_undefined_bases(&orphans);

        let state = DocumentsClassified {
            canonical,
            primaries,
            variants,
            orphans,
        };

        Pipeline::<_> {
            state,
            issues: self.issues,
            _state: Default::default(),
        }
    }

    fn emit_undefined_bases(&mut self, orphans: &Vec<FluentDocument>) {
        let locales_by_root =
            orphans
                .iter()
                .map(|orphan| orphan.locale())
                .fold(HashMap::new(), |mut acc, locale| {
                    let root = LanguageRoot::from(locale);
                    acc.entry(root).or_insert(Vec::new()).push(locale.clone());
                    acc
                });

        locales_by_root.iter().for_each(|(root, locales)| {
            self.issues
                .push(AuditIssue::undefined_base_locale(&root, &locales));
        });
    }

    fn emit_missing_bases(&mut self, canonical_locale: &Locale, primary_locales: &[Locale]) {
        std::iter::once(canonical_locale)
            .chain(primary_locales.iter())
            .for_each(|locale| {
                let document = self
                    .state
                    .documents
                    .iter()
                    .find(|document| document.locale() == locale);
                if document.is_none() {
                    self.issues
                        .push(AuditIssue::missing_base_translation(locale))
                };
            });
    }
}

impl Pipeline<DocumentsClassified> {
    pub fn audit(mut self) -> Pipeline<Audited> {
        self.emit_duplicate_identifiers();
        self.emit_invalid_references();
        self.emit_canonical_to_primary_issues();
        self.emit_base_to_variant_issues();

        let state = Audited {
            canonical: self.state.canonical,
            primaries: self.state.primaries,
            variants: self.state.variants,
            orphans: self.state.orphans,
        };

        Pipeline::<_> {
            state,
            issues: self.issues,
            _state: Default::default(),
        }
    }

    pub fn emit_duplicate_identifiers(&mut self) {
        self.state
            .canonical
            .iter()
            .chain(self.state.primaries.iter())
            .chain(self.state.variants.iter())
            .chain(self.state.orphans.iter())
            .for_each(|document| {
                document
                    .duplicate_identifier_names()
                    .for_each(|identifier| {
                        self.issues.push(AuditIssue::duplicate_identifier(
                            document.locale(),
                            &identifier,
                        ))
                    })
            });
    }

    pub fn emit_invalid_references(&mut self) {
        self.state
            .canonical
            .iter()
            .chain(self.state.primaries.iter())
            .chain(self.state.variants.iter())
            .chain(self.state.orphans.iter())
            .for_each(|document| {
                document.invalid_references().for_each(|reference| {
                    self.issues
                        .push(AuditIssue::invalid_reference(document.locale(), &reference))
                })
            });
    }

    pub fn emit_canonical_to_primary_issues(&mut self) {
        if let Some(canonical) = &self.state.canonical {
            let canonical_identifiers = canonical.entry_identifiers().collect::<HashSet<_>>();
            self.state.primaries.iter().for_each(|primary| {
                let primary_identifiers = primary.entry_identifiers().collect::<HashSet<_>>();

                canonical_identifiers
                    .difference(&primary_identifiers)
                    .for_each(|i| {
                        self.issues
                            .push(AuditIssue::missing_translation(primary.locale(), i))
                    });

                canonical_identifiers
                    .intersection(&primary_identifiers)
                    .for_each(|i| {
                        if canonical.signature(i) != primary.signature(i) {
                            self.issues
                                .push(AuditIssue::signature_mismatch(primary.locale(), i));
                        }
                    });

                primary_identifiers
                    .difference(&canonical_identifiers)
                    .for_each(|i| {
                        self.issues
                            .push(AuditIssue::redundant_translation(primary.locale(), i));
                    });
            });
        }
    }

    pub fn emit_base_to_variant_issues(&mut self) {
        self.state
            .canonical
            .iter()
            .chain(self.state.primaries.iter())
            .for_each(|base| {
                let base_root = base.language_root();
                let base_identifiers = base.entry_identifiers().collect::<HashSet<_>>();

                self.state
                    .variants
                    .iter()
                    .filter(|variant| variant.language_root() == base_root)
                    .for_each(|variant| {
                        let variant_identifiers =
                            variant.entry_identifiers().collect::<HashSet<_>>();

                        base_identifiers
                            .intersection(&variant_identifiers)
                            .for_each(|i| {
                                if base.signature(i) != variant.signature(i) {
                                    self.issues
                                        .push(AuditIssue::signature_mismatch(variant.locale(), i));
                                }
                            });

                        variant_identifiers
                            .difference(&base_identifiers)
                            .for_each(|i| {
                                self.issues
                                    .push(AuditIssue::redundant_translation(variant.locale(), i));
                            });
                    });
            });
    }
}

impl Pipeline<Audited> {
    fn with_role<I>(
        role: DocumentRole,
        docs: I,
    ) -> impl Iterator<Item = (DocumentRole, FluentDocument)>
    where
        I: IntoIterator<Item = FluentDocument>,
    {
        docs.into_iter().map(move |d| (role, d.clone()))
    }

    fn to_node(role: DocumentRole, document: &FluentDocument) -> AuditedDocument {
        AuditedDocument::from_document(role, document)
    }

    pub fn get_result(self, workspace: &Workspace) -> AuditResult {
        let documents = Self::with_role(DocumentRole::Canonical, self.state.canonical)
            .chain(Self::with_role(DocumentRole::Primary, self.state.primaries))
            .chain(Self::with_role(DocumentRole::Variant, self.state.variants))
            .chain(Self::with_role(DocumentRole::Orphan, self.state.orphans))
            .map(|(role, document)| Self::to_node(role, &document))
            .collect::<Vec<_>>();

        AuditResult::new(&self.issues, &documents, workspace)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        audit::issue::{Kind, Subject},
        test_support::{identifier, locale, root, with_temp_fluent_files},
    };

    fn assert_issue_has(issues: &[AuditIssue], kind: Kind, subject: Subject) {
        let actual = issues
            .iter()
            .find_map(|i| (i.kind() == &kind && i.subject() == &subject).then_some(i.kind()));

        assert!(
            actual.is_some(),
            "expected issue kind {kind:?} subject {subject:?}, got: {issues:#?}"
        );
    }

    #[test]
    fn parse_errors() {
        with_temp_fluent_files(
            &[
                (
                    "en-GB",
                    r#"
kdahf(#Q)$)(
"#,
                ),
                (
                    "en-AU",
                    r#"
message = Hello
"#,
                ),
            ],
            |files| {
                let pipeline = Pipeline::default()
                    .parse_fluent_files(&files)
                    .expect("valid pipeline");

                assert_eq!(pipeline.issues.len(), 1);
                assert_issue_has(
                    &pipeline.issues,
                    Kind::ParseError,
                    Subject::File(files[0].path().to_path_buf()),
                );
            },
        );
    }

    #[test]
    fn missing_bases() {
        with_temp_fluent_files(
            &[
                (
                    "en-GB",
                    r#"
message = Hello
"#,
                ),
                (
                    "fr-FR",
                    r#"
message = Bonjour
"#,
                ),
            ],
            |files| {
                let canonical = locale("en-GB");
                let primaries = vec!["fr-FR", "de-DE"]
                    .iter()
                    .map(|l| locale(l))
                    .collect::<Vec<_>>();

                let pipeline = Pipeline::default()
                    .parse_fluent_files(&files)
                    .expect("valid pipeline")
                    .collect_documents_by_locale()
                    .classify_documents(&canonical, &primaries);

                assert_eq!(pipeline.issues.len(), 1);
                assert_issue_has(
                    &pipeline.issues,
                    Kind::MissingBase,
                    Subject::Locale(locale("de-DE")),
                );
            },
        );
    }

    #[test]
    fn undefined_bases() {
        with_temp_fluent_files(
            &[
                (
                    "en-GB",
                    r#"
message = Hello
"#,
                ),
                (
                    "fr-FR",
                    r#"
message = Bonjour
"#,
                ),
                (
                    "de-DE",
                    r#"
message = Guten Tag
"#,
                ),
            ],
            |files| {
                let canonical = locale("en-GB");
                let primaries = vec!["fr-FR"].iter().map(|l| locale(l)).collect::<Vec<_>>();

                let pipeline = Pipeline::default()
                    .parse_fluent_files(&files)
                    .expect("valid pipeline")
                    .collect_documents_by_locale()
                    .classify_documents(&canonical, &primaries);

                assert_eq!(pipeline.issues.len(), 1);
                assert_issue_has(
                    &pipeline.issues,
                    Kind::UndefinedBase,
                    Subject::LanguageRoot(root("de-DE")),
                );
            },
        );
    }

    #[test]
    fn duplicate_identifiers() {
        with_temp_fluent_files(
            &[(
                "en-GB",
                r#"
message = Hello
message = Hello again
-term   = World
-term   = World again
"#,
            )],
            |files| {
                let canonical = locale("en-GB");
                let primaries = vec![];

                let pipeline = Pipeline::default()
                    .parse_fluent_files(&files)
                    .expect("valid pipeline")
                    .collect_documents_by_locale()
                    .classify_documents(&canonical, &primaries)
                    .audit();

                assert_eq!(pipeline.issues.len(), 2);

                ["message", "-term"].iter().for_each(|i| {
                    assert_issue_has(
                        &pipeline.issues,
                        Kind::DuplicateIdentifier,
                        Subject::Entry(locale("en-GB"), identifier(i)),
                    );
                });
            },
        );
    }

    #[test]
    fn invalid_references() {
        with_temp_fluent_files(
            &[(
                "en-GB",
                r#"
message1 = Hello
    .attr1 = Attribute 1
message1ref = { message1 }
attr11ref = { message1.attr1 }
attr12ref = { message1.attr2 }
message2ref = { message2 }
attr2ref = { message2.attr1 }
-term1 = World
    .attr1 = Attribute 1
-term1ref = { -term1 }
termattr11ref = { -term1.attr1 }
termattr12ref = { -term1.attr2 }
-term2ref = { -term2 }
termattr1ref = { -term2.attr1 }
"#,
            )],
            |files| {
                let canonical = locale("en-GB");
                let primaries = vec![];

                let pipeline = Pipeline::default()
                    .parse_fluent_files(&files)
                    .expect("valid pipeline")
                    .collect_documents_by_locale()
                    .classify_documents(&canonical, &primaries)
                    .audit();

                assert_eq!(pipeline.issues.len(), 8);

                [
                    "message1.attr2",
                    "message2",
                    "message2.attr1",
                    "-term1.attr2",
                    "-term2",
                    "-term2.attr1",
                ]
                .iter()
                .for_each(|i| {
                    assert_issue_has(
                        &pipeline.issues,
                        Kind::InvalidReference,
                        Subject::Entry(locale("en-GB"), identifier(i)),
                    )
                });
            },
        );
    }

    #[test]
    fn duplicate_variants() {
        with_temp_fluent_files(
            &[(
                "en-GB",
                r#"
message =
    { $colour ->
        [red]    Red
        [green]  Green
        [red]    Red2
        *[other] Other
    }
message2 =
    { $colour ->
        [red]    Red
        [green]  Green
        [other]  Other
        *[other] Other
    }
"#,
            )],
            |files| {
                let canonical = locale("en-GB");
                let primaries = vec![];

                let pipeline = Pipeline::default()
                    .parse_fluent_files(&files)
                    .expect("valid pipeline")
                    .collect_documents_by_locale()
                    .classify_documents(&canonical, &primaries)
                    .audit();

                assert_eq!(pipeline.issues.len(), 2);

                ["message[red]", "message2[other]"].iter().for_each(|i| {
                    assert_issue_has(
                        &pipeline.issues,
                        Kind::DuplicateIdentifier,
                        Subject::Entry(locale("en-GB"), identifier(i)),
                    );
                });
            },
        );
    }

    #[test]
    fn canonical_to_primary_missing_translations() {
        with_temp_fluent_files(
            &[
                (
                    "en-AU",
                    r#"
message1 = G'day en 1
message2 = G'day en 2
message3 =
    .hello = G'day en 3
    .world = World 3
message4 =
    .hello = G'day en 4
    .world = World 4
-term1 = G'day en 1
-term2 = G'day en 2
"#,
                ),
                (
                    "it-IT",
                    r#"
message1 = Buongiorno it 1
message3 =
    .hello = Buongiorno it 3
    .world = Mondo 3
message4 =
    .hello = Buongiorno it 4
-term1 = Buongiorno it 1
"#,
                ),
            ],
            |files| {
                let canonical = locale("en-AU");
                let primaries = vec![locale("it-IT")];

                let pipeline = Pipeline::default()
                    .parse_fluent_files(&files)
                    .expect("valid pipeline")
                    .collect_documents_by_locale()
                    .classify_documents(&canonical, &primaries)
                    .audit();

                assert_eq!(pipeline.issues.len(), 3);

                ["message2", "-term2"].iter().for_each(|i| {
                    assert_issue_has(
                        &pipeline.issues,
                        Kind::MissingTranslation,
                        Subject::Entry(locale("it-IT"), identifier(i)),
                    );
                });
            },
        );
    }

    #[test]
    fn canonical_to_primary_signature_mismatch() {
        with_temp_fluent_files(
            &[
                (
                    "en-AU",
                    r#"
emails1 =
    { $unreadEmails ->
        [one] You have one unread email.
        [two] You have two unread emails.
        *[other] You have { $unreadEmails } unread emails.
    }
emails2 =
    { $unreadEmails ->
        [one] You have one unread email.
        [two] You have two unread emails.
        *[other] You have { $unreadEmails } unread emails.
    }
"#,
                ),
                (
                    "it-IT",
                    r#"
emails1 =
    { $unreadEmails ->
        [one] Hai un'email non letta.
        *[other] Hai { $unreadEmails } email non lette.
    }
emails2 =
    { $unreadEmails ->
        [one] Hai un'email non letta.
        [two] Hai due email non lette.
        *[other] Hai { $unreadEmails } email non lette.
    }
"#,
                ),
            ],
            |files| {
                let canonical = locale("en-AU");
                let primaries = vec![locale("it-IT")];

                let pipeline = Pipeline::default()
                    .parse_fluent_files(&files)
                    .expect("valid pipeline")
                    .collect_documents_by_locale()
                    .classify_documents(&canonical, &primaries)
                    .audit();

                assert_eq!(pipeline.issues.len(), 1);

                assert_issue_has(
                    &pipeline.issues,
                    Kind::SignatureMismatch,
                    Subject::Entry(locale("it-IT"), identifier("emails1")),
                );
            },
        );
    }

    #[test]
    fn canonical_to_primary_redundant_translations() {
        with_temp_fluent_files(
            &[
                (
                    "en-AU",
                    r#"
message1 = G'day en 1
-term1   = G'day en 1
"#,
                ),
                (
                    "it-IT",
                    r#"
message1 = Buongiorno it 1
    .world = Mondo 1
message2 = Buongiorno it 2
-term1 = Buongiorno it 1
-term2 = Buongiorno it 2
"#,
                ),
            ],
            |files| {
                let canonical = locale("en-AU");
                let primaries = vec![locale("it-IT")];

                let pipeline = Pipeline::default()
                    .parse_fluent_files(&files)
                    .expect("valid pipeline")
                    .collect_documents_by_locale()
                    .classify_documents(&canonical, &primaries)
                    .audit();

                assert_eq!(pipeline.issues.len(), 3);

                ["message2", "-term2"].iter().for_each(|i| {
                    assert_issue_has(
                        &pipeline.issues,
                        Kind::RedundantTranslation,
                        Subject::Entry(locale("it-IT"), identifier(i)),
                    );
                });
            },
        );
    }

    #[test]
    fn base_to_variant_missing_translations_is_valid() {
        with_temp_fluent_files(
            &[
                (
                    "en-GB",
                    r#"
message1 = Hello
message2 = Hello again
"#,
                ),
                (
                    "en-AU",
                    r#"
message1 = G'day
"#,
                ),
            ],
            |files| {
                let canonical = locale("en-GB");
                let primaries = vec![];

                let pipeline = Pipeline::default()
                    .parse_fluent_files(&files)
                    .expect("valid pipeline")
                    .collect_documents_by_locale()
                    .classify_documents(&canonical, &primaries)
                    .audit();

                assert_eq!(pipeline.issues.len(), 0);
            },
        );
    }

    #[test]
    fn base_to_variant_signature_mismatch_issues() {
        with_temp_fluent_files(
            &[
                (
                    "en-GB",
                    r#"
emails1 =
    { $unreadEmails ->
        [one] You have one unread email.
        [two] You have two unread emails.
        *[other] You have { $unreadEmails } unread emails.
    }
emails2 =
    { $unreadEmails ->
        [one] You have one unread email.
        [two] You have two unread emails.
        *[other] You have { $unreadEmails } unread emails.
    }
"#,
                ),
                (
                    "en-AU",
                    r#"
emails1 =
    { $unreadEmails ->
        [one] You have one unread email.
        *[other] You have { $unreadEmails } unread emails.
    }
emails2 =
    { $unreadEmails ->
        [one] You have one unread email.
        [two] You have two unread emails.
        *[other] You have { $unreadEmails } unread emails.
    }
"#,
                ),
            ],
            |files| {
                let canonical = locale("en-GB");
                let primaries = vec![];

                let pipeline = Pipeline::default()
                    .parse_fluent_files(&files)
                    .expect("valid pipeline")
                    .collect_documents_by_locale()
                    .classify_documents(&canonical, &primaries)
                    .audit();

                assert_eq!(pipeline.issues.len(), 1);

                assert_issue_has(
                    &pipeline.issues,
                    Kind::SignatureMismatch,
                    Subject::Entry(locale("en-AU"), identifier("emails1")),
                );
            },
        );
    }

    #[test]
    fn base_to_variant_redundant_translation_issues() {
        with_temp_fluent_files(
            &[
                (
                    "en-AU",
                    r#"
message1 = G'day en 1
-term1   = G'day en 1
"#,
                ),
                (
                    "en-GB",
                    r#"
message1 = Hello en 1
    .world = World 1
message2 = Hello en 2
-term1 = Hello en 1
-term2 = Hello en 2
"#,
                ),
            ],
            |files| {
                let canonical = locale("en-AU");
                let primaries = vec![];

                let pipeline = Pipeline::default()
                    .parse_fluent_files(&files)
                    .expect("valid pipeline")
                    .collect_documents_by_locale()
                    .classify_documents(&canonical, &primaries)
                    .audit();

                assert_eq!(pipeline.issues.len(), 3);

                ["message2", "-term2"].iter().for_each(|i| {
                    assert_issue_has(
                        &pipeline.issues,
                        Kind::RedundantTranslation,
                        Subject::Entry(locale("en-GB"), identifier(i)),
                    );
                });
            },
        );
    }
}
