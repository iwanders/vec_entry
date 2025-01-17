


trait VecEntry<'a, K, C: 'a, V> where C: std::ops::IndexMut<K, Output=V>{
    fn entry(&mut self, key: K) -> Entry<'_, C>;
}

pub enum Entry<'a, C: 'a> {
    /// An occupied entry.
    Occupied(OccupiedEntry<'a, C>),

    /// A vacant entry.
    Vacant(VacantEntry<'a, C>),
}


pub struct OccupiedEntry<'a, C: 'a> {
    z: &'a mut C,
}

pub struct VacantEntry<'a, C: 'a> {
    z: &'a mut C,
}

impl<'a, K, V:'a > VecEntry<'a, K, Vec<V>, V> for Vec<V>  where Vec<V>: std::ops::IndexMut<K, Output=V>{
    fn entry(&mut self, key: K) -> Entry<'_, Vec<V>> {
        use std::ops::IndexMut;
        // let x = key.get_mut(self);
        // let _ = self.index_mut(&key);
        // let _ = key.index_mut(&self);
        Entry::Occupied(OccupiedEntry{z: self})
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_grow() {
        let mut m = vec![0u8];
        let z = m.entry(1);
    }
}