use std::{fmt::{self, Debug, Display, Formatter}, sync::{Mutex, MutexGuard}};

use crate::{LockGroup, SortKey, SortableLock};

/// A sortable lock that ensures exclusive access to a resource. 
/// This is a sortable version of rust's `Mutex` type.
///
/// Locking looks a little different to `Mutex`, as this lock allows sorting with other locks
/// through the use of `lock_all`.
/// ```
/// use sortlock::{SortMutex, LockGroup};
///
/// let lock = SortMutex::new("some value");
///
/// let guard = lock.lock().lock_all();
/// println!("{}", *guard);
/// ```
///
/// With multiple locks this ensures that locks are always locked in the same order:
/// ```
/// use sortlock::{SortMutex, LockGroup};
///
/// let lock1 = SortMutex::new("some value");
/// let lock2 = SortMutex::new("some other value");
///
/// // Here lock1 is locked then lock2.
/// let (guard1, guard2) = (lock1.lock(), lock2.lock()).lock_all();
/// println!("{}", *guard1);
/// println!("{}", *guard2);
///
/// // Unlock so we can lock again.
/// drop(guard1);
/// drop(guard2);
///
/// // Despite the order change the same is true here.
/// let (guard2, guard1) = (lock2.lock(), lock1.lock()).lock_all();
/// println!("{}", *guard1);
/// println!("{}", *guard2);
/// ```
pub struct SortMutex<T> {
    /// The internal mutex.
    mutex: Mutex<T>,
    /// The sort key for this lock.
    key: SortKey,
}

impl <T> SortMutex<T> {
    /// Creates a new `SortLock`.
    ///
    /// - `value` - The value of the lock.
    pub fn new(value: T) -> Self {
        Self {
            mutex: Mutex::new(value),
            key: SortKey::new()
        }
    }

    /// Requests to lock this lock.
    /// This method returns a guard which can be used with `lock_all` to perform a sorted lock.
    ///
    /// # Panicking
    /// The guard will panic when locked if this lock becomes poisoned.
    pub fn lock(&self) -> SortMutexGuard<T> {
        SortMutexGuard {
            lock: self
        }
    }
}

impl <T: Debug> Debug for SortMutex<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.lock().lock_all().fmt(f)
    }
}

impl <T: Display> Display for SortMutex<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.lock().lock_all().fmt(f)
    }
}

impl <T: Default> Default for SortMutex<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

/// A guard for a `SortMutex`.
pub struct SortMutexGuard<'l, T> {
    /// The lock this request references.
    lock: &'l SortMutex<T>,
}

impl <'l, T> SortableLock for SortMutexGuard<'l, T> {
    type Guard = MutexGuard<'l, T>;

    fn sort_key(&self) -> SortKey {
        self.lock.key
    }

    fn lock_presorted(&self) -> Self::Guard {
        self.lock.mutex.lock()
            .expect("Failed to lock mutex: mutex is poisoned.")
    }
}

#[cfg(test)]
mod tests {
    use std::{any::Any, sync::Arc, thread};

    use crate::{LockGroup, SortMutex};

    #[test]
    fn test_lock2() {
        let lock1 = SortMutex::new(1);
        let lock2 = SortMutex::new(2);

        let (guard1, guard2) = (lock1.lock(), lock2.lock()).lock_all();

        println!("{} {}", guard1, guard2);
    }
    
    #[test]
    fn test_deadlock() -> Result<(), Box<dyn Any + Send + 'static>> {
        let lock1 = Arc::new(SortMutex::new(0));
        let lock2 = Arc::new(SortMutex::new(0));
        
        let lock1b = lock1.clone();
        let lock2b = lock2.clone();
        
        let lock1c = lock1.clone();
        let lock2c = lock2.clone();

        let count = 1000000;

        let thread1 = thread::spawn(move || {
            for _ in 0..count {
                let (mut guard1, mut guard2) = (lock1.lock(), lock2.lock()).lock_all();
               
                *guard1 += 1;
                *guard2 += 2;
            }
        });
        let thread2 = thread::spawn(move || {
            for _ in 0..count {
                let (mut guard2, mut guard1) = (lock2b.lock(), lock1b.lock()).lock_all();
               
                *guard1 += 1;
                *guard2 += 2;
            }
        });
        thread1.join()?;
        thread2.join()?;

        assert_eq!(2 * count, *lock1c.lock().lock_all());
        assert_eq!(4 * count, *lock2c.lock().lock_all());

        Ok(())
    }
}
