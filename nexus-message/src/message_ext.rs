use crate::Header;
use actix::Message;
use nexus_ids::{Aid, Sid};

pub trait MessageExt: Message {
    /// The short name of the message
    fn short_name(&self) -> &str;

    /// Returns a sequence of references to the message headers.
    fn headers(&self) -> impl Iterator<Item = &Header>;

    /// Returns a sequence of the [`u8`] values that make the body.
    fn body(&self) -> impl Iterator<Item = u8>;

    /// Attemtps to retrieve the from header value.
    fn from(&self) -> Option<&Aid> {
        self.headers()
            .filter_map(|h| h.try_from())
            .take(1)
            .fold(None, |acc, h| acc.or(Some(h)))
    }

    /// Attempts to retrieve the to header value.
    fn to(&self) -> Option<&Aid> {
        self.headers()
            .filter_map(|h| h.try_to())
            .take(1)
            .fold(None, |acc, h| acc.or(Some(h)))
    }

    /// Attempts to retrieve the session header value.
    fn session(&self) -> Option<&Sid> {
        self.headers()
            .filter_map(|h| h.try_session())
            .take(1)
            .fold(None, |acc, h| acc.or(Some(h)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nexus_ids::SidGenerator;

    struct MyMessage {
        from: Header,
        to: Header,
        session: Header,
    }

    impl MyMessage {
        fn new(fid: Aid, tid: Aid, sid: Sid) -> Self {
            Self {
                from: Header::From(fid),
                to: Header::To(tid),
                session: Header::Session(sid),
            }
        }
    }

    impl Message for MyMessage {
        type Result = ();
    }

    impl MessageExt for MyMessage {
        fn headers(&self) -> impl Iterator<Item = &Header> {
            [&self.from, &self.to, &self.session].into_iter()
        }

        fn body(&self) -> impl Iterator<Item = u8> {
            vec![].into_iter()
        }

        fn short_name(&self) -> &str {
            "MYMSG"
        }
    }

    #[test]
    fn headers() {
        let fid = 10.into();
        let tid = 20.into();
        let sid = SidGenerator::from(fid).take(10).next().unwrap();
        let msg = MyMessage::new(fid, tid, sid);
        let mut hs = msg.headers();

        assert_eq!(hs.next().unwrap(), &Header::From(fid));
        assert_eq!(hs.next().unwrap(), &Header::To(tid));
        assert_eq!(hs.next().unwrap(), &Header::Session(sid));
    }

    #[test]
    fn from() {
        let fid = 10.into();
        let tid = 20.into();
        let sid = SidGenerator::from(fid).take(10).next().unwrap();
        let msg = MyMessage::new(fid, tid, sid);

        let from = msg.from().unwrap();
        assert_eq!(from, &fid);
    }

    #[test]
    fn to() {
        let fid = 10.into();
        let tid = 20.into();
        let sid = SidGenerator::from(fid).take(10).next().unwrap();
        let msg = MyMessage::new(fid, tid, sid);

        let to = msg.to().unwrap();
        assert_eq!(to, &tid);
    }

    #[test]
    fn session() {
        let fid = Aid::from(10);
        let tid = Aid::from(20);
        let sid = SidGenerator::from(fid).take(10).next().unwrap();
        let msg = MyMessage::new(fid, tid, sid);

        let session = msg.session().unwrap();
        assert_eq!(session, &sid);
    }
}
