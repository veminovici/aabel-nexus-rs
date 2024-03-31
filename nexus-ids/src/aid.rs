use std::{
    fmt::{Display, LowerHex, UpperHex},
    hash::Hash,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Aid(usize);

impl Display for Aid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A{}", self.0)
    }
}

impl UpperHex for Aid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl LowerHex for Aid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl AsRef<usize> for Aid {
    fn as_ref(&self) -> &usize {
        &self.0
    }
}

impl Hash for Aid {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl From<usize> for Aid {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<&usize> for Aid {
    fn from(value: &usize) -> Self {
        Self(*value)
    }
}

impl From<Aid> for usize {
    fn from(aid: Aid) -> Self {
        aid.0
    }
}

pub trait IntoAid {
    fn into_aid(self) -> Aid;
}

impl IntoAid for usize {
    fn into_aid(self) -> Aid {
        self.into()
    }
}
