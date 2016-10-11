extern crate stash;
use stash::Stash;

#[test]
fn iter() {
    let mut stash = Stash::new();
    stash.extend(0..2).count();
    {
        let mut iter = stash.values();
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    {
        let mut iter = stash.values_mut();
        assert_eq!(iter.next(), Some(&mut 0));
        let it = iter.next().unwrap();
        assert_eq!(it, &mut 1);
        *it = 2;
        assert_eq!(iter.next(), None);
    }

    {
        let mut iter = stash.into_values();
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None)
    }
}

#[test]
fn get() {
    let mut stash = Stash::new();
    let indices: Vec<usize> = stash.extend(0usize..10).collect();
    for (i, t) in indices.iter().enumerate() {
        assert_eq!(stash[*t], i);
    }
    stash[indices[2]] = 1;
    assert_eq!(stash[indices[2]], 1);
}

#[test]
fn reset_zero() {
    let mut stash1 = Stash::new();
    for _ in 0..3 {
      stash1.put (());
    }
    stash1.clear();
    assert_eq!(stash1.len(), 0);
    let mut stash2 = Stash::new();
    for _ in 0..4 {
      assert_eq!(stash1.put (()), stash2.put (()));
    }
    for i in 0..4 {
      stash1.take (i);
    }
    for i in 0..4 {
      stash2.take (3-i);
    }
    assert_eq!(stash1.len(), 0);
    assert_eq!(stash2.len(), 0);
    let mut stash3 = Stash::new();
    for _ in 0..5 {
      let i = stash3.put (());
      assert_eq!(i, stash1.put(()));
      assert_eq!(i, stash2.put(()));
    }
}

