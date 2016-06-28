extern crate stash;

use std::iter::IntoIterator;
use stash::*;

#[test]
fn iter() {
    let mut stash = Stash::new();
    stash.extend(0..2).count();
    {
        let mut iter = stash.iter();
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    {
        let mut iter = stash.iter_mut();
        assert_eq!(iter.next(), Some(&mut 0));
        let it = iter.next().unwrap();
        assert_eq!(it, &mut 1);
        *it = 2;
        assert_eq!(iter.next(), None);
    }

    {
        let mut iter = stash.into_iter();
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
