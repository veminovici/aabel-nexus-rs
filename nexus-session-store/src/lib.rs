use nexus_hashmap::HashmapExt;
use nexus_ids::{Aid, Sid};
use std::collections::HashMap;

pub struct Store<V>(HashMap<Sid, HashMap<Aid, V>>);

impl<V> Store<V> {
    pub fn new() -> Self {
        Self::default()
    }

    fn get_create_session_mut<'a>(&'a mut self, sid: &Sid) -> &'a mut HashMap<Aid, V> {
        if !self.0.contains_key(sid) {
            let _ = self.0.insert(*sid, Default::default());
        }

        self.0.get_mut(sid).unwrap()
    }

    pub fn update<F>(&mut self, sid: &Sid, aid: &Aid, f: F, def: V)
    where
        F: FnMut(&mut V),
        V: Clone,
    {
        let _ = self
            .get_create_session_mut(sid)
            .update_or_insert(*aid, f, def);
    }

    pub fn get(&self, sid: &Sid, aid: &Aid) -> Option<&V> {
        self.0.get(sid).and_then(|hm| hm.get(aid))
    }

    pub fn get_mut(&mut self, sid: &Sid, aid: &Aid) -> Option<&mut V> {
        self.0.get_mut(sid).and_then(|hm| hm.get_mut(aid))
    }

    pub fn contains_sid(&self, sid: &Sid) -> bool {
        self.0.contains_key(sid)
    }

    pub fn contains_aid(&self, sid: &Sid, aid: &Aid) -> bool {
        self.0
            .get(sid)
            .map(|hm| hm.contains_key(aid))
            .unwrap_or(false)
    }
}

impl<V> Default for Store<V> {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store() {
        let aid: Aid = 10.into();
        let sid = Sid::from(aid);

        let mut store = Store::<u8>::new();
        store.update(&sid, &aid, |v| *v += 1, 10);

        // The store should have out session stored.
        assert!(store.contains_sid(&sid));
        assert!(store.contains_aid(&sid, &aid));

        // The session should store one entry of value 10 (the default value)
        let r = store.get(&sid, &aid).unwrap();
        assert_eq!(&10, r);

        // Update the stored value (increment its value), make sure we dont default this time.
        store.update(&sid, &aid, |v| *v += 1, 5);
        let r = store.get(&sid, &aid).unwrap();
        assert_eq!(&11, r);

        // Gets a mutable state and update the value.
        let _ = store.get_mut(&sid, &aid).map(|u| *u += 10);

        // Check if the new value is saved.
        let r = store.get(&sid, &aid).unwrap();
        assert_eq!(&21, r);
    }
}
