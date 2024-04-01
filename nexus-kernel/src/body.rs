use std::fmt::{Display, LowerHex, UpperHex};

pub struct Body(Vec<u8>);

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

impl AsRef<[u8]> for Body {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl IntoIterator for Body {
    type Item = <Vec<u8> as IntoIterator>::Item;

    type IntoIter = <Vec<u8> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<I> From<I> for Body
where
    I: Iterator<Item = u8>,
{
    fn from(values: I) -> Self {
        Self(values.collect())
    }
}
