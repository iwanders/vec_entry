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

pub trait VecInterface {
    type ElementType;
    fn resize_with<F: FnMut() -> Self::ElementType>(&mut self, new_size: usize, f: F);
    fn len(&self) -> usize;
}

impl<T> VecInterface for Vec<T> {
    type ElementType = T;
    fn resize_with<F: FnMut() -> Self::ElementType>(&mut self, new_size: usize, f: F) {
        self.resize_with(new_size, f)
    }
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
    fn as_mut(&mut self) -> Option<&mut Self::ElementType> {
        self.as_mut()
    }
    fn insert(&mut self, value: Self::ElementType) -> &mut Self::ElementType {
        self.insert(value)
    }
    fn is_some(&self) -> bool {
        self.is_some()
    }
}

/// Helper to retrieve the inner type of a vector of Options.
type ElementOfOptionalVec<C> =
    <<C as std::ops::Index<usize>>::Output as OptionInterface>::ElementType;

/*
Methods
and_modify
insert_entry
key
or_default
or_insert
or_insert_with
or_insert_with_key
*/

mod vec_entry;
mod vec_option_entry;

#[cfg(test)]
mod test {
    // use super::prelude::*;

    #[test]
    fn test_type_alias() {
        use crate::ElementOfOptionalVec;
        type V = ElementOfOptionalVec<Vec<Option<u32>>>;
        let x: V = 3;
        let y: u32 = x;
        let _ = y;
    }
}
