use crate::Counters;

/// Specific functionality for tuples of two elements.
impl Counters<2> {
    /// The successed tuple.
    pub const SUCCEEDED2: Counters<2> = Counters([1, 0]);

    /// The failed tuple.
    pub const FAILED2: Counters<2> = Counters([0, 1]);

    /// Increments the succeeded value in the given tuple.
    pub fn incr_succeeded(&mut self) {
        *self += Self::SUCCEEDED2;
    }

    /// Increments the failed value in the given tuple.
    pub fn incr_failed(&mut self) {
        *self += Self::FAILED2;
    }

    /// Creates a new tuple from two given values.
    pub fn from_succeeded_failed(succeeded: usize, failed: usize) -> Self {
        Self([succeeded, failed])
    }

    /// Returns the succeeded value.
    pub fn succeeded(&self) -> usize {
        self.0[0]
    }

    /// Returns the failed value.
    pub fn failed(&self) -> usize {
        self.0[1]
    }
}
