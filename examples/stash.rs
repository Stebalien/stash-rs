extern crate stash;
use stash::Stash;

fn main() {
    let mut stash = Stash::new();
    let key1 = stash.put("foo");
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
