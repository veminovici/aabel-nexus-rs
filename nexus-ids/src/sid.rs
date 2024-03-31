use std::{
    fmt::{Display, LowerHex, UpperHex},
    hash::Hash,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sid(usize);

impl Display for Sid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A{}", self.0)
    }
}

impl UpperHex for Sid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:X}", self)
    }
}

impl LowerHex for Sid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}", self)
    }
}

impl AsRef<usize> for Sid {
    fn as_ref(&self) -> &usize {
        &self.0
    }
}

impl Hash for Sid {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl From<usize> for Sid {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<&usize> for Sid {
    fn from(value: &usize) -> Self {
        Self(*value)
    }
}

impl From<Sid> for usize {
    fn from(aid: Sid) -> Self {
        aid.0
    }
}

pub trait IntoSid {
    fn into_sid(self) -> Sid;
}

impl IntoSid for usize {
    fn into_sid(self) -> Sid {
        self.into()
    }
}
