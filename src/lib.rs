//! A crate providing ordered locking. That is locks that will always lock in the same order
//! regardless of the order in which the methods are called. This reduces deadlocks caused by
//! cycles as one lock waits for another. In particular the following case cannot happen when
//! exclusively using lock sorting:
//!
//! 1. Thread A acquires `lock1`.
//! 2. Thread B acquires `lock2`.
//! 3. Thread A tries to acquire `lock2` but it is locked by thread B.
//! 4. Thread B tries to acquire `lock1` but it is locked by thread A.
//! 5. Neither thread can continue and will not unlock so a deadlock has occured.
//!
//! With lock sorting this cannot occur as locks are always locked in the same order for both
//! threads. This is done by first requesting to lock each lock. Then, by placing the locks in a
//! tuple and calling the `lock_all` method, the locks will be locked in the same order regardless
//! of their order in the tuple.
//!
//! To allow for sorted locking, this crates provides two new types of lock:
//! - `SortMuted` - A sorted version of `Mutex`.
//! - `SortRwLock` - A sorted version of `RwLock`.
//!
//! # Examples
//! ```
//! use sortlock::{SortMutex, LockGroup};
//!
//! let lock1 = SortMutex::new("some value");
//! let lock2 = SortMutex::new("some other value");
//!
//! // Here lock1 is locked then lock2.
//! let (guard1, guard2) = (lock1.lock(), lock2.lock()).lock_all();
//! println!("{}", *guard1);
//! println!("{}", *guard2);
//!
//! // Unlock so we can lock again.
//! drop(guard1);
//! drop(guard2);
//!
//! // Despite the order change the same is true here.
//! let (guard2, guard1) = (lock2.lock(), lock1.lock()).lock_all();
//! println!("{}", *guard1);
//! println!("{}", *guard2);
//! ```

mod mutex;
mod key;
mod rwlock;

pub use key::SortKey;
pub use mutex::{SortMutex, SortMutexGuard};
pub use rwlock::{SortRwLock, SortReadGuard, SortWriteGuard};

/// A lock that can be locked in a way that ensures that multiple locks are always locked in the
/// same order..
pub trait SortableLock {
    /// The type of the lock guard once locked.
    type Guard;

    /// Gets the sort key of the lock.
    fn sort_key(&self) -> SortKey;
    
    /// Lock this lock.
    ///
    /// This method assumes that lock sorting has already been done.
    /// `lock_all` from `LockGroup` should be used if you want to lock with sorting. 
    fn lock_presorted(&self) -> Self::Guard;
}

/// A group of values that can be locked.
pub trait LockGroup {
    /// The type of the group once locked.
    type Locked;

    /// Lock all items in the group.
    ///
    /// The locking order will be consistent regardless of the order of the locks within in this
    /// group.
    fn lock_all(self) -> Self::Locked;
}

impl <T: SortableLock> LockGroup for T {
    type Locked = T::Guard;

    fn lock_all(self) -> Self::Locked {
        self.lock_presorted()
    }
}

impl <T1: SortableLock, T2: SortableLock> LockGroup for (T1, T2) {
    type Locked = (T1::Guard, T2::Guard);

    fn lock_all(self) -> Self::Locked {
        let mut locks = [(0, self.0.sort_key()), (1, self.1.sort_key())];

        locks.sort_by_key(|(_, key)| *key);

        let mut guards = (None, None);

        for (i, _) in locks {
            match i {
                0 => guards.0 = Some(self.0.lock_presorted()),
                1 => guards.1 = Some(self.1.lock_presorted()),
                _ => unreachable!(),
            }
        }

        (guards.0.unwrap(), guards.1.unwrap())
    }
}

impl <T1: SortableLock, T2: SortableLock, T3: SortableLock> LockGroup for (T1, T2, T3) {
    type Locked = (T1::Guard, T2::Guard, T3::Guard);

    fn lock_all(self) -> Self::Locked {
        let mut locks = [(0, self.0.sort_key()), (1, self.1.sort_key()), (2, self.2.sort_key())];

        locks.sort_by_key(|(_, key)| *key);

        let mut guards = (None, None, None);

        for (i, _) in locks {
            match i {
                0 => guards.0 = Some(self.0.lock_presorted()),
                1 => guards.1 = Some(self.1.lock_presorted()),
                2 => guards.2 = Some(self.2.lock_presorted()),
                _ => unreachable!(),
            }
        }

        (guards.0.unwrap(), guards.1.unwrap(), guards.2.unwrap())
    }
}

impl <T1: SortableLock, T2: SortableLock, T3: SortableLock, T4: SortableLock> LockGroup for (T1, T2, T3, T4) {
    type Locked = (T1::Guard, T2::Guard, T3::Guard, T4::Guard);

    fn lock_all(self) -> Self::Locked {
        let mut locks = [
            (0, self.0.sort_key()),
            (1, self.1.sort_key()),
            (2, self.2.sort_key()),
            (3, self.3.sort_key())
        ];

        locks.sort_by_key(|(_, key)| *key);

        let mut guards = (None, None, None, None);

        for (i, _) in locks {
            match i {
                0 => guards.0 = Some(self.0.lock_presorted()),
                1 => guards.1 = Some(self.1.lock_presorted()),
                2 => guards.2 = Some(self.2.lock_presorted()),
                3 => guards.3 = Some(self.3.lock_presorted()),
                _ => unreachable!(),
            }
        }

        (
            guards.0.unwrap(),
            guards.1.unwrap(),
            guards.2.unwrap(),
            guards.3.unwrap()
        )
    }
}

impl <T1: SortableLock, T2: SortableLock, T3: SortableLock, T4: SortableLock, T5: SortableLock> LockGroup for (T1, T2, T3, T4, T5) {
    type Locked = (T1::Guard, T2::Guard, T3::Guard, T4::Guard, T5::Guard);

    fn lock_all(self) -> Self::Locked {
        let mut locks = [
            (0, self.0.sort_key()),
            (1, self.1.sort_key()),
            (2, self.2.sort_key()),
            (3, self.3.sort_key()),
            (4, self.4.sort_key()),
        ];

        locks.sort_by_key(|(_, key)| *key);

        let mut guards = (None, None, None, None, None);

        for (i, _) in locks {
            match i {
                0 => guards.0 = Some(self.0.lock_presorted()),
                1 => guards.1 = Some(self.1.lock_presorted()),
                2 => guards.2 = Some(self.2.lock_presorted()),
                3 => guards.3 = Some(self.3.lock_presorted()),
                4 => guards.4 = Some(self.4.lock_presorted()),
                _ => unreachable!(),
            }
        }

        (
            guards.0.unwrap(),
            guards.1.unwrap(),
            guards.2.unwrap(),
            guards.3.unwrap(),
            guards.4.unwrap()
        )
    }
}

