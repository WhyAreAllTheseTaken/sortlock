use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::{SortKey, SortGuard};

/// An exclusive that can be locked in order.
pub struct SortRwLock<T> {
    /// The internal mutex.
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

    /// Request to lock this lock for reading.
    pub fn read(&self) -> SortReadGuard<T> {
        SortReadGuard {
            lock: self
        }
    }
    
    /// Request to lock this lock for writing.
    pub fn write(&self) -> SortWriteGuard<T> {
        SortWriteGuard {
            lock: self
        }
    }
}

/// A read guard for a `SortRwLock`.
pub struct SortReadGuard<'l, T> {
    /// The lock this request references.
    lock: &'l SortRwLock<T>,
}

impl <'l, T> SortGuard for SortReadGuard<'l, T> {
    type Guard = RwLockReadGuard<'l, T>;

    fn sort_key(&self) -> SortKey {
        self.lock.key
    }

    fn lock(&self) -> Self::Guard {
        self.lock.mutex.read()
            .expect("Failed to lock mutex.")
    }
}

/// A write guard for a `SortRwLock`.
pub struct SortWriteGuard<'l, T> {
    /// The lock this request references.
    lock: &'l SortRwLock<T>,
}

impl <'l, T> SortGuard for SortWriteGuard<'l, T> {
    type Guard = RwLockWriteGuard<'l, T>;

    fn sort_key(&self) -> SortKey {
        self.lock.key
    }

    fn lock(&self) -> Self::Guard {
        self.lock.mutex.write()
            .expect("Failed to lock mutex.")
    }
}

#[cfg(test)]
mod tests {
    use std::{any::Any, sync::Arc, thread};

    use crate::{SortRwLock, SortLockGroup};

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

