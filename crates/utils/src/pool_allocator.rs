//! Pool allocator functionality.

/// Pool [`Allocator`]
///
/// # Examples
/// ```
/// # use qinetic_utils::prelude::*;
/// #
/// let pool_allocator = PoolAllocator::with_capacity(std::mem::size_of<u32>, 1024);
/// ```
pub struct PoolAllocator {
    /// Size of allocated `memory`.
    allocated: usize,

    /// Size of chunk.
    chunk: usize,

    /// Capacity of allocated `memory`.
    capacity: usize,
}

impl PoolAllocator {
    const CAPACITY: usize = 64;

    /// Returns a [`PoolAllocator`] with `default` capacity.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_utils::prelude::*;
    /// #
    /// let pool_allocator = PoolAllocator::new(std::mem::size_of<u32>);
    /// ```
    pub fn new(chunk: usize) -> Self {
        Self {
            allocated: 0,
            chunk,
            capacity: Self::CAPACITY,
        }
    }

    /// Returns a [`PoolAllocator`] with given chunk size and capacity.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_utils::prelude::*;
    /// #
    /// let pool_allocator = PoolAllocator::with_capacity(std::mem::size_of<u32>, 1024);
    /// ```
    pub fn with_capacity(chunk: usize, capacity: usize) -> Self {
        Self {
            allocated: 0,
            chunk,
            capacity,
        }
    }
}
