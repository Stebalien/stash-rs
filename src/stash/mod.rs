use std::fmt;
use std::vec;
use std::iter;
use std::marker;
use std::ops;
use std::slice;
use std::mem;

mod entry;
use self::entry::Entry;
use index::Index;

pub struct Extend<'a, I, Ix>
    where I: Iterator,
          I::Item: 'a,
          Ix: Index + 'a
{
    iter: I,
    stash: &'a mut Stash<I::Item, Ix>,
}

impl<'a, I, Ix: Index> Drop for Extend<'a, I, Ix>
    where I: Iterator,
          I::Item: 'a
{
    fn drop(&mut self) {
        for _ in self {}
    }
}

impl<'a, I, Ix: Index> Iterator for Extend<'a, I, Ix>
    where I: Iterator,
          I::Item: 'a
{
    type Item = Ix;

    fn next(&mut self) -> Option<Ix> {
        self.iter.next().map(|v| self.stash.put(v))
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, I, Ix: Index> ExactSizeIterator for Extend<'a, I, Ix>
    where I: ExactSizeIterator,
          I::Item: 'a
{
}

impl<'a, I, Ix: Index> DoubleEndedIterator for Extend<'a, I, Ix>
    where I: DoubleEndedIterator,
          I::Item: 'a
{
    fn next_back(&mut self) -> Option<Ix> {
        self.iter.next_back().map(|v| self.stash.put(v))
    }
}

/// Iterator over the `(index, &value)` pairs.
pub struct Iter<'a, V: 'a, Ix: Index> {
    inner: iter::Enumerate<slice::Iter<'a, Entry<V>>>,
    len: usize,
    _marker: marker::PhantomData<fn() -> Ix>,
}

/// Iterator over the `(index, &mut value)` pairs.
pub struct IterMut<'a, V: 'a, Ix: Index> {
    inner: iter::Enumerate<slice::IterMut<'a, Entry<V>>>,
    len: usize,
    _marker: marker::PhantomData<fn() -> Ix>,
}

/// Iterator over the `(index, value)` pairs.
pub struct IntoIter<V, Ix: Index> {
    inner: iter::Enumerate<vec::IntoIter<Entry<V>>>,
    len: usize,
    _marker: marker::PhantomData<fn() -> Ix>,
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

impl_iter!(Values, (<'a, V>), &'a V, entry::value_ref, ());
impl_iter!(ValuesMut, (<'a, V>), &'a mut V, entry::value_mut, ());
impl_iter!(IntoValues, (<V>), V, entry::value, ());

impl_iter!(Iter, (<'a, V, Ix>), (Ix, &'a V), entry::value_index_ref, (where Ix: Index));
impl_iter!(IterMut, (<'a, V, Ix>), (Ix, &'a mut V), entry::value_index_mut, (where Ix: Index));
impl_iter!(IntoIter, (<V, Ix>), (Ix, V), entry::value_index, (where Ix: Index));

/// An `O(1)` amortized table that reuses keys.
///
/// # Guarantees and non-guarantees:
///
/// 1. `Stash` is deterministic and keys do not depend on the inserted values.
///    This means you can update two stashes in tandem and get the same keys
///    back. This could be useful for, e.g., primary/secondary replication.
/// 2. Keys will always be less than the maximum number of items that have ever
///    been present in the `Stash` at any single point in time. In other words,
///    if you never store more than `n` items in a `Stash`, the stash will only
///    assign keys less than `n`. You can take advantage of this guarantee to
///    truncate the key from a `usize` to some smaller type.
/// 3. Except the guarantees noted above, you can assume nothing about key
///    assignment or iteration order. They can change at any time.
///
/// An example use case is a file descriptor table.
#[derive(Clone)]
pub struct Stash<V, Ix = usize> {
    data: Vec<Entry<V>>,
    size: usize,
    next_free: usize,
    // add a phantom user of the Ix type to make sure an instance of Stash is bound to one
    // specific index type, separate calls to put and get can't use different index types.
    _marker: marker::PhantomData<fn(Ix) -> Ix>,
}

impl<V> Stash<V, usize> {
    /// Constructs a new, empty `Stash<V, usize>`.
    ///
    /// This is a convenience method. Use `Stash::default` for
    /// a constructor that is generic in the type of index used.
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

    /// Constructs a new, empty `Stash<V, usize>` with the specified capacity.
    ///
    /// This is a convenience method. Use `Stash::default` for
    /// a constructor that is generic in the type of index used. In that case
    /// you can call `reserve` on the newly created stash to specify the
    /// capacity you need.
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
    /// let mut stash = Stash::with_capacity(10);
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
            _marker: marker::PhantomData,
        }
    }
}

impl<V, Ix> Stash<V, Ix>
    where Ix: Index
{
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

    /// Get the index that would be returned from next call to `put`.
    ///
    /// # Panics
    ///
    /// Panics if the size of the `Stash<V, Ix>` would overflow the `Ix` index type.
    pub fn next_index(&self) -> Ix {
        Ix::from_usize(self.next_free)
    }

    /// Put a value into the stash.
    ///
    /// Returns the index at which this value was stored.
    ///
    /// # Panics
    ///
    /// Panics if the size of the `Stash<V, Ix>` would overflow the `Ix` index type.
    #[inline]
    pub fn put(&mut self, value: V) -> Ix {
        // create index first so the potential panic would happen before any modification
        let idx = Ix::from_usize(self.next_free);
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
                    _ => ::unreachable::unreachable(),
                }
            }
        };
        self.size += 1;
        idx
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

    /// Iterate over the items in this `Stash<V>`.
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

    /// Mutably iterate over the items in this `Stash<V>`.
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
    pub fn take(&mut self, index: Ix) -> Option<V> {
        let take_index = index.into_usize();
        match self.data.get_mut(take_index) {
            None => None,
            Some(entry) => match mem::replace(entry, Entry::Empty(self.next_free)) {
                Entry::Empty(free_slot) => {
                    *entry = Entry::Empty(free_slot);
                    None
                },
                Entry::Full(value) => {
                    self.next_free = take_index;
                    self.size -= 1;
                    Some(value)
                }
            }
        }
    }

    /// Take an item from a slot (if non empty) without bounds or empty checking.
    /// So use it very carefully!
    /// 
    /// This can be safely used as long as the user does not mutate
    /// `indices` from `put` and is sure not to have taken the value
    /// associated with the given `index`.
    #[inline]
    pub unsafe fn take_unchecked(&mut self, index: Ix) -> V {
        let take_index = index.into_usize();
        match mem::replace(self.data.get_unchecked_mut(take_index), Entry::Empty(self.next_free)) {
            Entry::Empty(_) => ::unreachable::unreachable(),
            Entry::Full(value) => {
                self.next_free = take_index;
                self.size -= 1;
                value
            }
        }
    }

    /// Get a reference to the value at `index`.
    #[inline]
    pub fn get(&self, index: Ix) -> Option<&V> {
        match self.data.get(index.into_usize()) {
            Some(&Entry::Full(ref v)) => Some(v),
            _ => None,
        }
    }

    /// Get a reference to the value at `index` without bounds or empty checking.
    /// So use it very carefully!
    /// 
    /// This can be safely used as long as the user does not mutate
    /// `indices` from `put` and is sure not to have taken the value
    /// associated with the given `index`.
    #[inline]
    pub unsafe fn get_unchecked(&self, index: Ix) -> &V {
        match self.data.get_unchecked(index.into_usize()) {
            &Entry::Full(ref v) => v,
            _ => ::unreachable::unreachable()
        }
    }

    /// Get a mutable reference to the value at `index`.
    #[inline]
    pub fn get_mut(&mut self, index: Ix) -> Option<&mut V> {
        match self.data.get_mut(index.into_usize()) {
            Some(&mut Entry::Full(ref mut v)) => Some(v),
            _ => None,
        }
    }

    /// Get a mutable reference to the value at `index` without bounds or empty checking.
    /// So use it very carefully!
    /// 
    /// This can be safely used as long as the user does not mutate
    /// `indices` from `put` and is sure not to have taken the value
    /// associated with the given `index`.
    #[inline]
    pub unsafe fn get_unchecked_mut(&mut self, index: Ix) -> &mut V {
        match self.data.get_unchecked_mut(index.into_usize()) {
            &mut Entry::Full(ref mut v) => v,
            _ => ::unreachable::unreachable()
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
            self.next_free = i;
            self.size -= 1;
            // Do this last, that way a panic just stops this half way through.
            *entry = Entry::Empty(self.next_free);
        }
        // We've already replaced every element with `Empty` so all destructors
        // are no-ops. Use `set_len` to avoid traversing the list twice.
        unsafe {
            self.data.set_len(0);
            self.next_free = 0;
        }
    }
}

impl<V, Ix: Index> IntoIterator for Stash<V, Ix> {
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

impl<'a, V, Ix: Index> IntoIterator for &'a Stash<V, Ix> {
    type Item = (Ix, &'a V);
    type IntoIter = Iter<'a, V, Ix>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, V, Ix: Index> IntoIterator for &'a mut Stash<V, Ix> {
    type Item = (Ix, &'a mut V);
    type IntoIter = IterMut<'a, V, Ix>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<V, Ix> fmt::Debug for Stash<V, Ix>
    where V: fmt::Debug,
          Ix: fmt::Debug + Index
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map().entries(self).finish()
    }
}

impl<'a, V, Ix: Index> ops::Index<Ix> for Stash<V, Ix> {
    type Output = V;
    #[inline]
    fn index(&self, index: Ix) -> &V {
        self.get(index).expect("index out of bounds")
    }
}

impl<'a, V, Ix: Index> ops::IndexMut<Ix> for Stash<V, Ix> {
    #[inline]
    fn index_mut(&mut self, index: Ix) -> &mut V {
        self.get_mut(index).expect("index out of bounds")
    }
}


impl<V, Ix: Index> Default for Stash<V, Ix> {
    #[inline]
    fn default() -> Self {
        Stash {
            data: Vec::new(),
            next_free: 0,
            size: 0,
            _marker: marker::PhantomData,
        }
    }
}
