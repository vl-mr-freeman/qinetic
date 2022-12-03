//! Stack allocator functionality.

/// Stack [`Allocator`]
///
/// # Examples
/// ```
/// # use qinetic_utils::prelude::*;
/// #
/// let stack_allocator = StackAllocator::with_capacity(std::mem::size_of<u32> * 1024);
/// ```
pub struct StackAllocator {
    /// Size of allocated `memory`.
    allocated: usize,

    /// Capacity of allocated `memory`.
    capacity: usize,
}

impl StackAllocator {
    const CAPACITY: usize = 64;

    /// Returns a [`StackAllocator`] with `default` capacity.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_utils::prelude::*;
    /// #
    /// let stack_allocator = StackAllocator::new();
    /// ```
    pub fn new() -> Self {
        Self {
            allocated: 0,
            capacity: Self::CAPACITY,
        }
    }

    /// Returns a [`StackAllocator`] with given capacity.
    ///
    /// # Examples
    /// ```
    /// # use qinetic_utils::prelude::*;
    /// #
    /// let stack_allocator = StackAllocator::with_capacity(std::mem::size_of<u32> * 1024);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            allocated: 0,
            capacity,
        }
    }
}

impl Default for StackAllocator {
    fn default() -> Self {
        Self {
            allocated: 0,
            capacity: Self::CAPACITY,
        }
    }
}
