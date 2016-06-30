#[derive(Clone)]
pub enum Entry<V> {
    Empty(usize /* next free index */),
    Full(V),
}

use self::Entry::*;

/// Take the value if it exists.
pub fn value<V>(entry: Entry<V>) -> Option<V> {
    match entry {
        Full(value) => Some(value),
        Empty(_) => None
    }
}

/// Get an optional reference to the value.
pub fn value_ref<V>(entry: &Entry<V>) -> Option<&V> {
    match entry {
        &Full(ref value) => Some(value),
        _ => None
    }
}

/// Get an optional mutable reference to the value.
pub fn value_mut<V>(entry: &mut Entry<V>) -> Option<&mut V> {
    match entry {
        &mut Full(ref mut value) => Some(value),
        _ => None
    }
}

pub fn value_index_ref<V>((i, entry): (usize, &Entry<V>)) -> Option<(usize, &V)> {
    match *entry {
        Full(ref value) => Some((i, value)),
        Empty(_) => None,
    }
}

pub fn value_index_mut<V>((i, entry): (usize, &mut Entry<V>)) -> Option<(usize, &mut V)> {
    match *entry {
        Full(ref mut value) => Some((i, value)),
        Empty(_) => None,
    }
}

pub fn value_index<V>((i, entry): (usize, Entry<V>)) -> Option<(usize, V)> {
    match entry {
        Full(value) => Some((i, value)),
        Empty(_) => None,
    }
}
