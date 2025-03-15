#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Identifier {
    context: Vec<String>,
    name: String,
}

impl Identifier {
    pub fn new(context: &[&str], name: &str) -> Self {
        let context = context.iter().map(|s| s.to_string());
        Self {
            context: Vec::from_iter(context),
            name: String::from(name),
        }
    }
}

impl Identifier {
    pub fn path(&self) -> String {
        let mut path = self.context.clone();
        path.push(self.name.clone());
        path.join(" / ")
    }

    pub fn stem(&self) -> String {
        self.name.clone()
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.stem().fmt(f)
    }
}
