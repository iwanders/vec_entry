use vec_entry::VecOptionEntry;

#[test]
fn test_with_option_trait() {
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
