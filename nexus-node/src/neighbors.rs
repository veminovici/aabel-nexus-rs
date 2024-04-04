use std::{
    borrow::Borrow,
    ops::{Add, AddAssign, Sub, SubAssign},
};

pub struct Neighbors<T>(Vec<T>);

impl<T> Neighbors<T> {
    pub fn new() -> Self {
        Self(Default::default())
    }

    pub fn with_items(iter: impl Iterator<Item = T>) -> Self {
        Self::from(iter)
    }
}

impl<T> Default for Neighbors<T> {
    fn default() -> Self {
        Self::new()
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

impl<T, I> From<I> for Neighbors<T>
where
    I: Iterator<Item = T>,
{
    fn from(iter: I) -> Self {
        Self(Vec::from_iter(iter))
    }
}

impl<T, U> Add<U> for Neighbors<T>
where
    T: PartialEq + Eq + Copy,
    U: Borrow<T>,
{
    type Output = Self;

    fn add(self, u: U) -> Self::Output {
        let other = u.borrow();

        if self.0.iter().all(|t| t != other) {
            let mut ns = self.0;
            ns.push(*other);
            Self::from(ns.into_iter())
        } else {
            Self(self.0)
        }
    }
}

impl<T, U> AddAssign<U> for Neighbors<T>
where
    T: PartialEq + Eq + Copy,
    U: Borrow<T>,
{
    fn add_assign(&mut self, u: U) {
        let other = u.borrow();

        if self.0.iter().all(|t| t != other) {
            self.0.push(*other)
        }
    }
}

impl<T, U> Sub<U> for Neighbors<T>
where
    T: PartialEq + Eq,
    U: Borrow<T>,
{
    type Output = Self;

    fn sub(self, u: U) -> Self::Output {
        let other = u.borrow();
        Self::from(self.0.into_iter().filter(|t| t != other))
    }
}

impl<T, U> SubAssign<U> for Neighbors<T>
where
    T: PartialEq + Eq + Copy,
    U: Borrow<T>,
{
    fn sub_assign(&mut self, u: U) {
        let other = u.borrow();

        self.0 = self
            .0
            .iter()
            .filter_map(|t| if t != other { Some(*t) } else { None })
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
