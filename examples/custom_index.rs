extern crate stash;

use std::u16;

use stash::Stash;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct SmallIndex(u16);

impl stash::Index for SmallIndex {
    fn from_usize(idx: usize) -> Self {
        if idx > u16::MAX as usize {
            panic!("index type overflowing!");
        }
        SmallIndex(idx as u16)
    }

    fn into_usize(self) -> usize {
        self.0 as usize
    }
}

fn main() {
    let mut stash = Stash::default();

    // the following type annotation makes the rust compiler understand what type of index
    // we want to use. Only needed once.
    let key1: SmallIndex = stash.put("foo");
    let key2 = stash.put("bar");
    let key3 = stash.put("baz");

    assert_eq!(stash[key1], "foo");
    assert_eq!(stash[key2], "bar");
    assert_eq!(stash[key3], "baz");
    assert_eq!(stash.len(), 3);

    assert_eq!(stash.take(key2), Some("bar"));
    assert_eq!(stash.len(), 2);

    let key4 = stash.put("bin");
    assert_eq!(stash.len(), 3);
    assert_eq!(stash[key4], "bin");
    let mut values: Vec<_> = stash.into_iter().map(|(_, v)| v).collect();
    values.sort();
    assert_eq!(values, vec!["baz", "bin", "foo"]);
}
