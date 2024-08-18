use core::sync::atomic::Ordering;

use portable_atomic::AtomicU64;

/// The next sort key to use.
static NEXT_KEY: AtomicU64 = AtomicU64::new(0);

/// A sort key for sorting locks.
/// This must be unique to each lock.
///
/// A unique key can be generated with `SortKey::new`.
/// ```
/// use sortlock::SortKey;
///
/// let key = SortKey::new();
/// let key2 = SortKey::new();
///
/// assert_ne!(key, key2);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SortKey(u64);

impl SortKey {
    /// Creates a new unique sort key.
    pub fn new() -> Self {
        Self(NEXT_KEY.fetch_add(1, Ordering::Relaxed))
    }
}

