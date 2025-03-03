pub enum IntegrityStatus {
    Ok,
    Warning,
    Error,
}

impl IntegrityStatus {
    pub fn css_class(&self) -> &'static str {
        match self {
            IntegrityStatus::Ok => "ok",
            IntegrityStatus::Warning => "warning",
            IntegrityStatus::Error => "error",
        }
    }
}
