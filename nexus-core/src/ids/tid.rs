use serde::{Deserialize, Serialize};
use std::fmt::{Display, LowerHex, UpperHex};

use crate::{Aid, Fid, IntoAid, IntoFid};

pub trait IntoTid {
    fn into_tid(self) -> Tid;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tid(pub(crate) Aid);

//
// Display, UpperHex, LowerHex traits
//

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

//
// Utility traits
//

impl AsRef<Aid> for Tid {
    fn as_ref(&self) -> &Aid {
        &self.0
    }
}

impl<A> From<A> for Tid
where
    A: IntoAid,
{
    fn from(tid: A) -> Self {
        Self(tid.into_aid())
    }
}

//
// From traits
//

impl From<Fid> for Tid {
    fn from(fid: Fid) -> Self {
        let aid = fid.as_ref();
        Self(*aid)
    }
}

//
// Into traits
//

impl IntoTid for Tid {
    fn into_tid(self) -> Tid {
        self
    }
}

impl IntoFid for Tid {
    fn into_fid(self) -> Fid {
        self.0.into_fid()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_aid() {
        let tid: Tid = 10.into();
        let serialized = serde_json::to_string(&tid).unwrap();

        // eprintln!("fid={serialized}");

        let deserialized: Tid = serde_json::from_str(&serialized).unwrap();
        assert_eq!(tid, deserialized)
    }
}
