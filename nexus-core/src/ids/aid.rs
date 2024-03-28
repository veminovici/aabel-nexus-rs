use crate::{Fid, IntoFid, IntoTid, Tid};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, LowerHex, UpperHex},
    hash::Hash,
};

pub trait IntoAid {
    fn into_aid(self) -> Aid;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Aid(usize);

//
// Display, UpperHex, LowerHex traits
//

impl Display for Aid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A{}", self.0)
    }
}

impl UpperHex for Aid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A{:X}", self.0)
    }
}

impl LowerHex for Aid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A{:x}", self.0)
    }
}

//
// Utility traits
//

impl AsRef<usize> for Aid {
    fn as_ref(&self) -> &usize {
        &self.0
    }
}

impl Hash for Aid {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_usize(self.0)
    }
}

//
// From traits
//

impl From<Aid> for usize {
    fn from(aid: Aid) -> Self {
        aid.0
    }
}

impl From<usize> for Aid {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

//
// Into traits
//

impl IntoAid for usize {
    fn into_aid(self) -> Aid {
        self.into()
    }
}

impl IntoAid for Aid {
    fn into_aid(self) -> Aid {
        self
    }
}

impl IntoTid for Aid {
    fn into_tid(self) -> Tid {
        Tid(self)
    }
}

impl IntoFid for Aid {
    fn into_fid(self) -> crate::Fid {
        Fid(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json() {
        let aid = Aid::from(10);
        let serialized = serde_json::to_string(&aid).unwrap();

        // eprintln!("aid={serialized}");

        let deserialized: Aid = serde_json::from_str(&serialized).unwrap();
        assert_eq!(aid, deserialized)
    }
}
