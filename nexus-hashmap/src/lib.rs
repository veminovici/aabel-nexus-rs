//! The crate extends the [`std::collections::HashMap`] functionality.
//!
//! # Example
//!
//! ```
//! use nexus_hashmap::*;
//! use std::collections::HashMap;
//!
//! let mut map: HashMap<usize, u8> = HashMap::new();
//! let res = map.insert(1usize, 10);
//! assert!(res.is_none());
//!
//! let res = map.update_or_insert(1usize, |u| *u += 1, 1);
//! assert_eq!(res, Some(11));
//!
//! ```
mod hashmap_ext;

pub use hashmap_ext::*;
