use std::fmt;
use std::vec;
use std::iter;
use std::ops::{Index, IndexMut};
use std::slice;
use std::mem;

/// A versioned index into a `VerStash`. A `VerStash` will never reuse a tag.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Tag {
    idx: usize,
    ver: u64,
}

impl fmt::Debug for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.idx, self.ver)
    }
}

mod entry {
    use super::Tag;
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

    pub fn entry_index_ref<V>((i, entry): (usize, &VerEntry<V>)) -> Option<(Tag, &V)> {
        let version = entry.version;
        match entry.entry {
            Full(ref value) => Some((Tag { idx: i, ver: version }, value)),
            Empty(_) => None,
        }
    }

    pub fn entry_index_mut<V>((i, entry): (usize, &mut VerEntry<V>)) -> Option<(Tag, &mut V)> {
        let version = entry.version;
        match entry.entry {
            Full(ref mut value) => Some((Tag { idx: i, ver: version }, value)),
            Empty(_) => None,
        }
    }

    pub fn entry_index<V>((i, entry): (usize, VerEntry<V>)) -> Option<(Tag, V)> {
        let version = entry.version;
        match entry.entry {
            Full(value) => Some((Tag { idx: i, ver: version }, value)),
            Empty(_) => None,
        }
    }

    pub fn entry_ref<V>(entry: &VerEntry<V>) -> Option<&V> {
        match entry.entry {
            Full(ref value) => Some(value),
            Empty(_) => None,
        }
    }

    pub fn entry_mut<V>(entry: &mut VerEntry<V>) -> Option<&mut V> {
        match entry.entry {
            Full(ref mut value) => Some(value),
            Empty(_) => None,
        }
    }

    pub fn entry<V>(entry: VerEntry<V>) -> Option<V> {
        match entry.entry {
            Full(value) => Some(value),
            Empty(_) => None,
        }
    }
}

use self::entry::{VerEntry, Entry};

pub struct ExtendIndices<'a, I> where I: Iterator, I::Item: 'a {
    iter: I,
    stash: &'a mut VerStash<I::Item>,
}

impl<'a, I> Drop for ExtendIndices<'a, I> where I: Iterator, I::Item: 'a {
    fn drop(&mut self) {
        for _ in self {}
    }
}

impl<'a, I> Iterator for ExtendIndices<'a, I> where I: Iterator, I::Item: 'a {
    type Item = Tag;

    fn next(&mut self) -> Option<Tag> {
        self.iter.next().map(|v| self.stash.put(v))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, I> ExactSizeIterator for ExtendIndices<'a, I> where
    I: ExactSizeIterator,
    I::Item: 'a
{ }

impl<'a, I> DoubleEndedIterator for ExtendIndices<'a, I> where
    I: DoubleEndedIterator,
    I::Item: 'a
{
    fn next_back(&mut self) -> Option<Tag> {
        self.iter.next_back().map(|v| self.stash.put(v))
    }
}

/// Iterator over the `(index, &value)` pairs.
pub struct Iter<'a, V: 'a> {
    inner: iter::Enumerate<slice::Iter<'a, VerEntry<V>>>,
    len: usize,
}

/// Iterator over the `(index, &mut value)` pairs.
pub struct IterMut<'a, V: 'a> {
    inner: iter::Enumerate<slice::IterMut<'a, VerEntry<V>>>,
    len: usize,
}

/// Iterator over the `(index, value)` pairs.
pub struct IntoIter<V> {
    inner: iter::Enumerate<vec::IntoIter<VerEntry<V>>>,
    len: usize,
}

/// Iterator over references to the values in the stash.
pub struct Values<'a, V: 'a> {
    inner: slice::Iter<'a, VerEntry<V>>,
    len: usize,
}

/// Iterator over mutable references to the values in the stash.
pub struct ValuesMut<'a, V: 'a> {
    inner: slice::IterMut<'a, VerEntry<V>>,
    len: usize,
}

/// Iterator over values in the stash.
pub struct IntoValues<V> {
    inner: vec::IntoIter<VerEntry<V>>,
    len: usize,
}

macro_rules! item_identity {
    ($it:item) => {
        $it
    }
}

macro_rules! impl_iter {
    ($name:ident, ($($tparm:tt)*), $item:ty, $fun:expr) => {
        item_identity! {
            impl $($tparm)* Iterator for $name $($tparm)* {
                type Item = $item;

                fn next(&mut self) -> Option<$item> {
                    let item = (&mut self.inner).filter_map($fun).next();
                    if item.is_some() {
                        self.len -= 1;
                    }
                    item
                }
                fn size_hint(&self) -> (usize, Option<usize>) {
                    (self.len, Some(self.len))
                }
            }
        }

        item_identity! {
            impl $($tparm)* ExactSizeIterator for $name $($tparm)* {
                fn len(&self) -> usize {
                    self.len
                }
            }
        }

        item_identity! {
            impl $($tparm)* DoubleEndedIterator for $name $($tparm)* {
                fn next_back(&mut self) -> Option<$item> {
                    let item = (&mut self.inner).rev().filter_map($fun).next();
                    if item.is_some() {
                        self.len -= 1;
                    }
                    item
                }
            }
        }
    }
}

impl_iter!(Values, (<'a, V>), &'a V, entry::entry_ref);
impl_iter!(ValuesMut, (<'a, V>), &'a mut V, entry::entry_mut);
impl_iter!(IntoValues, (<V>), V, entry::entry);

impl_iter!(Iter, (<'a, V>), (Tag, &'a V), entry::entry_index_ref);
impl_iter!(IterMut, (<'a, V>), (Tag, &'a mut V), entry::entry_index_mut);
impl_iter!(IntoIter, (<V>), (Tag, V), entry::entry_index);

/// A data structure storing values indexed by tickets.
#[derive(Clone)]
pub struct VerStash<V> {
    data: Vec<VerEntry<V>>,
    size: usize,
    next_free: usize,
}

impl<V> VerStash<V> {
    /// Constructs a new, empty `VerStash<T>`.
    ///
    /// The stash will not allocate until elements are put onto it.
    ///
    /// # Examples
    ///
    /// ```
    /// use stash::VerStash;
    ///
    /// let mut stash: VerStash<i32> = VerStash::new();
    /// ```
    #[inline]
    pub fn new() -> Self {
        VerStash::with_capacity(0)
    }

    /// Constructs a new, empty `VerStash<T>` with the specified capacity.
    ///
    /// The stash will be able to hold exactly `capacity` elements without
    /// reallocating. If `capacity` is 0, the stash will not allocate.
    ///
    /// It is important to note that this function does not specify the *length*
    /// of the returned stash , but only the *capacity*. (For an explanation of
    /// the difference between length and capacity, see the main `Vec<T>` docs
    /// in the `std::vec` module, 'Capacity and reallocation'.)
    ///
    /// # Examples
    ///
    /// ```
    /// use stash::VerStash;
    ///
    /// let mut stash: VerStash<i32> = VerStash::with_capacity(10);
    ///
    /// // The stash contains no items, even though it has capacity for more
    /// assert_eq!(stash.len(), 0);
    ///
    /// // These are all done without reallocating...
    /// for i in 0i32..10 {
    ///     let _ = stash.put(i);
    /// }
    ///
    /// // ...but this may make the stash reallocate
    /// stash.put(11);
    /// ```
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        VerStash {
            data: Vec::with_capacity(capacity),
            next_free: 0,
            size: 0,
        }
    }

    /// Returns the number of elements the stash can hold without reallocating.
    ///
    /// # Examples
    ///
    /// ```
    /// use stash::VerStash;
    ///
    /// let stash: VerStash<i32> = VerStash::with_capacity(10);
    /// assert_eq!(stash.capacity(), 10);
    /// ```
    #[inline]
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    /// The number of items in the stash.
    ///
    /// # Examples
    ///
    /// ```
    /// use stash::VerStash;
    ///
    /// let mut stash = VerStash::new();
    /// assert_eq!(stash.len(), 0);
    /// stash.put("a");
    /// assert_eq!(stash.len(), 1);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.size
    }

    /// Reserves capacity for at least `additional` more elements to be put into
    /// the given `VerStash<T>`. The collection may reserve more space to avoid
    /// frequent reallocations.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity overflows `usize`.
    ///
    /// # Examples
    ///
    /// ```
    /// use stash::VerStash;
    ///
    /// let mut stash: VerStash<i32> = VerStash::new();
    /// let t1 = stash.put(1);
    /// stash.reserve(10);
    /// assert!(stash.capacity() >= 11);
    /// ```
    pub fn reserve(&mut self, additional: usize) {
        let extra_space = self.data.len() - self.len();
        if extra_space < additional {
            self.data.reserve(additional - extra_space)
        }
    }

    /// Reserves the minimum capacity for exactly `additional` more elements to
    /// be put into the given `VerStash<T>`. Does nothing if the capacity is already
    /// sufficient.
    ///
    /// Note that the allocator may give the collection more space than it requests. Therefore
    /// capacity can not be relied upon to be precisely minimal. Prefer `reserve` if future
    /// puts are expected.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity overflows `usize`.
    ///
    /// # Examples
    ///
    /// ```
    /// use stash::VerStash;
    ///
    /// let mut stash: VerStash<i32> = VerStash::new();
    /// let t1 = stash.put(1);
    /// stash.reserve_exact(10);
    /// assert!(stash.capacity() >= 11);
    /// ```
    pub fn reserve_exact(&mut self, additional: usize){
        let extra_space = self.data.len() - self.len();
        if extra_space < additional {
            self.data.reserve_exact(additional - extra_space)
        }
    }

    /// Put a value into the stash.
    ///
    /// Returns the index at which this value was stored.
    ///
    /// *Panics* if the size of the `VerStash<V>` would overflow `usize::MAX`.
    pub fn put(&mut self, value: V) -> Tag {
        let loc = self.next_free;
        debug_assert!(loc <= self.data.len());

        let version;

        if self.next_free == self.data.len() {
            self.data.push(entry::new(value));
            version = 0;
            self.next_free += 1;
        } else {
            // Safe because we've recorded that it is safe.
            unsafe {
                let entry = self.data.get_unchecked_mut(loc);
                version = entry.version;
                self.next_free = entry::fill(entry, value);
            }
        }
        self.size += 1;
        Tag { idx: loc, ver: version }
    }

    /// Put all items in the iterator into the stash.
    ///
    /// Returns an iterator over the indices where the items were inserted. The
    /// items are actually inserted as the Iterator is read. If the returned
    /// Iterator is dropped, the rest of the items will be inserted all at once.
    #[inline]
    pub fn extend<I>(&mut self, iter: I) -> ExtendIndices<I> where I: Iterator<Item=V> {
        let (lower, _) = iter.size_hint();
        self.reserve(lower);
        ExtendIndices { iter: iter, stash: self }
    }

    /// Iterate over the items in this `VerStash<V>`.
    ///
    /// Returns an iterator that yields `(index, &value)` pairs.
    #[inline]
    pub fn iter<'a>(&'a self) -> Iter<'a, V> {
        Iter {
            len: self.len(),
            inner: self.data.iter().enumerate(),
        }
    }

    /// Mutably iterate over the items in this `VerStash<V>`.
    ///
    /// Returns an iterator that yields `(index, &mut value)` pairs.
    #[inline]
    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, V> {
        IterMut {
            len: self.len(),
            inner: self.data.iter_mut().enumerate(),
        }
    }

    /// Iterate over the values in this `VerStash<V>` by reference.
    #[inline]
    pub fn values<'a>(&'a self) -> Values<'a, V> {
        Values {
            len: self.len(),
            inner: self.data.iter(),
        }
    }

    /// Mutably iterate over the values in this `VerStash<V>` by reference.
    #[inline]
    pub fn values_mut<'a>(&'a mut self) -> ValuesMut<'a, V> {
        ValuesMut {
            len: self.len(),
            inner: self.data.iter_mut(),
        }
    }

    /// Iterate over the values in this `VerStash<V>` by value.
    #[inline]
    pub fn into_values(self) -> IntoValues<V> {
        IntoValues {
            len: self.len(),
            inner: self.data.into_iter(),
        }
    }

    /// Check if this `VerStash<V>` is empty.
    ///
    /// Returns `true` if this `VerStash<V>` is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Take an item from a slot (if non empty).
    pub fn take(&mut self, index: Tag) -> Option<V> {
        match self.data.get_mut(index.idx) {
            Some(&mut VerEntry { ref mut version, ref mut entry }) if *version == index.ver => {
                match mem::replace(entry, Entry::Empty(self.next_free)) {
                    Entry::Full(value) => {
                        // Don't bother checking. Won't overflow in any
                        // reasonable amount of time.
                        *version += 1;
                        self.next_free = index.idx;
                        self.size -= 1;
                        return Some(value);
                    }
                    empty => {
                        // Just put it back.
                        *entry = empty;
                        None
                    }
                }
            }
            _ => None,
        }
    }

    /// Get a reference to the value at `index`.
    pub fn get(&self, index: Tag) -> Option<&V> {
        match self.data.get(index.idx) {
            Some(&VerEntry { version, entry: Entry::Full(ref value) }) if version == index.ver => Some(value),
            _ => None,
        }
    }

    /// Get a mutable reference to the value at `index`.
    pub fn get_mut(&mut self, index: Tag) -> Option<&mut V> {
        match self.data.get_mut(index.idx) {
            Some(&mut VerEntry { version, entry: Entry::Full(ref mut value) }) if version == index.ver => Some(value),
            _ => None,
        }
    }
}

impl<V> IntoIterator for VerStash<V> {
    type Item = (Tag, V);
    type IntoIter = IntoIter<V>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            len: self.len(),
            inner: self.data.into_iter().enumerate(),
        }
    }
}

impl<'a, V> IntoIterator for &'a VerStash<V> {
    type Item = (Tag, &'a V);
    type IntoIter = Iter<'a, V>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, V> IntoIterator for &'a mut VerStash<V> {
    type Item = (Tag, &'a mut V);
    type IntoIter = IterMut<'a, V>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}


impl<V> fmt::Debug for VerStash<V> where V: fmt::Debug {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map().entries(self).finish()
    }
}

impl<'a, V> Index<Tag> for VerStash<V> {
    type Output = V;
    #[inline]
    fn index(&self, index: Tag) -> &V {
        self.get(index).expect("index out of bounds")
    }
}

impl<'a, V> IndexMut<Tag> for VerStash<V> {
    #[inline]
    fn index_mut(&mut self, index: Tag) -> &mut V {
        self.get_mut(index).expect("index out of bounds")
    }
}


impl<V> Default for VerStash<V> {
    #[inline]
    fn default() -> Self {
        VerStash::new()
    }
}
