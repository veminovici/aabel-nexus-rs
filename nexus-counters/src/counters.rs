use std::{
    cmp::{max, min},
    ops::{Add, AddAssign},
};

use nexus_lattice::{Join, JoinAssign, Meet, MeetAssign};

/// A generalization of tuple of [`usize`] values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Counters<const N: usize>(pub(crate) [usize; N]);

impl<const N: usize> Default for Counters<N> {
    fn default() -> Self {
        Self([0; N])
    }
}

impl<const N: usize> Add for Counters<N> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(
            self.0
                .iter()
                .zip(other.0.iter())
                .map(|(x, y)| x + y)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        )
    }
}

impl<const N: usize> AddAssign for Counters<N> {
    fn add_assign(&mut self, other: Self) {
        let _ = self
            .0
            .iter_mut()
            .zip(other.0.iter())
            .map(|(x, y)| *x += *y)
            .count();
    }
}

impl<const N: usize> Join for Counters<N> {
    type Output = Self;

    fn join(self, other: Self) -> Self::Output {
        Self(
            self.0
                .iter()
                .zip(other.0.iter())
                .map(|(x, y)| max(*x, *y))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        )
    }
}

impl<const N: usize> JoinAssign for Counters<N> {
    fn join_assign(&mut self, other: Self) {
        self.0
            .iter_mut()
            .zip(other.0.iter())
            .map(|(x, y)| *x = max(*x, *y))
            .count();
    }
}

impl<const N: usize> Meet for Counters<N> {
    type Output = Self;

    fn meet(self, other: Self) -> Self::Output {
        Self(
            self.0
                .iter()
                .zip(other.0.iter())
                .map(|(x, y)| min(*x, *y))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        )
    }
}

impl<const N: usize> MeetAssign for Counters<N> {
    fn meet_assign(&mut self, other: Self) {
        self.0
            .iter_mut()
            .zip(other.0.iter())
            .map(|(x, y)| *x = min(*x, *y))
            .count();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        const ONE: Counters<4> = Counters([1, 0, 0, 0]);

        let vals = Counters([0, 0, 0, 0]);
        let vals = vals + ONE;
        assert_eq!(vals.0[0], 1);

        let vals = vals + ONE;
        assert_eq!(vals.0[0], 2);
    }

    #[test]
    fn add_assign() {
        const ONE: Counters<4> = Counters([1, 0, 0, 0]);

        let mut vals = Counters([0, 0, 0, 0]);
        vals += ONE;
        assert_eq!(vals.0[0], 1);

        vals += ONE;
        assert_eq!(vals.0[0], 2);
    }

    #[test]
    fn join() {
        let x = Counters([1, 2, 3]);
        let y = Counters([3, 2, 1]);
        let res = x.join(y);
        assert_eq!(res, Counters([3, 2, 3]));
    }

    #[test]
    fn join_assign() {
        let mut x = Counters([1, 2, 3]);
        let y = Counters([3, 2, 1]);
        x.join_assign(y);
        assert_eq!(x, Counters([3, 2, 3]));
    }

    #[test]
    fn meet() {
        let x = Counters([1, 2, 3]);
        let y = Counters([3, 2, 1]);
        let res = x.meet(y);
        assert_eq!(res, Counters([1, 2, 1]));
    }

    #[test]
    fn meet_assign() {
        let mut x = Counters([1, 2, 3]);
        let y = Counters([3, 2, 1]);
        x.meet_assign(y);
        assert_eq!(x, Counters([1, 2, 1]));
    }
}
