/// Join opertion as it is defined in lattice.
pub trait Join<Rhs = Self> {
    /// The resulting type after applying the join operator.
    type Output;

    /// Joins two elements of the lattice.
    fn join(self, rhs: Rhs) -> Self::Output;
}

/// Join operation as it is defined in lattice.
pub trait JoinAssign<Rhs = Self> {
    /// Applies the result of the join operation to the current element
    fn join_assign(&mut self, rhs: Rhs);
}
