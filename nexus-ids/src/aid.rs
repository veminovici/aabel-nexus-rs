use std::{borrow::Borrow, fmt::Display, hash::Hash};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Aid(usize);

impl Display for Aid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A{}", self.0)
    }
}

impl From<usize> for Aid {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<Aid> for usize {
    fn from(aid: Aid) -> Self {
        aid.0
    }
}

impl AsRef<usize> for Aid {
    fn as_ref(&self) -> &usize {
        &self.0
    }
}

impl Borrow<usize> for Aid {
    fn borrow(&self) -> &usize {
        &self.0
    }
}

impl Hash for Aid {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
