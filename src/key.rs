use std::sync::atomic::{AtomicUsize, Ordering};

/// The next sort key to use.
static NEXT_KEY: AtomicUsize = AtomicUsize::new(0);

/// A sort key for sorting locks.
///
/// This must be unique to each lock.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SortKey(usize);

impl SortKey {
    /// Creates a new unique sort key.
    pub fn new() -> Self {
        Self(NEXT_KEY.fetch_add(1, Ordering::Relaxed))
    }
}

