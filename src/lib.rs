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

pub mod vec_entry {
    use super::VecInterface;
    use std::ops::{Index, IndexMut};

    pub trait VecEntry<'a, C: 'a>
    where
        C: IndexMut<usize>,
    {
        /// Gets an entry to the specified key in the Vec, does not modify the vec until action is taken.
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

        /// Insert
        pub fn or_insert(
            self,
            default: <C as Index<usize>>::Output,
        ) -> &'a mut <C as Index<usize>>::Output
        where
            <C as Index<usize>>::Output: Sized,
            C: VecInterface,
            <C as VecInterface>::ElementType: Default,
        {
            match self {
                Entry::Occupied(entry) => entry.into_mut(),
                Entry::Vacant(entry) => entry.insert(default),
            }
        }
    }
    impl<'a, C: 'a + std::ops::IndexMut<usize> + VecInterface> Entry<'a, C>
    where
        <C as VecInterface>::ElementType: Default,
    {
        pub fn or_default(self) -> &'a mut <C as Index<usize>>::Output {
            match self {
                Entry::Occupied(entry) => entry.into_mut(),
                Entry::Vacant(entry) => {
                    entry.z.resize_with(entry.key() + 1, Default::default);
                    entry.z.index_mut(*entry.key())
                }
            }
        }
    }

    pub struct OccupiedEntry<'a, C: 'a> {
        z: &'a mut C,
        key: usize,
    }

    impl<'a, C: 'a> OccupiedEntry<'a, C> {
        pub fn key(&self) -> &usize {
            &self.key
        }
    }

    impl<'a, C: 'a + std::ops::IndexMut<usize>> OccupiedEntry<'a, C> {
        pub fn into_mut(self) -> &'a mut <C as Index<usize>>::Output {
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

    impl<'a, C: 'a + std::ops::IndexMut<usize> + VecInterface> VacantEntry<'a, C> {
        pub fn insert(
            self,
            value: <C as Index<usize>>::Output,
        ) -> &'a mut <C as Index<usize>>::Output
        where
            <C as Index<usize>>::Output: Sized,
            <C as VecInterface>::ElementType: Default,
        {
            self.z.resize_with(self.key() + 1, Default::default);
            let z = self.z.index_mut(*self.key());
            *z = value;
            z
        }
    }

    impl<'a, V: 'a> VecEntry<'a, Vec<V>> for Vec<V>
    where
        Vec<V>: std::ops::IndexMut<usize>,
    {
        fn entry(&mut self, key: usize) -> Entry<'_, Vec<V>> {
            if key < self.len() {
                // value must be occupied.
                Entry::Occupied(OccupiedEntry { z: self, key })
            } else {
                Entry::Vacant(VacantEntry { z: self, key })
            }
        }
    }
}

pub mod vec_option_entry {
    use super::ElementOfOptionalVec;
    use super::OptionInterface;
    use super::VecInterface;
    use std::ops::{Index, IndexMut};

    pub trait VecOptionEntry<'a, C: 'a>
    where
        C: IndexMut<usize>,
    {
        /// Gets an entry to the specified key in the Vec, does not modify the vec until action is taken.
        fn entry(&mut self, key: usize) -> Entry<'_, C>;
    }

    impl<'a, V: 'a> VecOptionEntry<'a, Vec<Option<V>>> for Vec<Option<V>> {
        fn entry(&mut self, key: usize) -> Entry<'_, Vec<Option<V>>> {
            if key < self.len() {
                // There is an option, but it still depends on whether it is none or not.
                if self[key].is_some() {
                    Entry::Occupied(OccupiedEntry { z: self, key })
                } else {
                    Entry::Vacant(VacantEntry { z: self, key })
                }
            } else {
                Entry::Vacant(VacantEntry { z: self, key })
            }
        }
    }

    pub enum Entry<'a, C: 'a + std::ops::IndexMut<usize>> {
        /// An occupied entry.
        Occupied(OccupiedEntry<'a, C>),

        /// A vacant entry.
        Vacant(VacantEntry<'a, C>),
    }

    impl<'a, C: 'a + std::ops::IndexMut<usize> + VecInterface> Entry<'a, C> {
        pub fn key(&self) -> &usize {
            match *self {
                Entry::Occupied(ref entry) => entry.key(),
                Entry::Vacant(ref entry) => entry.key(),
            }
        }

        /// Insert
        pub fn or_insert(self, value: ElementOfOptionalVec<C>) -> &'a mut ElementOfOptionalVec<C>
        where
            <C as Index<usize>>::Output: Sized,
            <C as Index<usize>>::Output: OptionInterface,
            <C as VecInterface>::ElementType: Default,
        {
            match self {
                Entry::Occupied(entry) => entry.into_mut(),
                Entry::Vacant(entry) => entry.insert(value),
            }
        }

        pub fn or_insert_with<F: FnOnce() -> ElementOfOptionalVec<C>>(
            self,
            f: F,
        ) -> &'a mut ElementOfOptionalVec<C>
        where
            <C as Index<usize>>::Output: Sized,
            <C as Index<usize>>::Output: OptionInterface,
            <C as VecInterface>::ElementType: Default,
        {
            match self {
                Entry::Occupied(entry) => entry.into_mut(),
                Entry::Vacant(entry) => entry.insert(f()),
            }
        }
    }

    impl<'a, C: 'a + std::ops::IndexMut<usize> + VecInterface> Entry<'a, C> {
        pub fn or_default(self) -> &'a mut ElementOfOptionalVec<C>
        where
            <C as Index<usize>>::Output: Sized,
            <C as Index<usize>>::Output: OptionInterface,
            <C as VecInterface>::ElementType: Default,
            <<C as Index<usize>>::Output as OptionInterface>::ElementType: Default,
        {
            match self {
                Entry::Occupied(entry) => entry.into_mut(),
                Entry::Vacant(entry) => entry.insert(Default::default()),
            }
        }
    }

    pub struct OccupiedEntry<'a, C: 'a> {
        z: &'a mut C,
        key: usize,
    }
    impl<'a, C: 'a + std::ops::IndexMut<usize> + VecInterface> OccupiedEntry<'a, C>
    where
        <C as Index<usize>>::Output: OptionInterface,
    {
        pub fn into_mut(self) -> &'a mut ElementOfOptionalVec<C> {
            self.z.index_mut(self.key).as_mut().unwrap()
        }
    }

    impl<'a, C: 'a> OccupiedEntry<'a, C> {
        pub fn key(&self) -> &usize {
            &self.key
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

    impl<'a, C: 'a + std::ops::IndexMut<usize> + VecInterface> VacantEntry<'a, C> {
        pub fn insert(self, value: ElementOfOptionalVec<C>) -> &'a mut ElementOfOptionalVec<C>
        where
            <C as Index<usize>>::Output: Sized,
            <C as Index<usize>>::Output: OptionInterface,
            <C as VecInterface>::ElementType: Default,
        {
            self.z
                .resize_with(self.z.len().max(self.key() + 1), Default::default);
            let z = self.z.index_mut(*self.key());
            z.insert(value);
            z.as_mut().unwrap()
        }
    }
}

#[cfg(test)]
mod test {
    // use super::prelude::*;
    use super::vec_entry;

    #[test]
    fn test_type_alias() {
        use crate::ElementOfOptionalVec;
        type V = ElementOfOptionalVec<Vec<Option<u32>>>;
        let x: V = 3;
        let y: u32 = x;
        let _ = y;
    }
    #[test]
    fn test_simple() {
        use crate::vec_entry::VecEntry;
        let mut m = vec![20u8];

        // No growth yet.
        let v0e = m.entry(0);
        assert!(matches!(v0e, vec_entry::Entry::Occupied(_)));
        let v0 = v0e.or_default();
        assert_eq!(v0, &20u8);
        *v0 = 0;
        assert_eq!(m[0], 0);
        assert_eq!(m.len(), 1);

        let z = m.entry(1);
        assert_eq!(z.key(), &1);
        assert!(matches!(z, vec_entry::Entry::Vacant(_)));
        let v1 = z.or_default();
        *v1 = 1;
        assert_eq!(m[1], 1);
    }

    #[test]
    fn test_with_example() {
        use crate::vec_entry::VecEntry;
        let mut m: Vec<u32> = vec![];
        let a = m.entry(1).or_default();
        assert_eq!(a, &0);
        *a = 1;
        let b = m.entry(3).or_insert(5);
        assert_eq!(b, &5);
        assert_eq!(m, vec![0, 1, 0, 5]);
        assert!(matches!(m.entry(2), vec_entry::Entry::Occupied(_)));
        assert!(matches!(m.entry(8), vec_entry::Entry::Vacant(_)));
    }

    #[test]
    fn test_with_optionals() {
        use crate::vec_entry::VecEntry;
        let mut m: Vec<Option<u32>> = vec![Some(3)];
        let r = m.entry(2).or_insert(Some(5));
        assert_eq!(r, &Some(5));
        assert_eq!(m, vec![Some(3), None, Some(5)]);
    }

    #[test]
    fn test_with_option_trait() {
        use crate::vec_option_entry::VecOptionEntry;
        let mut m: Vec<Option<u32>> = vec![Some(3)];
        let r = m.entry(2).or_insert(5);
        assert_eq!(r, &5);
        assert_eq!(m, vec![Some(3), None, Some(5)]);

        let r = m.entry(1).or_insert(1);
        assert_eq!(r, &1);
        assert_eq!(m.len(), 3);
        assert_eq!(m, vec![Some(3), Some(1), Some(5)]);

        let r = m.entry(4).or_default();
        assert_eq!(r, &0);
        assert_eq!(m, vec![Some(3), Some(1), Some(5), None, Some(0)]);
    }

    #[test]
    fn test_with_option_trait_insert_with() {
        use crate::vec_option_entry::VecOptionEntry;
        let mut m: Vec<Option<u32>> = vec![Some(3)];
        let r = m.entry(2).or_insert_with(|| 5);
        assert_eq!(r, &5);
        assert_eq!(m, vec![Some(3), None, Some(5)]);

        let r = m.entry(1).or_insert_with(|| 1);
        assert_eq!(r, &1);
        assert_eq!(m.len(), 3);
        assert_eq!(m, vec![Some(3), Some(1), Some(5)]);
    }
}
