use crate::{Aid, IntoAid, IntoTid, Tid};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, LowerHex, UpperHex},
    hash::Hash,
};

impl From<Tid> for Fid {
    fn from(tid: Tid) -> Self {
        let aid = tid.as_ref();
        Self(*aid)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Fid(pub(crate) Aid);

//
// Display, UppperHex, LowerHex traits
//

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

//
// Utility triats
//

impl AsRef<Aid> for Fid {
    fn as_ref(&self) -> &Aid {
        &self.0
    }
}

impl Hash for Fid {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

//
// From traits
//

impl<A> From<A> for Fid
where
    A: IntoAid,
{
    fn from(aid: A) -> Self {
        Self(aid.into_aid())
    }
}

//
// Into traits
//

pub trait IntoFid {
    fn into_fid(self) -> Fid;
}

impl IntoFid for usize {
    fn into_fid(self) -> Fid {
        Fid(self.into_aid())
    }
}

impl IntoFid for Fid {
    fn into_fid(self) -> Fid {
        self
    }
}

impl IntoTid for Fid {
    fn into_tid(self) -> Tid {
        self.0.into_tid()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_aid() {
        let fid: Fid = 10.into();
        let serialized = serde_json::to_string(&fid).unwrap();

        // eprintln!("fid={serialized}");

        let deserialized: Fid = serde_json::from_str(&serialized).unwrap();
        assert_eq!(fid, deserialized)
    }
}
