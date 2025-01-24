use super::ElementOfOptionalVec;
use super::OptionInterface;
use super::VecInterface;
use std::ops::{Index, IndexMut};

/*
Methods

Implemented:
or_default
or_insert
and_modify
key
or_insert_with
or_insert_with_key

Doesn't make much sense:
insert_entry
*/

pub trait VecOptionEntry<'a, C: 'a>
where
    C: IndexMut<usize>,
{
    /// Gets an entry to the specified key in the Vec, does not modify the vec until action is taken.
    fn entry(&mut self, key: usize) -> Entry<'_, C>;
}

impl<'a, V: 'a> VecOptionEntry<'a, Vec<Option<V>>> for Vec<Option<V>> {
    #[inline]
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
    #[inline]
    pub fn key(&self) -> &usize {
        match *self {
            Entry::Occupied(ref entry) => entry.key(),
            Entry::Vacant(ref entry) => entry.key(),
        }
    }

    /// Insert
    #[inline]
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

    #[inline]
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

    #[inline]
    pub fn and_modify<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut ElementOfOptionalVec<C>),
        <C as Index<usize>>::Output: Sized,
        <C as Index<usize>>::Output: OptionInterface,
        <C as VecInterface>::ElementType: Default,
    {
        match self {
            Entry::Occupied(mut entry) => {
                // let mut x = entry;
                f(entry.get_mut());
                Entry::Occupied(entry)
            }
            Entry::Vacant(entry) => Entry::Vacant(entry),
        }
    }
}

impl<'a, C: 'a + std::ops::IndexMut<usize> + VecInterface> Entry<'a, C> {
    #[inline]
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
    #[inline]
    pub fn into_mut(self) -> &'a mut ElementOfOptionalVec<C> {
        self.z.index_mut(self.key).as_mut().unwrap()
    }

    #[inline]
    pub fn get_mut(&mut self) -> &mut ElementOfOptionalVec<C> {
        self.z.index_mut(self.key).as_mut().unwrap()
    }
}

impl<'a, C: 'a> OccupiedEntry<'a, C> {
    #[inline]
    pub fn key(&self) -> &usize {
        &self.key
    }
}

pub struct VacantEntry<'a, C: 'a> {
    z: &'a mut C,
    key: usize,
}

impl<'a, C: 'a> VacantEntry<'a, C> {
    #[inline]
    pub fn key(&self) -> &usize {
        &self.key
    }
}

impl<'a, C: 'a + std::ops::IndexMut<usize> + VecInterface> VacantEntry<'a, C> {
    #[inline]
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

#[cfg(test)]
mod test {

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

        let r = m.entry(0).or_default();
        assert_eq!(r, &3);
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

    #[test]
    fn test_with_option_trait_and_modify() {
        use crate::vec_option_entry::VecOptionEntry;
        let mut map: Vec<Option<u32>> = vec![Some(3)];

        map.entry(3).and_modify(|e| *e += 1).or_insert(42);
        assert_eq!(map[3], Some(42));

        map.entry(3).and_modify(|e| *e += 1).or_insert(42);
        assert_eq!(map[3], Some(43));
    }
}
