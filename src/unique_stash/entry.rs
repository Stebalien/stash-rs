use crate::index::UniqueIndex;
use self::Entry::*;
use std::mem;

#[derive(Clone)]
pub enum Entry<V> {
    Full(V),
    Empty(usize),
}

// TODO: Use a union so we don't pay for a tag *and* a version...
#[derive(Clone)]
pub struct VerEntry<V> {
    pub version: u64,
    pub entry: Entry<V>,
}


pub fn new<V>(value: V) -> VerEntry<V> {
    VerEntry {
        version: 0,
        entry: Full(value),
    }
}

pub fn fill<V>(entry: &mut VerEntry<V>, value: V) -> usize {
    match mem::replace(&mut entry.entry, Full(value)) {
        Empty(next_free) => next_free,
        _ => panic!("expected no entry"),
    }
}

pub fn value_index_ref<V, Ix: UniqueIndex>((i, entry): (usize, &VerEntry<V>)) -> Option<(Ix, &V)> {
    let version = entry.version;
    match entry.entry {
        Full(ref value) => {
            Some((Ix::new(i,version), value))
        }
        Empty(_) => None,
    }
}

pub fn value_index_mut<V, Ix: UniqueIndex>((i, entry): (usize, &mut VerEntry<V>)) -> Option<(Ix, &mut V)> {
    let version = entry.version;
    match entry.entry {
        Full(ref mut value) => {
            Some((Ix::new(i, version), value))
        }
        Empty(_) => None,
    }
}

pub fn value_index<V, Ix: UniqueIndex>((i, entry): (usize, VerEntry<V>)) -> Option<(Ix, V)> {
    let version = entry.version;
    match entry.entry {
        Full(value) => {
            Some((Ix::new(i, version), value))
        }
        Empty(_) => None,
    }
}

pub fn value_ref<V>(entry: &VerEntry<V>) -> Option<&V> {
    match entry.entry {
        Full(ref value) => Some(value),
        Empty(_) => None,
    }
}

pub fn value_mut<V>(entry: &mut VerEntry<V>) -> Option<&mut V> {
    match entry.entry {
        Full(ref mut value) => Some(value),
        Empty(_) => None,
    }
}

pub fn value<V>(entry: VerEntry<V>) -> Option<V> {
    match entry.entry {
        Full(value) => Some(value),
        Empty(_) => None,
    }
}
