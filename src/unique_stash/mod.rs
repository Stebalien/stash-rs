use std::fmt;
use std::marker;
use std::vec;
use std::iter;
use std::str::FromStr;
use std::ops::{Index, IndexMut};
use std::slice;
use std::mem;
use std::error::Error;

use self::entry::{VerEntry, Entry};
use crate::index::UniqueIndex;

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
/// Uses 64 bits for storing the version.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
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

impl UniqueIndex for Tag {
    const VERSION_BITS: u8 = 64;
    fn new(idx: usize, ver: u64) -> Self {
        Tag{idx, ver}
    }
    fn offset(&self) -> usize {
        self.idx
    }
    fn version(&self) -> u64 {
        self.ver
    }
}

/// The iterator produced by `Unique::extend`.
pub struct Extend<'a, I, Ix>
    where I: Iterator,
          I::Item: 'a,
          Ix: UniqueIndex
{
    iter: I,
    stash: &'a mut UniqueStash<I::Item, Ix>,
}

impl<'a, I, Ix> Drop for Extend<'a, I, Ix>
    where I: Iterator,
          I::Item: 'a,
          Ix: UniqueIndex
{
    fn drop(&mut self) {
        for _ in self {}
    }
}

impl<'a, I, Ix> Iterator for Extend<'a, I, Ix>
    where I: Iterator,
          I::Item: 'a,
          Ix: UniqueIndex
{
    type Item = Ix;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|v| self.stash.put(v))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, I, Ix> ExactSizeIterator for Extend<'a, I, Ix>
    where I: ExactSizeIterator,
          I::Item: 'a,
          Ix: UniqueIndex
{}

impl<'a, I, Ix> DoubleEndedIterator for Extend<'a, I, Ix>
    where I: DoubleEndedIterator,
          I::Item: 'a,
          Ix: UniqueIndex
{
    fn next_back(&mut self) -> Option<Ix> {
        self.iter.next_back().map(|v| self.stash.put(v))
    }
}

/// Iterator over the `(index, &value)` pairs.
pub struct Iter<'a, V: 'a, Ix> {
    inner: iter::Enumerate<slice::Iter<'a, VerEntry<V>>>,
    len: usize,
    _marker: marker::PhantomData<fn() -> Ix>,
}

/// Iterator over the `(index, &mut value)` pairs.
pub struct IterMut<'a, V: 'a, Ix> {
    inner: iter::Enumerate<slice::IterMut<'a, VerEntry<V>>>,
    len: usize,
    _marker: marker::PhantomData<fn() -> Ix>,
}

/// Iterator over the `(index, value)` pairs.
pub struct IntoIter<V, Ix> {
    inner: iter::Enumerate<vec::IntoIter<VerEntry<V>>>,
    len: usize,
    _marker: marker::PhantomData<fn() -> Ix>,
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

impl_iter!(Iter, (<'a, V, Ix>), (Ix, &'a V), entry::value_index_ref, (where Ix: UniqueIndex));
impl_iter!(IterMut, (<'a, V, Ix>), (Ix, &'a mut V), entry::value_index_mut, (where Ix: UniqueIndex));
impl_iter!(IntoIter, (<V, Ix>), (Ix, V), entry::value_index, (where Ix: UniqueIndex));

/// An `O(1)` amortized table that rarely reuses indices.
///
/// Indices store a version which makes it less likely that an old index (which data has
/// been removed) can be used, incorrectly, to retrieve new data stored at the same location.
/// How unlikely that is to happen depends on the type of index used. The default index type, `Tag`,
/// uses 64 bits for the version. That means you would need to insert and remove data from the
/// same location in the table more than 18 billion billion times before such a mishap is possible.
///
/// An example use case is a file descriptor table.
///
/// An example use case is a session table where expired session IDs should
/// never be re-used.
#[derive(Clone)]
pub struct UniqueStash<V, Ix=Tag> {
    data: Vec<VerEntry<V>>,
    size: usize,
    next_free: usize,
    // add a phantom user of the Ix type to make sure an instance of Stash is bound to one
    // specific index type, separate calls to put and get can't use different index types.
    _marker: marker::PhantomData<fn(Ix) -> Ix>,
}

impl<V> UniqueStash<V, Tag> {
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
            _marker: marker::PhantomData,
        }
    }
}

impl<V, Ix> UniqueStash<V, Ix>
where
    Ix: UniqueIndex,
{

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
    pub fn put(&mut self, value: V) -> Ix {
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
        Ix::new(loc, version)
    }

    /// Put all items in the iterator into the stash.
    ///
    /// Returns an iterator over the indices where the items were inserted. The
    /// items are actually inserted as the Iterator is read. If the returned
    /// Iterator is dropped, the rest of the items will be inserted all at once.
    #[inline]
    pub fn extend<I>(&mut self, iter: I) -> Extend<I, Ix>
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
    pub fn iter(&self) -> Iter<V, Ix> {
        Iter {
            len: self.len(),
            inner: self.data.iter().enumerate(),
             _marker: marker::PhantomData,
        }
    }

    /// Mutably iterate over the items in this `UniqueStash<V>`.
    ///
    /// Returns an iterator that yields `(index, &mut value)` pairs.
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<V, Ix> {
        IterMut {
            len: self.len(),
            inner: self.data.iter_mut().enumerate(),
             _marker: marker::PhantomData,
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
    pub fn take(&mut self, index: Ix) -> Option<V> {
        match self.data.get_mut(index.offset()) {
            Some(&mut VerEntry { ref mut version, ref mut entry }) if *version == index.version() => {
                match mem::replace(entry, Entry::Empty(self.next_free)) {
                    Entry::Full(value) => {
                        Self::incr_version(version);
                        self.next_free = index.offset();
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
    pub fn get(&self, index: Ix) -> Option<&V> {
        match self.data.get(index.offset()) {
            Some(&VerEntry { version, entry: Entry::Full(ref value) }) if version == index.version() => {
                Some(value)
            }
            _ => None,
        }
    }

    /// Get a mutable reference to the value at `index`.
    pub fn get_mut(&mut self, index: Ix) -> Option<&mut V> {
        match self.data.get_mut(index.offset()) {
            Some(&mut VerEntry { version, entry: Entry::Full(ref mut value) }) if version ==
                                                                                  index.version() => {
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
            Self::incr_version(&mut item.version);
            self.next_free = i;
            self.size -= 1;
            item.entry = Entry::Empty(self.next_free);
        }
    }

    #[inline(always)]
    fn incr_version(version: &mut u64) {
        let all_ones = (1u128 << Ix::VERSION_BITS) - 1;
        if *version == all_ones as u64 {
            *version = 0;
        } else {
            *version += 1;
        }
    }
}

impl<V, Ix: UniqueIndex> IntoIterator for UniqueStash<V, Ix> {
    type Item = (Ix, V);
    type IntoIter = IntoIter<V, Ix>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            len: self.len(),
            inner: self.data.into_iter().enumerate(),
             _marker: marker::PhantomData,
        }
    }
}

impl<'a, V, Ix: UniqueIndex> IntoIterator for &'a UniqueStash<V, Ix> {
    type Item = (Ix, &'a V);
    type IntoIter = Iter<'a, V, Ix>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, V, Ix: UniqueIndex> IntoIterator for &'a mut UniqueStash<V, Ix> {
    type Item = (Ix, &'a mut V);
    type IntoIter = IterMut<'a, V, Ix>;

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

impl<'a, V, Ix: UniqueIndex> Index<Ix> for UniqueStash<V, Ix> {
    type Output = V;
    #[inline]
    fn index(&self, index: Ix) -> &V {
        self.get(index).expect("index out of bounds")
    }
}

impl<'a, V, Ix: UniqueIndex> IndexMut<Ix> for UniqueStash<V, Ix> {
    #[inline]
    fn index_mut(&mut self, index: Ix) -> &mut V {
        self.get_mut(index).expect("index out of bounds")
    }
}


impl<V> Default for UniqueStash<V> {
    #[inline]
    fn default() -> Self {
        UniqueStash {
            data: Vec::with_capacity(0),
            next_free: 0,
            size: 0,
            _marker: marker::PhantomData,
        }
    }
}

#[cfg(feature = "serialization")]
mod serialization {
    use super::*;
    use std::marker;
    use serde::de::{ SeqAccess, Visitor, Deserialize, Deserializer };
    use serde::ser::{ SerializeSeq, Serialize, Serializer };

    impl<V> Serialize for UniqueStash<V>
        where
            V: Serialize,
    {
        fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
            let mut seq = serializer.serialize_seq(Some(self.data.len()))?;
            for ve in &self.data {
                let option = match ve.entry {
                    Entry::Full(ref v) => Some(v),
                    Entry::Empty(_) => None,
                };
                seq.serialize_element(&(ve.version, option))?;
            }
            seq.end()
        }
    }

    impl<'de, V> Deserialize<'de> for UniqueStash<V>
        where
            V: Deserialize<'de>,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
        {
            deserializer.deserialize_seq(StashVisitor::new())
        }
    }

    struct StashVisitor<V> {
        _marker: marker::PhantomData<fn(V) -> V>,
    }

    impl<V> StashVisitor<V> {
        fn new() -> StashVisitor<V> {
            StashVisitor {
                _marker: marker::PhantomData,
            }
        }
    }

    impl<'de, V> Visitor<'de> for StashVisitor<V>
        where
            V: Deserialize<'de>,
    {
        type Value = UniqueStash<V>;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "a sequence of optional values and versions")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
        {
            use std::usize;
            let initial_size = seq.size_hint().unwrap_or(8);
            let mut data = Vec::with_capacity(initial_size);
            let mut i = 0usize;
            let mut next_free = usize::MAX;
            let mut size = 0usize;
            let mut first_empty = usize::MAX;
            while let Some((version, option)) = seq.next_element()? {
                match option {
                    Some(v) => {
                        data.push(VerEntry{entry: Entry::Full(v), version});
                        size += 1;
                    }
                    None => {
                        if next_free == usize::MAX {
                            first_empty = i;
                        }
                        data.push(VerEntry{entry: Entry::Empty(next_free), version});
                        next_free = i;
                    }
                }
                i += 1;
            }
            // fix the last entry in linked list now that we know total length
            let final_length = data.len();
            if let Some(entry) = data.get_mut(first_empty) {
                if let VerEntry{entry: Entry::Empty(ref mut next), ..} = entry {
                    *next = final_length;
                }
            }
            Ok(UniqueStash {
                data,
                next_free,
                size,
            })
        }
    }
}
