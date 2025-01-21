/*
    I tried generalising over the K type, but the problem then is that it cannot be known how
    much the vector should be grown, because there's no way to convert it to an usize.

    So we're placing an extra constraint on the key that is must be an usize, then we know how far
    to grow.

    One of the initial commits has this over a generic key, if the vector holds options, we can
    technically still support that, but we can't support resizing the vector if the entry is out of
    bounds.

*/

pub use vec_entry::VecEntry;
pub use vec_option_entry::VecOptionEntry;

/// Interface for a vec-like container.
pub trait VecInterface {
    /// The elements that are contained in this container.
    type ElementType;

    /// Resize the vector to this number of elements, populating values with f.
    fn resize_with<F: FnMut() -> Self::ElementType>(&mut self, new_size: usize, f: F);

    /// Returns the current length of the vector.
    fn len(&self) -> usize;
}

impl<T> VecInterface for Vec<T> {
    type ElementType = T;
    #[inline]
    fn resize_with<F: FnMut() -> Self::ElementType>(&mut self, new_size: usize, f: F) {
        self.resize_with(new_size, f)
    }
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
}

pub trait OptionInterface {
    type ElementType;
    fn as_mut(&mut self) -> Option<&mut Self::ElementType>;
    fn insert(&mut self, value: Self::ElementType) -> &mut Self::ElementType;
    fn is_some(&self) -> bool;
}

impl<T> OptionInterface for Option<T> {
    type ElementType = T;
    #[inline]
    fn as_mut(&mut self) -> Option<&mut Self::ElementType> {
        self.as_mut()
    }
    #[inline]
    fn insert(&mut self, value: Self::ElementType) -> &mut Self::ElementType {
        self.insert(value)
    }
    #[inline]
    fn is_some(&self) -> bool {
        self.is_some()
    }
}

/// Helper to retrieve the inner type of a vector of Options.
type ElementOfOptionalVec<C> =
    <<C as std::ops::Index<usize>>::Output as OptionInterface>::ElementType;

/// This supports any Vec<T>, but only considers entries vacant if beyond the current length.
mod vec_entry;

/// This supports Vec<Option<T>> and considers entries vacant on None or beyond the current length.
mod vec_option_entry;

#[cfg(test)]
mod test {
    #[test]
    fn test_type_alias() {
        use crate::ElementOfOptionalVec;
        type V = ElementOfOptionalVec<Vec<Option<u32>>>;
        let x: V = 3;
        let y: u32 = x;
        let _ = y;
    }
}
