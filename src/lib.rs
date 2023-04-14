use num::{Integer, Unsigned};

/// Implements constants for the size of something in bits or bytes,
/// whose size is known at compile time, and whose size will never change.
pub trait ConstantSize<M: Unsigned + Integer> {
    /// Size of implementor's type in bits
    const SIZE_IN_BITS: M;

    /// Size of the implementor's type in bytes.
    /// MAKE SURE that SIZE_IN_BITS = SIZE_IN_BYTES * 8
    const SIZE_IN_BYTES: M;
}

/// Implements functions for the size of something whose size is known at compile time, but which might grow or shrink.
pub trait MaybeMutSize<M: Unsigned + Integer> {
    /// Memory currently allocated in bits
    fn size_allocated_in_bits(&self) -> M;

    /// Memory currently allocated in bytes
    fn size_allocated_in_bytes(&self) -> M;

    // Of the memory currently allocated, returns the number of bits being used, whatever that means for this type.
    fn size_used_in_bits(&self) -> M;

    // Of the memory currently allocated, returns the number of bytes being used, whatever that means for this type.
    fn size_used_in_bytes(&self) -> M;
}

pub trait MaybeMutSizeCollection<M: Unsigned + Integer> {
    const ITEM_SIZE: M; // Size of a single item in the Collection

    /// Memory currently allocated in bits
    fn size_allocated_in_bits(&self) -> M;

    /// Memory currently allocated in bytes
    fn size_allocated_in_bytes(&self) -> M;

    // Of the memory currently allocated, returns the number of bits being used, whatever that means for this type.
    fn size_used_in_bits(&self) -> M;

    // Of the memory currently allocated, returns the number of bytes being used, whatever that means for this type.
    fn size_used_in_bytes(&self) -> M;
}

impl<M: Unsigned + Integer, T: ConstantSize<M>> MaybeMutSizeCollection<usize> for Vec<T> {
    const ITEM_SIZE_IN_BITS: usize = T::SIZE_IN_BITS;
    const ITEM_SIZE_IN_BYTES: usize = T::SIZE_IN_BYTES;

    fn size_allocated_in_bits(&self) -> usize {
        self.capacity() * Self::ITEM_SIZE
    }

    fn size_allocated_in_bytes(&self) -> usize {
        todo!()
    }

    fn size_used_in_bits(&self) -> usize {
        todo!()
    }

    fn size_used_in_bytes(&self) -> usize {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
