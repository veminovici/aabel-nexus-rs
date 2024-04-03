//! A create for distributed counters
//!
//! # Example
//!
//! ```
//! use nexus_counters::*;
//! use nexus_lattice::*;
//!
//! let one: Counters<4> = Counters::new([1, 0, 0, 0]);
//!
//! let vals = Counters::new([0, 0, 0, 0]);
//!
//! let vals = vals + one;
//! assert_eq!(vals.as_ref()[0], 1);
//!
//! let vals = vals + one;
//! assert_eq!(vals.as_ref()[0], 2);
//!
//! let mut x = Counters::new([1, 2, 3]);
//! let y = Counters::new([3, 2, 1]);
//! x.join_assign(y);
//! assert_eq!(x, Counters::new([3, 2, 3]));
//!
//! let mut x = Counters::new([1, 2, 3]);
//! let y = Counters::new([3, 2, 1]);
//! x.meet_assign(y);
//! assert_eq!(x, Counters::new([1, 2, 1]));
//! ```

mod counters;
mod counters2;

pub use counters::*;
// pub use counters2::*;
