use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_KEY: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SortKey(usize);

impl SortKey {
    /// Creates a new sort key.
    pub fn new() -> Self {
        Self(NEXT_KEY.fetch_add(1, Ordering::Relaxed))
    }
}

