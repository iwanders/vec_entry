
/*
    I tried generalising over the K type, but the problem then is that it cannot be known how
    much the vector should be grown, because there's no way to convert it to an usize.

    So we're placing an extra constraint on the key that is must be an usize, then we know how far
    to grow.

    One of the initial commits has this over a generic key, if the vector holds options, we can
    technically still support that, but we can't support resizing.
*/


pub mod prelude {
    pub use crate::vec_usize_entry::VecUsizeEntry;
}

pub mod vec_usize_entry {
    use std::ops::{Index, IndexMut};
    pub trait VecUsizeEntry<'a, C: 'a, V> where C: IndexMut<usize>{
        fn entry(&mut self, key: usize) -> Entry<'_, C>;
    }

    pub trait VecUsizeInterface {
        type ElementType;
        fn resize_with<F: FnMut() -> Self::ElementType>(&mut self, new_size: usize, f: F);
    }

    impl<T> VecUsizeInterface for Vec<T> {
        type ElementType=T;
        fn resize_with<F: FnMut() -> Self::ElementType>(&mut self, new_size: usize, f: F) {
            self.resize_with(new_size, f)
        }
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
    impl<'a, C:'a + std::ops::IndexMut<usize> + VecUsizeInterface> Entry<'a, C> where <C as VecUsizeInterface>::ElementType: Default {
        pub fn or_default(self)  -> &'a mut <C as Index<usize>>::Output  {
            match self {
                Entry::Occupied(entry) => entry.into_mut(),
                Entry::Vacant(entry) => { 
                    entry.z.resize_with(entry.key() + 1, Default::default);
                    entry.z.index_mut(*entry.key())
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
    fn test_simple() {
        let mut m = vec![20u8];

        // No growth yet.
        let v0e = m.entry(0);
        assert!(matches!(v0e, vec_usize_entry::Entry::Occupied(_)));
        let v0 = v0e.or_default();
        assert_eq!(v0, &20u8);
        *v0 = 0;
        assert_eq!(m[0], 0);
        assert_eq!(m.len(), 1);

        let z = m.entry(1);
        assert_eq!(z.key(), &1);
        assert!(matches!(z, vec_usize_entry::Entry::Vacant(_)));
        let v1 = z.or_default();
        *v1 = 1;
        assert_eq!(m[1], 1);


        
    }
}