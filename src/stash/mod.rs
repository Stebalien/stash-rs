use std::fmt;
use std::vec;
use std::iter;
use std::ops::{Index, IndexMut};
use std::slice;
use std::mem;

mod entry;
use self::entry::Entry;

pub struct Extend<'a, I>
    where I: Iterator,
          I::Item: 'a
{
    iter: I,
    stash: &'a mut Stash<I::Item>,
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
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        self.iter.next().map(|v| self.stash.put(v))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, I> ExactSizeIterator for Extend<'a, I>
    where I: ExactSizeIterator,
          I::Item: 'a
{
}

impl<'a, I> DoubleEndedIterator for Extend<'a, I>
    where I: DoubleEndedIterator,
          I::Item: 'a
{
    fn next_back(&mut self) -> Option<usize> {
        self.iter.next_back().map(|v| self.stash.put(v))
    }
}

/// Iterator over the `(index, &value)` pairs.
pub struct Iter<'a, V: 'a> {
    inner: iter::Enumerate<slice::Iter<'a, Entry<V>>>,
    len: usize,
}

/// Iterator over the `(index, &mut value)` pairs.
pub struct IterMut<'a, V: 'a> {
    inner: iter::Enumerate<slice::IterMut<'a, Entry<V>>>,
    len: usize,
}

/// Iterator over the `(index, value)` pairs.
pub struct IntoIter<V> {
    inner: iter::Enumerate<vec::IntoIter<Entry<V>>>,
    len: usize,
}

/// Iterator over references to the values in the stash.
pub struct Values<'a, V: 'a> {
    inner: slice::Iter<'a, Entry<V>>,
    len: usize,
}

/// Iterator over mutable references to the values in the stash.
pub struct ValuesMut<'a, V: 'a> {
    inner: slice::IterMut<'a, Entry<V>>,
    len: usize,
}

/// Iterator over values in the stash.
pub struct IntoValues<V> {
    inner: vec::IntoIter<Entry<V>>,
    len: usize,
}

impl_iter!(Values, (<'a, V>), &'a V, entry::value_ref);
impl_iter!(ValuesMut, (<'a, V>), &'a mut V, entry::value_mut);
impl_iter!(IntoValues, (<V>), V, entry::value);

impl_iter!(Iter, (<'a, V>), (usize, &'a V), entry::value_index_ref);

impl_iter!(IterMut, (<'a, V>), (usize, &'a mut V), entry::value_index_mut);
impl_iter!(IntoIter, (<V>), (usize, V), entry::value_index);

/// An `O(1)` amortized table that reuses keys.
///
/// An example use case is a file descriptor table.
#[derive(Clone)]
pub struct Stash<V> {
    data: Vec<Entry<V>>,
    size: usize,
    next_free: usize,
}

impl<V> Stash<V> {
    /// Constructs a new, empty `Stash<T>`.
    ///
    /// The stash will not allocate until elements are put onto it.
    ///
    /// # Examples
    ///
    /// ```
    /// use stash::Stash;
    ///
    /// let mut stash: Stash<i32> = Stash::new();
    /// ```
    #[inline]
    pub fn new() -> Self {
        Stash::with_capacity(0)
    }

    /// Constructs a new, empty `Stash<T>` with the specified capacity.
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
    /// use stash::Stash;
    ///
    /// let mut stash: Stash<i32> = Stash::with_capacity(10);
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
        Stash {
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
    /// use stash::Stash;
    ///
    /// let stash: Stash<i32> = Stash::with_capacity(10);
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
    /// use stash::Stash;
    ///
    /// let mut stash = Stash::new();
    /// assert_eq!(stash.len(), 0);
    /// stash.put("a");
    /// assert_eq!(stash.len(), 1);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.size
    }

    /// Reserves capacity for at least `additional` more elements to be put into
    /// the given `Stash<T>`. The collection may reserve more space to avoid
    /// frequent reallocations.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity overflows `usize`.
    ///
    /// # Examples
    ///
    /// ```
    /// use stash::Stash;
    ///
    /// let mut stash: Stash<i32> = Stash::new();
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
    /// be put into the given `Stash<T>`. Does nothing if the capacity is already
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
    /// use stash::Stash;
    ///
    /// let mut stash: Stash<i32> = Stash::new();
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
    ///
    /// *Panics* if the size of the `Stash<V>` would overflow `usize::MAX`.
    #[inline]
    pub fn put(&mut self, value: V) -> usize {
        let loc = self.next_free;
        debug_assert!(loc <= self.data.len());

        self.next_free = if self.next_free == self.data.len() {
            self.data.push(Entry::Full(value));
            self.next_free.checked_add(1).unwrap()
        } else {
            // Safe because we've recorded that it is safe.
            unsafe {
                match mem::replace(self.data.get_unchecked_mut(loc), Entry::Full(value)) {
                    Entry::Empty(next_free) => next_free,
                    _ => unreachable!(),
                }
            }
        };
        self.size += 1;
        loc
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

    /// Iterate over the items in this `Stash<V>`.
    ///
    /// Returns an iterator that yields `(index, &value)` pairs.
    #[inline]
    pub fn iter(&self) -> Iter<V> {
        Iter {
            len: self.len(),
            inner: self.data.iter().enumerate(),
        }
    }

    /// Mutably iterate over the items in this `Stash<V>`.
    ///
    /// Returns an iterator that yields `(index, &mut value)` pairs.
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<V> {
        IterMut {
            len: self.len(),
            inner: self.data.iter_mut().enumerate(),
        }
    }

    /// Iterate over the values in this `Stash<V>` by reference.
    #[inline]
    pub fn values(&self) -> Values<V> {
        Values {
            len: self.len(),
            inner: self.data.iter(),
        }
    }

    /// Mutably iterate over the values in this `Stash<V>` by reference.
    #[inline]
    pub fn values_mut(&mut self) -> ValuesMut<V> {
        ValuesMut {
            len: self.len(),
            inner: self.data.iter_mut(),
        }
    }

    /// Iterate over the values in this `Stash<V>` by value.
    #[inline]
    pub fn into_values(self) -> IntoValues<V> {
        IntoValues {
            len: self.len(),
            inner: self.data.into_iter(),
        }
    }

    /// Check if this `Stash<V>` is empty.
    ///
    /// Returns `true` if this `Stash<V>` is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Take an item from a slot (if non empty).
    pub fn take(&mut self, index: usize) -> Option<V> {
        if let Some(entry) = self.data.get_mut(index) {
            match mem::replace(entry, Entry::Empty(self.next_free)) {
                Entry::Empty(free_slot) => {
                    // Just put it back.
                    *entry = Entry::Empty(free_slot);
                }
                Entry::Full(value) => {
                    self.next_free = index;
                    self.size -= 1;
                    return Some(value);
                }
            }
        }
        None
    }

    /// Get a reference to the value at `index`.
    #[inline]
    pub fn get(&self, index: usize) -> Option<&V> {
        match self.data.get(index) {
            Some(&Entry::Full(ref v)) => Some(v),
            _ => None,
        }
    }

    /// Get a mutable reference to the value at `index`.
    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut V> {
        match self.data.get_mut(index) {
            Some(&mut Entry::Full(ref mut v)) => Some(v),
            _ => None,
        }
    }

    /// Clear the stash. Cleared stash will give the same keys as a
    /// new stash for subsequent puts.
    pub fn clear(&mut self) {
        // Do it this way so that nothing bad happens if a destructor panics.
        for (i, entry) in self.data.iter_mut().enumerate() {
            // Skip if empty.
            if let Entry::Empty(_) = *entry {
                continue;
            }
            // Drops *then* writes. If drop panics, nothing bad happens (we just
            // stop clearing.
            *entry = Entry::Empty(self.next_free);
            self.next_free = i;
            self.size -= 1;
        }
        self.data.clear();
        self.next_free = 0;
    }
}

impl<V> IntoIterator for Stash<V> {
    type Item = (usize, V);
    type IntoIter = IntoIter<V>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            len: self.len(),
            inner: self.data.into_iter().enumerate(),
        }
    }
}

impl<'a, V> IntoIterator for &'a Stash<V> {
    type Item = (usize, &'a V);
    type IntoIter = Iter<'a, V>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, V> IntoIterator for &'a mut Stash<V> {
    type Item = (usize, &'a mut V);
    type IntoIter = IterMut<'a, V>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}


impl<V> fmt::Debug for Stash<V>
    where V: fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map().entries(self).finish()
    }
}

impl<'a, V> Index<usize> for Stash<V> {
    type Output = V;
    #[inline]
    fn index(&self, index: usize) -> &V {
        self.get(index).expect("index out of bounds")
    }
}

impl<'a, V> IndexMut<usize> for Stash<V> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut V {
        self.get_mut(index).expect("index out of bounds")
    }
}


impl<V> Default for Stash<V> {
    #[inline]
    fn default() -> Self {
        Stash::new()
    }
}
