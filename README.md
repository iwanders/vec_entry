# An 'entry' interface for Vec

The `std::collections::HashMap` container has a nice [entry](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.entry) method.

This provides two similar interfaces for a `Vec`, or rather any container that implements the `VecInterface` trait.

## VecOptionEntry

Properties:
- Requires the index key to be an `usize` (necessary to know the required size when resizing).
- An entry is `Vacant` if the value is none, or the index is beyond the size of the current container.
- The container is grown and populated with `None` on accessing a `Vacant` entry beyond the current size.
- The elements in the container must implement `OptionInterface`, which is implemented for `Option`.

This is useful if you can use `usize` keys and fields may be populated or empty and need to be iterated over
in-order later and a contiguous container is desired.


```rust
use crate::vec_option_entry::VecOptionEntry;
let mut m: Vec<Option<u32>> = vec![Some(3)];
let r = m.entry(2).or_insert(5);
assert_eq!(r, &5);
assert_eq!(m, vec![Some(3), None, Some(5)]);
```

## VecEntry

Properties:
- Requires the index key to be an `usize` (necessary to know the required size when resizing).
- An entry is only `Vacant` if beyond the current size of the container.
- The container is grown and populated with `Default` on accessing a `Vacant` entry beyond the current size.

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