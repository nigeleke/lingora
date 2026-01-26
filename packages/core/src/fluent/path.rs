#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PathSegment {
    Message(String),
    Term(String),
    Attribute(String),
    Variant(String),
    DefaultVariant(String),
    Variable(String),
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Path(Vec<PathSegment>);

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
        Some(self.cmp(other))
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
