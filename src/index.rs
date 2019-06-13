/// Every index type to be used with Stash needs to implement this trait
pub trait Index {
    /// Create an index from `usize`.
    ///
    /// This method should panic if `idx` is out of acceptable range.
    fn from_usize(idx: usize) -> Self;

    /// Turn this index into `usize`
    fn into_usize(self) -> usize;
}

// Auto implement this for types equivalent to `usize`.
impl<T> Index for T
    where T: From<usize> + Into<usize>
{
    fn from_usize(idx: usize) -> Self {
        From::from(idx)
    }
    fn into_usize(self) -> usize {
        Into::into(self)
    }
}

/// Every index type to be used with UniqueStash needs to implement this trait
pub trait UniqueIndex{
    /// Number of bits that this type uses to store index version.
    ///
    /// Must be equal to or less than 64.
    const VERSION_BITS: u8;

    /// Create a new UniqueIndex.
    ///
    /// This method should panic if `offset` is out of acceptable range.
    fn new_index(offset: usize, version: u64) -> Self;

    /// get the offset of this index
    fn offset(&self) -> usize;

    /// get the version of this index
    fn version(&self) -> u64;
}
