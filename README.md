# An 'entry' interface for Vec

The `std::collections::HashMap` container has a nice [entry](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.entry) method.
At some point I needed to assign values into a vector and just grow the vector with `Default::default` if it wasn't yet of that size.
Currently an entry is only considered vacant if it is beyond the size of the vector, the element in the container is required to be  `Default::default`, otherwise the container can't be grown. It also requires the key to be an `usize`, because otherwise we can't calculate how much to grow the vector.

## VecOptionEntry

This trait requires the `Vec` to hold an type that implements `OptionInterface`, if an entry is `None` it is considered unoccupied.

```rust
use crate::vec_option_entry::VecOptionEntry;
let mut m: Vec<Option<u32>> = vec![Some(3)];
let r = m.entry(2).or_insert(5);
assert_eq!(r, &5);
assert_eq!(m, vec![Some(3), None, Some(5)]);
```

## VecEntry

This is the version that I actually needed, one that just grows the `Vec` if it doesn't fit yet.

```rust
use crate::vec_entry::VecEntry;
let mut m: Vec<u32> = vec![];
let a = m.entry(1).or_default();
assert_eq!(a, &0);
*a = 1;
let b = m.entry(3).or_insert(5);
assert_eq!(b, &5);
assert_eq!(m, vec![0, 1, 0, 5]);
assert!(matches!(m.entry(2), vec_usize_entry::Entry::Occupied(_)));
assert!(matches!(m.entry(8), vec_usize_entry::Entry::Vacant(_)));
```


This works a bit awkward with `Vec<Option<V>>` types, which is why `VecOptionEntry` was created:
```rust
use crate::vec_entry::VecEntry;
let mut m: Vec<Option<u32>> = vec![Some(3)];
let r = m.entry(2).or_insert(Some(5));
assert_eq!(r, &Some(5));
assert_eq!(m, vec![Some(3), None, Some(5)]);
```






## License
License is MIT OR Apache-2.0.