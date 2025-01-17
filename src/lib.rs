
/*
    I tried generalising over the K type, but the problem then is that it cannot be known how
    much the vector should be grown, because there's no way to convert it to an usize.

    So we're placing an extra constraint on the key that is must be an usize, then we know how far
    to grow.

    One of the initial commits has this over a generic key, with an optional we can still do that
    without resizing.
*/


mod prelude {
    pub use crate::vec_usize_entry::VecUsizeEntry;
}

mod vec_usize_entry {
    use std::ops::{Index, IndexMut};
    pub trait VecUsizeEntry<'a, C: 'a, V> where C: std::ops::IndexMut<usize>{
        fn entry(&mut self, key: usize) -> Entry<'_, C>;
    }

    pub enum Entry<'a, C: 'a + std::ops::IndexMut<usize>> {
        /// An occupied entry.
        Occupied(OccupiedEntry<'a, C>),

        /// A vacant entry.
        Vacant(VacantEntry<'a, C>),
    }
    impl<'a, C: 'a + std::ops::IndexMut<usize>> Entry<'a, C> {
        pub fn key(&self) -> &usize {
            match *self {
                Entry::Occupied(ref entry) => entry.key(),
                Entry::Vacant(ref entry) => entry.key(),
            }
        }
    }
    impl<'a, C:'a + std::ops::IndexMut<usize>> Entry<'a, C> where <C as Index<usize>>::Output: Default{
        pub fn or_default(mut self)  -> &'a mut <C as Index<usize>>::Output  {
            match self {
                Entry::Occupied(entry) => entry.into_mut(),
                Entry::Vacant(ref entry) => {
                    // entry.z.resize(self.key() + 1);
                    // entry.key(),
                    todo!()
                }
            }
        }
    }


    pub struct OccupiedEntry<'a, C: 'a + std::ops::IndexMut<usize>> {
        z: &'a mut C,
        key: usize,
    }
    impl<'a, C: 'a + std::ops::IndexMut<usize>> OccupiedEntry<'a, C> {
        pub fn key(&self) -> &usize {
            &self.key
        }
        pub fn into_mut(self) -> &'a mut <C as Index<usize>>::Output  {
            self.z.index_mut(self.key)
        }
    }

    pub struct VacantEntry<'a, C: 'a> {
        z: &'a mut C,
        key: usize,
    }
    impl<'a, C: 'a> VacantEntry<'a, C> {
        pub fn key(&self) -> &usize {
            &self.key
        }
    }

    impl<'a, V:'a > VecUsizeEntry<'a, Vec<V>, V> for Vec<V>  where Vec<V>: std::ops::IndexMut<usize>{
        fn entry(&mut self, key: usize) -> Entry<'_, Vec<V>> {
            if key < self.len() {
                // value must be occupied.
                Entry::Occupied(OccupiedEntry{z: self, key})
            } else {
                Entry::Vacant(VacantEntry{z: self, key})
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::prelude::*;

    #[test]
    fn test_grow() {
        let mut m = vec![0u8];
        let z = m.entry(1);
        assert_eq!(z.key(), &1);
        
        // let mut x = ;
        let v0 = m.entry(0).or_default();
        assert_eq!(v0, &0u8);
        *v0 = 5;
        assert_eq!(m[0], 5);
    }
}