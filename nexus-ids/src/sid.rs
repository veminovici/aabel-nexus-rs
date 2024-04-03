use crate::Aid;
use std::{fmt::Display, hash::Hash};

/// The session identifier. It can be used for debugging purposes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sid {
    aid: Aid,
    seq: usize,
}

impl Sid {
    /// Creates a new session identifier.
    pub fn new(aid: Aid, seq: usize) -> Self {
        Self { aid, seq }
    }

    /// Create a new session by updating the sequence number.
    pub fn with_seq(self, seq: usize) -> Self {
        Self::new(self.aid, seq)
    }

    /// Returns the actor identifier.
    pub fn aid(&self) -> &Aid {
        &self.aid
    }

    /// Returns the sequence number.
    pub fn seq_number(&self) -> usize {
        self.seq
    }
}

impl Hash for Sid {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.aid.hash(state);
        self.seq.hash(state);
    }
}

impl Display for Sid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.aid, self.seq)
    }
}

impl From<Aid> for Sid {
    fn from(aid: Aid) -> Self {
        Self { aid, seq: 0 }
    }
}

impl PartialOrd for Sid {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.aid == other.aid {
            return self.seq.partial_cmp(&other.seq);
        }

        None
    }
}

mod generators {
    use crate::{Aid, Sid};

    pub struct SidGenerator {
        aid: Aid,
        seq: usize,
    }

    impl Iterator for SidGenerator {
        type Item = Sid;

        fn next(&mut self) -> Option<Self::Item> {
            let sid = Sid::new(self.aid, self.seq);
            self.seq += 1;
            Some(sid)
        }
    }

    impl From<Aid> for SidGenerator {
        fn from(aid: Aid) -> Self {
            Self {
                aid,
                seq: Default::default(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::generators::*;
    use super::*;
    use std::cmp::Ordering;

    #[test]
    fn generate_aid() {
        let aid = 10.into();
        let gen = SidGenerator::from(aid);
        let mut xs = gen.map(|sid| sid.aid);

        assert_eq!(xs.next(), Some(aid));
        assert_eq!(xs.next(), Some(aid));
    }

    #[test]
    fn generate_seq() {
        let aid = Aid::from(10);
        let gen = SidGenerator::from(aid);
        let mut xs = gen.map(|sid| sid.seq);

        assert_eq!(xs.next(), Some(0));
        assert_eq!(xs.next(), Some(1));
    }

    #[test]
    fn compare() {
        let aid1 = Aid::from(10);
        let mut gen1 = SidGenerator::from(aid1);

        let sid1 = gen1.next().unwrap();
        let sid2 = gen1.next().unwrap();
        let sid3 = gen1.next().unwrap();

        assert_eq!(sid1.partial_cmp(&sid2), Some(Ordering::Less));

        let aid2 = Aid::from(20);
        let sid3_2 = SidGenerator::from(aid2).skip(2).next().unwrap();

        assert_eq!(sid3.partial_cmp(&sid3_2), None)
    }

    #[test]
    fn with_seq() {
        let aid = Aid::from(10);
        let sid = Sid::from(aid);
        let sid = sid.with_seq(20);

        assert_eq!(sid.aid(), &aid);
        assert_eq!(sid.seq_number(), 20);
    }
}
