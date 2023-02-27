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
where
    T: From<usize> + Into<usize>,
{
    fn from_usize(idx: usize) -> Self {
        From::from(idx)
    }
    fn into_usize(self) -> usize {
        Into::into(self)
    }
}
