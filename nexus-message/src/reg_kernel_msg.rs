use crate::Dispatch;
use crate::{Header, MessageExt};
use actix::{Message, WeakRecipient};
use std::borrow::Borrow;
use std::fmt::Display;

pub struct RegKernel(WeakRecipient<Dispatch>);

impl RegKernel {
    const SHORT_NAME: &'static str = "KRNL+";

    pub fn new(recipient: WeakRecipient<Dispatch>) -> Self {
        Self(recipient)
    }

    pub fn recipient(&self) -> &WeakRecipient<Dispatch> {
        &self.0
    }
}

impl Message for RegKernel {
    type Result = ();
}

impl MessageExt for RegKernel {
    fn short_name(&self) -> &str {
        Self::SHORT_NAME
    }

    fn headers(&self) -> impl Iterator<Item = &Header> {
        [].into_iter()
    }

    fn body(&self) -> impl Iterator<Item = u8> {
        [].into_iter()
    }
}

impl From<WeakRecipient<Dispatch>> for RegKernel {
    fn from(recipient: WeakRecipient<Dispatch>) -> Self {
        Self::new(recipient)
    }
}

impl AsRef<WeakRecipient<Dispatch>> for RegKernel {
    fn as_ref(&self) -> &WeakRecipient<Dispatch> {
        self.recipient()
    }
}

impl Borrow<WeakRecipient<Dispatch>> for RegKernel {
    fn borrow(&self) -> &WeakRecipient<Dispatch> {
        self.as_ref()
    }
}

impl Display for RegKernel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::SHORT_NAME)
    }
}
