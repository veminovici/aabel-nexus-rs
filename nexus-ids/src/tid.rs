use std::{
    fmt::{Display, LowerHex, UpperHex},
    hash::Hash,
};

use crate::{Aid, Fid};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tid(Aid);

impl Display for Tid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl UpperHex for Tid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:X}", self.0)
    }
}

impl LowerHex for Tid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}", self.0)
    }
}

impl AsRef<Aid> for Tid {
    fn as_ref(&self) -> &Aid {
        &self.0
    }
}

impl AsRef<usize> for Tid {
    fn as_ref(&self) -> &usize {
        self.0.as_ref()
    }
}

impl Hash for Tid {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl From<Aid> for Tid {
    fn from(aid: Aid) -> Self {
        Self(aid)
    }
}

impl From<usize> for Tid {
    fn from(value: usize) -> Self {
        let aid = Aid::from(value);
        aid.into()
    }
}

impl From<Tid> for Aid {
    fn from(tid: Tid) -> Self {
        tid.0
    }
}

pub trait IntoTid {
    fn into_tid(self) -> Tid;
}

impl IntoTid for Aid {
    fn into_tid(self) -> Tid {
        Tid(self)
    }
}

impl IntoTid for usize {
    fn into_tid(self) -> Tid {
        let aid = Aid::from(self);
        aid.into_tid()
    }
}

impl IntoTid for Fid {
    fn into_tid(self) -> Tid {
        let aid = Aid::from(self);
        aid.into_tid()
    }
}
