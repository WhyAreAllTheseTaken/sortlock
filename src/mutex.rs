use std::sync::{Mutex, MutexGuard};

use crate::{SortKey, SortLock};

/// An exclusive that can be locked in order.
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

    /// Request to lock this lock.
    pub fn request(&self) -> SortMutexGuard<T> {
        SortMutexGuard {
            lock: self
        }
    }
}

/// A guard for a `SortMutex`.
pub struct SortMutexGuard<'l, T> {
    /// The lock this request references.
    lock: &'l SortMutex<T>,
}

impl <'l, T> SortLock for SortMutexGuard<'l, T> {
    type Guard = MutexGuard<'l, T>;

    fn sort_key(&self) -> SortKey {
        self.lock.key
    }

    fn lock(&self) -> Self::Guard {
        self.lock.mutex.lock()
            .expect("Failed to lock mutex.")
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

        let (guard1, guard2) = (lock1.request(), lock2.request()).lock_all();

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
                let (mut guard1, mut guard2) = (lock1.request(), lock2.request()).lock_all();
               
                *guard1 += 1;
                *guard2 += 2;
            }
        });
        let thread2 = thread::spawn(move || {
            for _ in 0..count {
                let (mut guard2, mut guard1) = (lock2b.request(), lock1b.request()).lock_all();
               
                *guard1 += 1;
                *guard2 += 2;
            }
        });
        thread1.join()?;
        thread2.join()?;

        assert_eq!(2 * count, *lock1c.request().lock_all());
        assert_eq!(4 * count, *lock2c.request().lock_all());

        Ok(())
    }
}
