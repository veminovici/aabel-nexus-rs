use std::borrow::Borrow;

#[derive(Debug, Clone)]
pub struct Body(Vec<u8>);

impl Body {
    pub fn new() -> Self {
        Body(vec![])
    }
}

impl Default for Body {
    fn default() -> Self {
        Self::new()
    }
}

impl AsRef<[u8]> for Body {
    fn as_ref(&self) -> &[u8] {
        self.0.as_slice()
    }
}

impl Borrow<[u8]> for Body {
    fn borrow(&self) -> &[u8] {
        self.as_ref()
    }
}
