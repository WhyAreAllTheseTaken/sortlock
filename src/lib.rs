mod mutex;
mod key;
mod rwlock;

pub use key::SortKey;
pub use mutex::{SortMutex, SortMutexGuard};
pub use rwlock::{SortRwLock, SortReadGuard, SortWriteGuard};

/// A lock that can be locked by sorting.
pub trait SortGuard {
    /// The type of the lock once locked.
    type Guard;

    /// Gets the sort key of the lock.
    fn sort_key(&self) -> SortKey;
    
    /// Lock this lock.
    fn lock(&self) -> Self::Guard;
}

/// A group of values that can be locked.
pub trait SortLockGroup {
    /// The type of the group once locked.
    type Locked;

    /// Lock all items in the group.
    fn lock_all(self) -> Self::Locked;
}

impl <T: SortGuard> SortLockGroup for T {
    type Locked = T::Guard;

    fn lock_all(self) -> Self::Locked {
        self.lock()
    }
}

impl <T1: SortGuard, T2: SortGuard> SortLockGroup for (T1, T2) {
    type Locked = (T1::Guard, T2::Guard);

    fn lock_all(self) -> Self::Locked {
        let mut locks = [(0, self.0.sort_key()), (1, self.1.sort_key())];

        locks.sort_by_key(|(_, key)| *key);

        let mut guards = (None, None);

        for (i, _) in locks {
            match i {
                0 => guards.0 = Some(self.0.lock()),
                1 => guards.1 = Some(self.1.lock()),
                _ => unreachable!(),
            }
        }

        (guards.0.unwrap(), guards.1.unwrap())
    }
}

impl <T1: SortGuard, T2: SortGuard, T3: SortGuard> SortLockGroup for (T1, T2, T3) {
    type Locked = (T1::Guard, T2::Guard, T3::Guard);

    fn lock_all(self) -> Self::Locked {
        let mut locks = [(0, self.0.sort_key()), (1, self.1.sort_key()), (2, self.2.sort_key())];

        locks.sort_by_key(|(_, key)| *key);

        let mut guards = (None, None, None);

        for (i, _) in locks {
            match i {
                0 => guards.0 = Some(self.0.lock()),
                1 => guards.1 = Some(self.1.lock()),
                2 => guards.2 = Some(self.2.lock()),
                _ => unreachable!(),
            }
        }

        (guards.0.unwrap(), guards.1.unwrap(), guards.2.unwrap())
    }
}

impl <T1: SortGuard, T2: SortGuard, T3: SortGuard, T4: SortGuard> SortLockGroup for (T1, T2, T3, T4) {
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
                0 => guards.0 = Some(self.0.lock()),
                1 => guards.1 = Some(self.1.lock()),
                2 => guards.2 = Some(self.2.lock()),
                3 => guards.3 = Some(self.3.lock()),
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

impl <T1: SortGuard, T2: SortGuard, T3: SortGuard, T4: SortGuard, T5: SortGuard> SortLockGroup for (T1, T2, T3, T4, T5) {
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
                0 => guards.0 = Some(self.0.lock()),
                1 => guards.1 = Some(self.1.lock()),
                2 => guards.2 = Some(self.2.lock()),
                3 => guards.3 = Some(self.3.lock()),
                4 => guards.4 = Some(self.4.lock()),
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

