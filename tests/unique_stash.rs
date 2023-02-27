extern crate bincode;
extern crate stash;
use stash::{Tag, UniqueStash};

#[test]
fn string_conversions() {
    let mut stash = UniqueStash::new();
    let tag1 = stash.put(1);
    let tag2: Tag = tag1.to_string().parse().unwrap();
    assert_eq!(stash.take(tag2).unwrap(), 1);
}

#[test]
fn string_format() {
    assert!("909/990".parse::<Tag>().is_ok());
    assert!("0909/990".parse::<Tag>().is_err());
    assert!("0909/0990".parse::<Tag>().is_err());
    assert!("909/0990".parse::<Tag>().is_err());
    assert!("909 /0990".parse::<Tag>().is_err());
    assert!("909//0990".parse::<Tag>().is_err());
    assert!("909/0990/".parse::<Tag>().is_err());
    assert!("/909/0990/".parse::<Tag>().is_err());
    assert!("/909/0990".parse::<Tag>().is_err());
}

#[test]
fn iter() {
    let mut stash = UniqueStash::new();
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
    let mut stash = UniqueStash::new();
    let indices: Vec<Tag> = stash.extend(0usize..10).collect();
    for (i, t) in indices.iter().enumerate() {
        assert_eq!(stash[*t], i);
    }
    stash[indices[2]] = 1;
    assert_eq!(stash[indices[2]], 1);
}

#[test]
fn no_reuse() {
    let mut stash = UniqueStash::new();
    let tag1 = stash.put(1);
    assert_eq!(stash[tag1], 1);
    assert_eq!(stash.len(), 1);
    stash[tag1] = 2;
    assert_eq!(stash[tag1], 2);
    assert_eq!(stash.len(), 1);
    assert_eq!(stash.take(tag1), Some(2));
    assert_eq!(stash.len(), 0);
    assert_eq!(stash.take(tag1), None);
    let tag2 = stash.put(3);
    assert!(tag1 != tag2);
    assert_eq!(stash.get(tag1), None);
    assert_eq!(stash[tag2], 3);
    assert_eq!(stash.len(), 1);

    stash.clear();
    assert_eq!(stash.len(), 0);
    assert!(stash.is_empty());

    assert!(stash.take(tag1).is_none());
    assert!(stash.take(tag2).is_none());
    let tag3 = stash.put(4);
    assert!(tag3 != tag1);
    assert!(tag3 != tag2);
    assert!(stash.take(tag1).is_none());
    assert!(stash.take(tag2).is_none());
    assert_eq!(stash.len(), 1);
    assert_eq!(stash.take(tag3), Some(4));
    assert_eq!(stash.len(), 0);
}

#[cfg(feature = "serialization")]
#[test]
fn serialize_empty() {
    let mut stash1 = UniqueStash::new();
    for a in 0..10 {
        stash1.put(a);
    }
    stash1.clear();

    let bytes = bincode::serialize(&stash1).unwrap();
    let mut stash2: UniqueStash<i32> = bincode::deserialize(&bytes).unwrap();

    assert_eq!(stash1.len(), stash2.len());
    let vec1: Vec<_> = stash1.iter().collect();
    let vec2: Vec<_> = stash2.iter().collect();
    assert_eq!(vec1, vec2);

    //test basic operations on the restored stash.
    let i = stash2.put(42);
    assert_eq!(stash2.get(i), Some(&42));
}

#[cfg(feature = "serialization")]
#[test]
fn serialize_half() {
    let mut stash1 = UniqueStash::new();
    let mut tags = Vec::new();
    for a in 0..10 {
        tags.push(stash1.put(a));
    }

    stash1.take(tags[0]);
    stash1.take(tags[3]);
    stash1.take(tags[9]);

    let bytes = bincode::serialize(&stash1).unwrap();
    let mut stash2: UniqueStash<i32> = bincode::deserialize(&bytes).unwrap();

    assert_eq!(stash1.len(), stash2.len());
    let vec1: Vec<_> = stash1.iter().collect();
    let vec2: Vec<_> = stash2.iter().collect();
    assert_eq!(vec1, vec2);

    //test basic operations on the restored stash.
    let i = stash2.put(42);
    assert_eq!(stash2.get(i), Some(&42));
}

#[cfg(feature = "serialization")]
#[test]
fn serialize_full() {
    let mut stash1 = UniqueStash::new();
    for a in 0..10 {
        stash1.put(a);
    }

    let bytes = bincode::serialize(&stash1).unwrap();
    let mut stash2: UniqueStash<i32> = bincode::deserialize(&bytes).unwrap();

    assert_eq!(stash1.len(), stash2.len());
    let vec1: Vec<_> = stash1.iter().collect();
    let vec2: Vec<_> = stash2.iter().collect();
    assert_eq!(vec1, vec2);

    //test basic operations on the restored stash.
    let i = stash2.put(42);
    assert_eq!(stash2.get(i), Some(&42));
}
