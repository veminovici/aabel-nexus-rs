use serde::{Deserialize, Serialize};
use std::fmt::{Display, LowerHex, UpperHex};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Body(Vec<u8>);

impl Body {
    fn new<I>(values: I) -> Self
    where
        I: Iterator<Item = u8>,
    {
        Self(values.collect())
    }
}

//
// Display, UpperHex, LowerHex traits
//

impl Display for Body {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let body = self
            .0
            .iter()
            .map(|u| format!("{u}"))
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{body}")
    }
}

impl UpperHex for Body {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let body = self
            .0
            .iter()
            .map(|u| format!("{u:X}"))
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{body}")
    }
}

impl LowerHex for Body {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let body = self
            .0
            .iter()
            .map(|u| format!("{u:x}"))
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "{body}")
    }
}

//
// Utility traits
//

impl AsRef<[u8]> for Body {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl IntoIterator for Body {
    type Item = u8;

    type IntoIter = <Vec<u8> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

//
// From traits
//

impl<I> From<I> for Body
where
    I: Iterator<Item = u8>,
{
    fn from(values: I) -> Self {
        Self::new(values)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_body() {
        let body = Body::new([1, 2, 3].into_iter());
        let serialized = serde_json::to_string(&body).unwrap();

        eprintln!("body: {serialized}");

        let deserialized: Body = serde_json::from_str(&serialized).unwrap();
        assert_eq!(body, deserialized);
    }
}
