#[derive(Debug, PartialEq, Eq, Hash)]
pub enum AnnotatedIdentifierState {
    Ok,
    OkUsingTargetFallback,
    MissingTarget,
    SuperfluousTarget,
    SuperfluousTargetFallback,
}

impl AnnotatedIdentifierState {
    pub fn css_class(&self) -> String {
        match self {
            AnnotatedIdentifierState::Ok => "ok",
            AnnotatedIdentifierState::OkUsingTargetFallback => "ok-uses-target-fallback",
            AnnotatedIdentifierState::MissingTarget => "missing-target",
            AnnotatedIdentifierState::SuperfluousTarget => "superfluous-target",
            AnnotatedIdentifierState::SuperfluousTargetFallback => "superfluous-target-fallback",
        }
        .to_string()
    }
}
