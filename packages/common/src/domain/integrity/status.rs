pub enum Status {
    Ok,
    Warning,
    Error,
}

impl Status {
    pub fn css_class(&self) -> &'static str {
        match self {
            Status::Ok => "ok",
            Status::Warning => "warning",
            Status::Error => "error",
        }
    }
}
