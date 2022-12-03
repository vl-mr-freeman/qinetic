//! Linear allocator functionality.

/// Linear [`Allocator`].
///
/// # Examples
/// ```
/// # use qinetic_utils::prelude::*;
/// #
/// let linear_allocator = LinearAllocator::with_capacity(std::mem::size_of<u32> * 1024);
/// ```
pub struct LinearAllocator {
    /// Size of allocated `memory`.
    allocated: usize,

    /// Capacity of allocated `memory`.
    capacity: usize,
}

impl LinearAllocator {
    const CAPACITY: usize = 64;

    /// Returns a [`LinearAllocator`] with `default` capacity.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_utils::prelude::*;
    /// #
    /// let linear_allocator = LinearAllocator::new();
    /// ```
    pub fn new() -> Self {
        Self {
            allocated: 0,
            capacity: Self::CAPACITY,
        }
    }

    /// Returns a [`LinearAllocator`] with given capacity.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_utils::prelude::*;
    /// #
    /// let linear_allocator = LinearAllocator::with_capacity(std::mem::size_of<u32> * 1024);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            allocated: 0,
            capacity,
        }
    }
}

impl Default for LinearAllocator {
    fn default() -> Self {
        Self {
            allocated: 0,
            capacity: Self::CAPACITY,
        }
    }
}
