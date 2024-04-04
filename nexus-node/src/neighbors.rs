use std::{
    borrow::Borrow,
    ops::{Add, AddAssign, Sub, SubAssign},
};

pub struct Neighbors<T>(Vec<T>);

impl<T> Neighbors<T> {
    pub fn new() -> Self {
        Self(Default::default())
    }

    pub fn with_items(xs: impl Iterator<Item = T>) -> Self {
        Self(Vec::from_iter(xs))
    }
}

impl<T> Default for Neighbors<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> AsRef<[T]> for Neighbors<T> {
    fn as_ref(&self) -> &[T] {
        self.0.as_ref()
    }
}

impl<T> Borrow<[T]> for Neighbors<T> {
    fn borrow(&self) -> &[T] {
        self.as_ref()
    }
}

impl<T, U> Add<U> for Neighbors<T>
where
    T: PartialEq + Eq + Clone,
    U: Borrow<T>,
{
    type Output = Self;

    fn add(self, aid: U) -> Self::Output {
        let aid = aid.borrow();

        if self.0.iter().all(|a| a != aid) {
            let mut ns = self.0;
            ns.push(aid.clone());
            Self(ns)
        } else {
            Self(self.0)
        }
    }
}

impl<T, U> AddAssign<U> for Neighbors<T>
where
    T: PartialEq + Eq + Clone,
    U: Borrow<T>,
{
    fn add_assign(&mut self, aid: U) {
        let aid = aid.borrow();
        if self.0.iter().all(|a| a != aid) {
            self.0.push(aid.clone())
        }
    }
}

impl<T, U> Sub<U> for Neighbors<T>
where
    T: PartialEq + Eq + Clone,
    U: Borrow<T>,
{
    type Output = Self;

    fn sub(self, aid: U) -> Self::Output {
        let aid = aid.borrow();
        let ns = self.0.into_iter().filter(|a| a != aid).collect();
        Self(ns)
    }
}

impl<T, U> SubAssign<U> for Neighbors<T>
where
    T: PartialEq + Eq + Clone,
    U: Borrow<T>,
{
    fn sub_assign(&mut self, aid: U) {
        let aid = aid.borrow();

        self.0 = self
            .0
            .iter()
            .filter_map(|a| if a != aid { Some(a.clone()) } else { None })
            .collect::<Vec<_>>();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nexus_ids::Aid;

    #[test]
    fn add() {
        let xs = Neighbors::with_items([Aid::from(1), Aid::from(2)].into_iter());

        let xs = xs + Aid::from(3);
        assert_eq!(xs.as_ref(), [Aid::from(1), Aid::from(2), Aid::from(3)]);

        let xs = xs + Aid::from(3);
        assert_eq!(xs.as_ref(), [Aid::from(1), Aid::from(2), Aid::from(3)]);
    }

    #[test]
    fn add_assign() {
        let mut xs = Neighbors::with_items([Aid::from(1), Aid::from(2)].into_iter());

        xs += Aid::from(3);
        assert_eq!(xs.as_ref(), [Aid::from(1), Aid::from(2), Aid::from(3)]);

        xs += Aid::from(3);
        assert_eq!(xs.as_ref(), [Aid::from(1), Aid::from(2), Aid::from(3)]);
    }

    #[test]
    fn sub() {
        let xs = Neighbors::with_items([Aid::from(1), Aid::from(2)].into_iter());

        let xs = xs - Aid::from(3);
        assert_eq!(xs.as_ref(), [Aid::from(1), Aid::from(2)]);

        let xs = xs - Aid::from(2);
        assert_eq!(xs.as_ref(), [Aid::from(1)]);
    }

    #[test]
    fn sub_assign() {
        let mut xs = Neighbors::with_items([Aid::from(1), Aid::from(2)].into_iter());

        xs -= Aid::from(3);
        assert_eq!(xs.as_ref(), [Aid::from(1), Aid::from(2)]);

        xs -= Aid::from(2);
        assert_eq!(xs.as_ref(), [Aid::from(1)]);
    }
}
