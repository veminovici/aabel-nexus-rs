use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, LowerHex, UpperHex},
    hash::Hash,
};

pub trait IntoSid {
    fn into_sid(self) -> Sid;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Sid(usize);

//
// Display, UpperHex, LowerHex traits
//

impl Display for Sid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "S{}", self.0)
    }
}

impl UpperHex for Sid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "S{:X}", self.0)
    }
}

impl LowerHex for Sid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "S{:x}", self.0)
    }
}

//
// Utility traits
//

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

//
// From traits
//

impl From<Sid> for usize {
    fn from(sid: Sid) -> Self {
        sid.0
    }
}

impl From<usize> for Sid {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

//
// Into traits
//

impl IntoSid for usize {
    fn into_sid(self) -> Sid {
        self.into()
    }
}

impl IntoSid for Sid {
    fn into_sid(self) -> Sid {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json() {
        let sid = Sid::from(10);
        let serialized = serde_json::to_string(&sid).unwrap();

        // eprintln!("aid={serialized}");

        let deserialized: Sid = serde_json::from_str(&serialized).unwrap();
        assert_eq!(sid, deserialized)
    }
}
