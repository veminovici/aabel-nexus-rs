use std::{
    fmt::{Display, LowerHex, UpperHex},
    hash::Hash,
};

use crate::{Aid, Tid};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fid(Aid);

impl Display for Fid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl UpperHex for Fid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:X}", self.0)
    }
}

impl LowerHex for Fid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:x}", self.0)
    }
}

impl AsRef<Aid> for Fid {
    fn as_ref(&self) -> &Aid {
        &self.0
    }
}

impl AsRef<usize> for Fid {
    fn as_ref(&self) -> &usize {
        self.0.as_ref()
    }
}

impl Hash for Fid {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl From<Aid> for Fid {
    fn from(aid: Aid) -> Self {
        Self(aid)
    }
}

impl From<usize> for Fid {
    fn from(value: usize) -> Self {
        let aid = Aid::from(value);
        aid.into()
    }
}

impl From<Fid> for Aid {
    fn from(fid: Fid) -> Self {
        fid.0
    }
}

pub trait IntoFid {
    fn into_fid(self) -> Fid;
}

impl IntoFid for Aid {
    fn into_fid(self) -> Fid {
        Fid(self)
    }
}

impl IntoFid for usize {
    fn into_fid(self) -> Fid {
        let aid = Aid::from(self);
        aid.into_fid()
    }
}

impl IntoFid for Tid {
    fn into_fid(self) -> Fid {
        let aid = Aid::from(self);
        aid.into_fid()
    }
}
