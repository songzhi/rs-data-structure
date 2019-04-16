mod raw;
mod fx;
mod map;
mod scopeguard;
#[macro_use]
mod macros;
mod set;

pub use self::map::HashMap;
pub use self::set::HashSet;

pub mod hash_map {
    pub use super::map::*;
}

pub mod hash_set {
    pub use super::set::*;
}

#[cfg(feature = "nightly")]
#[cfg_attr(test, macro_use)]
extern crate alloc;
#[cfg(not(feature = "nightly"))]
extern crate std as alloc;

/// Augments `AllocErr` with a `CapacityOverflow` variant.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum CollectionAllocErr {
    /// Error due to the computed capacity exceeding the collection's maximum
    /// (usually `isize::MAX` bytes).
    CapacityOverflow,
    /// Error due to the allocator (see the `AllocErr` type's docs).
    AllocErr,
}