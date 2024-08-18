use core::fmt::{self, Debug, Display, Formatter};

#[cfg(feature = "std")]
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
#[cfg(not(feature = "std"))]
use spin::{RwLock, RwLockWriteGuard, RwLockReadGuard};


use crate::{LockGroup, SortKey, SortableLock};

/// A sortable lock that allows either exclusive write access or shared read access. 
/// This is a sortable version of rust's `RwLock` type.
///
/// Locking looks a little different to `RwLock`, as this lock allows sorting with other locks
/// through the use of `lock_all`. Locking for reading can be performed with `read` while locking
/// for writing can be performed with `write`.
/// ```
/// use sortlock::{SortRwLock, LockGroup};
///
/// let lock = SortRwLock::new("some value");
///
/// let guard = lock.read().lock_all();
/// println!("{}", *guard);
/// ```
/// ```
/// use sortlock::{SortRwLock, LockGroup};
///
/// let lock = SortRwLock::new(1);
///
/// let mut guard = lock.write().lock_all();
/// *guard += 1;
/// assert_eq!(2, *guard);
/// ```
///
/// With multiple locks this ensures that locks are always locked in the same order.
/// This occurs regardless of whether the lock was locked for reading or writing.
/// ```
/// use sortlock::{SortRwLock, LockGroup};
///
/// let lock1 = SortRwLock::new(100);
/// let lock2 = SortRwLock::new(200);
///
/// // Here lock1 is locked then lock2.
/// let (guard1, mut guard2) = (lock1.read(), lock2.write()).lock_all();
/// println!("{}", *guard1);
/// *guard2 += 1;
///
/// // Unlock so we can lock again.
/// drop(guard1);
/// drop(guard2);
///
/// // Despite the order change the same is true here.
/// let (guard2, mut guard1) = (lock2.read(), lock1.write()).lock_all();
/// *guard1 += 1;
/// println!("{}", *guard2);
/// ```
pub struct SortRwLock<T> {
    /// The internal lock.
    mutex: RwLock<T>,
    /// The sort key for this lock.
    key: SortKey,
}

impl <T> SortRwLock<T> {
    /// Creates a new `SortRwLock`.
    ///
    /// - `value` - The value of the lock.
    pub fn new(value: T) -> Self {
        Self {
            mutex: RwLock::new(value),
            key: SortKey::new()
        }
    }

    /// Requests to lock this lock for reading.
    /// This method returns a guard which can be used with `lock_all` to perform a sorted lock.
    ///
    /// # Panicking
    /// The guard will panic when locked if this lock becomes poisoned.
    pub fn read(&self) -> SortReadGuard<T> {
        SortReadGuard {
            lock: self
        }
    }
    
    /// Requests to lock this lock for writing.
    /// This method returns a guard which can be used with `lock_all` to perform a sorted lock.
    ///
    /// # Panicking
    /// The guard will panic when locked if this lock becomes poisoned.
    pub fn write(&self) -> SortWriteGuard<T> {
        SortWriteGuard {
            lock: self
        }
    }
}

impl <T: Debug> Debug for SortRwLock<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.read().lock_all().fmt(f)
    }
}

impl <T: Display> Display for SortRwLock<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.read().lock_all().fmt(f)
    }
}

impl <T: Default> Default for SortRwLock<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

/// A read guard for a `SortRwLock`.
pub struct SortReadGuard<'l, T> {
    /// The lock this request references.
    lock: &'l SortRwLock<T>,
}

impl <'l, T> SortableLock for SortReadGuard<'l, T> {
    type Guard = RwLockReadGuard<'l, T>;

    fn sort_key(&self) -> SortKey {
        self.lock.key
    }

    #[cfg(feature = "std")]
    fn lock_presorted(&self) -> Self::Guard {
        self.lock.mutex.read()
            .expect("Failed to lock mutex.")
    }
    
    #[cfg(not(feature = "std"))]
    fn lock_presorted(&self) -> Self::Guard {
        self.lock.mutex.read()
    }
}

/// A write guard for a `SortRwLock`.
pub struct SortWriteGuard<'l, T> {
    /// The lock this request references.
    lock: &'l SortRwLock<T>,
}

impl <'l, T> SortableLock for SortWriteGuard<'l, T> {
    type Guard = RwLockWriteGuard<'l, T>;

    fn sort_key(&self) -> SortKey {
        self.lock.key
    }

    #[cfg(feature = "std")]
    fn lock_presorted(&self) -> Self::Guard {
        self.lock.mutex.write()
            .expect("Failed to lock mutex.")
    }
    
    #[cfg(not(feature = "std"))]
    fn lock_presorted(&self) -> Self::Guard {
        self.lock.mutex.write()
    }
}

#[cfg(test)]
mod tests {
    use std::{any::Any, sync::Arc, thread};

    use crate::{SortRwLock, LockGroup};

    #[test]
    fn test_lock2() {
        let lock1 = SortRwLock::new(1);
        let lock2 = SortRwLock::new(2);

        let (guard1, guard2) = (lock1.read(), lock2.write()).lock_all();

        println!("{} {}", guard1, guard2);
    }
    
    #[test]
    fn test_deadlock() -> Result<(), Box<dyn Any + Send + 'static>> {
        let lock1 = Arc::new(SortRwLock::new(0));
        let lock2 = Arc::new(SortRwLock::new(0));
        
        let lock1b = lock1.clone();
        let lock2b = lock2.clone();
        
        let lock1c = lock1.clone();
        let lock2c = lock2.clone();

        let count = 1000000;

        let thread1 = thread::spawn(move || {
            for _ in 0..count {
                let (mut guard1, guard2) = (lock1.write(), lock2.read()).lock_all();
               
                *guard1 += 1;

                drop(guard2);
            }
        });
        let thread2 = thread::spawn(move || {
            for _ in 0..count {
                let (mut guard2, guard1) = (lock2b.write(), lock1b.read()).lock_all();
               
                *guard2 += 1;
                
                drop(guard1);
            }
        });
        thread1.join()?;
        thread2.join()?;

        assert_eq!(count, *lock1c.read().lock_all());
        assert_eq!(count, *lock2c.read().lock_all());

        Ok(())
    }
}

