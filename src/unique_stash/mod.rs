use std::fmt;
use std::vec;
use std::iter;
use std::str::FromStr;
use std::ops::{Index, IndexMut};
use std::slice;
use std::mem;
use std::error::Error;

use self::entry::{VerEntry, Entry};

mod entry;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TagParseError;

impl Error for TagParseError {
    fn description(&self) -> &str {
        "failed to parse tag"
    }
}

impl fmt::Display for TagParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}


/// A versioned index into a `UniqueStash`.
///
/// Can be converted to and from strings of the form `###/###` (no leading
/// zeros). Every tag has exactly one valid string representation.
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
impl FromStr for Tag {
    type Err = TagParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pieces = s.split('/').fuse();
        if let (Some(first), Some(second), None) = (pieces.next(), pieces.next(), pieces.next()) {
            // Make sure we only accept one form of tag.
            if (first.len() > 1 && first.as_bytes()[0] == b'0') || (second.len() > 1 && second.as_bytes()[0] == b'0') {
                return Err(TagParseError);
            }

            if let (Ok(index), Ok(version)) = (first.parse(), second.parse()) {
                return Ok(Tag {
                    idx: index,
                    ver: version
                })
            }
        }
        Err(TagParseError)
    }
}

/// The iterator produced by `Unique::extend`.
pub struct Extend<'a, I>
    where I: Iterator,
          I::Item: 'a
{
    iter: I,
    stash: &'a mut UniqueStash<I::Item>,
}

impl<'a, I> Drop for Extend<'a, I>
    where I: Iterator,
          I::Item: 'a
{
    fn drop(&mut self) {
        for _ in self {}
    }
}

impl<'a, I> Iterator for Extend<'a, I>
    where I: Iterator,
          I::Item: 'a
{
    type Item = Tag;

    fn next(&mut self) -> Option<Tag> {
        self.iter.next().map(|v| self.stash.put(v))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, I> ExactSizeIterator for Extend<'a, I>
    where I: ExactSizeIterator,
          I::Item: 'a
{}

impl<'a, I> DoubleEndedIterator for Extend<'a, I>
    where I: DoubleEndedIterator,
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

impl_iter!(Values, (<'a, V>), &'a V, entry::value_ref, ());
impl_iter!(ValuesMut, (<'a, V>), &'a mut V, entry::value_mut, ());
impl_iter!(IntoValues, (<V>), V, entry::value, ());

impl_iter!(Iter, (<'a, V>), (Tag, &'a V), entry::value_index_ref, ());
impl_iter!(IterMut, (<'a, V>), (Tag, &'a mut V), entry::value_index_mut, ());
impl_iter!(IntoIter, (<V>), (Tag, V), entry::value_index, ());

/// An `O(1)` amortized table that does not reuse keys.
///
/// Guarantee: No two calls to `put` on the same `UniqueStash` will ever return the same `Key`.
///
/// An example use case is a file descriptor table.
///
/// An example use case is a session table where expired session IDs should
/// never be re-used.
#[derive(Clone)]
pub struct UniqueStash<V> {
    data: Vec<VerEntry<V>>,
    size: usize,
    next_free: usize,
}

impl<V> UniqueStash<V> {
    /// Constructs a new, empty `UniqueStash<T>`.
    ///
    /// The stash will not allocate until elements are put onto it.
    ///
    /// # Examples
    ///
    /// ```
    /// use stash::UniqueStash;
    ///
    /// let mut stash: UniqueStash<i32> = UniqueStash::new();
    /// ```
    #[inline]
    pub fn new() -> Self {
        UniqueStash::with_capacity(0)
    }

    /// Constructs a new, empty `UniqueStash<T>` with the specified capacity.
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
    /// use stash::UniqueStash;
    ///
    /// let mut stash: UniqueStash<i32> = UniqueStash::with_capacity(10);
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
        UniqueStash {
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
    /// use stash::UniqueStash;
    ///
    /// let stash: UniqueStash<i32> = UniqueStash::with_capacity(10);
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
    /// use stash::UniqueStash;
    ///
    /// let mut stash = UniqueStash::new();
    /// assert_eq!(stash.len(), 0);
    /// stash.put("a");
    /// assert_eq!(stash.len(), 1);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.size
    }

    /// Reserves capacity for at least `additional` more elements to be put into
    /// the given `UniqueStash<T>`. The collection may reserve more space to avoid
    /// frequent reallocations.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity overflows `usize`.
    ///
    /// # Examples
    ///
    /// ```
    /// use stash::UniqueStash;
    ///
    /// let mut stash: UniqueStash<i32> = UniqueStash::new();
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
    /// be put into the given `UniqueStash<T>`. Does nothing if the capacity is already
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
    /// use stash::UniqueStash;
    ///
    /// let mut stash: UniqueStash<i32> = UniqueStash::new();
    /// let t1 = stash.put(1);
    /// stash.reserve_exact(10);
    /// assert!(stash.capacity() >= 11);
    /// ```
    pub fn reserve_exact(&mut self, additional: usize) {
        let extra_space = self.data.len() - self.len();
        if extra_space < additional {
            self.data.reserve_exact(additional - extra_space)
        }
    }

    /// Put a value into the stash.
    ///
    /// Returns the index at which this value was stored.
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
        Tag {
            idx: loc,
            ver: version,
        }
    }

    /// Put all items in the iterator into the stash.
    ///
    /// Returns an iterator over the indices where the items were inserted. The
    /// items are actually inserted as the Iterator is read. If the returned
    /// Iterator is dropped, the rest of the items will be inserted all at once.
    #[inline]
    pub fn extend<I>(&mut self, iter: I) -> Extend<I>
        where I: Iterator<Item = V>
    {
        let (lower, _) = iter.size_hint();
        self.reserve(lower);
        Extend {
            iter: iter,
            stash: self,
        }
    }

    /// Iterate over the items in this `UniqueStash<V>`.
    ///
    /// Returns an iterator that yields `(index, &value)` pairs.
    #[inline]
    pub fn iter(&self) -> Iter<V> {
        Iter {
            len: self.len(),
            inner: self.data.iter().enumerate(),
        }
    }

    /// Mutably iterate over the items in this `UniqueStash<V>`.
    ///
    /// Returns an iterator that yields `(index, &mut value)` pairs.
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<V> {
        IterMut {
            len: self.len(),
            inner: self.data.iter_mut().enumerate(),
        }
    }

    /// Iterate over the values in this `UniqueStash<V>` by reference.
    #[inline]
    pub fn values(&self) -> Values<V> {
        Values {
            len: self.len(),
            inner: self.data.iter(),
        }
    }

    /// Mutably iterate over the values in this `UniqueStash<V>` by reference.
    #[inline]
    pub fn values_mut(&mut self) -> ValuesMut<V> {
        ValuesMut {
            len: self.len(),
            inner: self.data.iter_mut(),
        }
    }

    /// Iterate over the values in this `UniqueStash<V>` by value.
    #[inline]
    pub fn into_values(self) -> IntoValues<V> {
        IntoValues {
            len: self.len(),
            inner: self.data.into_iter(),
        }
    }

    /// Check if this `UniqueStash<V>` is empty.
    ///
    /// Returns `true` if this `UniqueStash<V>` is empty.
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
                        Some(value)
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
            Some(&VerEntry { version, entry: Entry::Full(ref value) }) if version == index.ver => {
                Some(value)
            }
            _ => None,
        }
    }

    /// Get a mutable reference to the value at `index`.
    pub fn get_mut(&mut self, index: Tag) -> Option<&mut V> {
        match self.data.get_mut(index.idx) {
            Some(&mut VerEntry { version, entry: Entry::Full(ref mut value) }) if version ==
                                                                                  index.ver => {
                Some(value)
            }
            _ => None,
        }
    }

    /// Clear the UniqueStash.
    ///
    /// Note: This will not cause `Tag`s to be reused.
    pub fn clear(&mut self) {
        for (i, item) in self.data.iter_mut().enumerate() {
            // Skip if empty. We do it this way so that panics on drop don't
            // mess up the datastructure.
            if let Entry::Empty(_) = item.entry {
                continue;
            }
            item.version += 1;
            self.next_free = i;
            self.size -= 1;
            item.entry = Entry::Empty(self.next_free);
        }
    }
}

impl<V> IntoIterator for UniqueStash<V> {
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

impl<'a, V> IntoIterator for &'a UniqueStash<V> {
    type Item = (Tag, &'a V);
    type IntoIter = Iter<'a, V>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, V> IntoIterator for &'a mut UniqueStash<V> {
    type Item = (Tag, &'a mut V);
    type IntoIter = IterMut<'a, V>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}


impl<V> fmt::Debug for UniqueStash<V>
    where V: fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map().entries(self).finish()
    }
}

impl<'a, V> Index<Tag> for UniqueStash<V> {
    type Output = V;
    #[inline]
    fn index(&self, index: Tag) -> &V {
        self.get(index).expect("index out of bounds")
    }
}

impl<'a, V> IndexMut<Tag> for UniqueStash<V> {
    #[inline]
    fn index_mut(&mut self, index: Tag) -> &mut V {
        self.get_mut(index).expect("index out of bounds")
    }
}


impl<V> Default for UniqueStash<V> {
    #[inline]
    fn default() -> Self {
        UniqueStash::new()
    }
}
