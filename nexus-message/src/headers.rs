use nexus_ids::{Aid, Sid};

/// The headers supported in the messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Header {
    /// From header
    From(Aid),
    /// To header
    To(Aid),
    /// Session header
    Session(Sid),
}

impl Header {
    /// Attempts to retrieve the from header.
    pub fn try_from(&self) -> Option<&Aid> {
        match self {
            Header::From(aid) => Some(aid),
            _ => None,
        }
    }

    /// Attempts to retrieve the to header.
    pub fn try_to(&self) -> Option<&Aid> {
        match self {
            Header::To(aid) => Some(aid),
            _ => None,
        }
    }

    /// Attempts to retrieve the session header.
    pub fn try_session(&self) -> Option<&Sid> {
        match self {
            Header::Session(sid) => Some(sid),
            _ => None,
        }
    }
}
