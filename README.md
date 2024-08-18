# Sort Lock

A crate providing ordered locking. That is locks that will always lock in the same order
regardless of the order in which the methods are called. This reduces deadlocks caused by
cycles as one lock waits for another. In particular the following case cannot happen when
exclusively using lock sorting:

1. Thread A acquires `lock1`.
2. Thread B acquires `lock2`.
3. Thread A tries to acquire `lock2` but it is locked by thread B.
4. Thread B tries to acquire `lock1` but it is locked by thread A.
5. Neither thread can continue and will not unlock so a deadlock has occured.

With lock sorting this cannot occur as locks are always locked in the same order for both
threads. This is done by first requesting to lock each lock. Then, by placing the locks in a
tuple and calling the `lock_all` method, the locks will be locked in the same order regardless
of their order in the tuple.

To allow for sorted locking, this crates provides two new types of lock:
- `SortMuted` - A sorted version of `Mutex`.
- `SortRwLock` - A sorted version of `RwLock`.

## Examples
```rust
use sortlock::{SortMutex, LockGroup};

let lock1 = SortMutex::new("some value");
let lock2 = SortMutex::new("some other value");

// Here lock1 is locked then lock2.
let (guard1, guard2) = (lock1.lock(), lock2.lock()).lock_all();
println!("{}", *guard1);
println!("{}", *guard2);

// Unlock so we can lock again.
drop(guard1);
drop(guard2);

// Despite the order change the same is true here.
let (guard2, guard1) = (lock2.lock(), lock1.lock()).lock_all();
println!("{}", *guard1);
println!("{}", *guard2);
```

## Feature Flags
To support `no-std` environments this crate can fall back to using `spin`'s `Mutex` and `RwLock` types. This can be done by disabiling the `std` feature.

