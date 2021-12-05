use std::sync::Mutex;
use std::collections::VecDeque;
use std::sync::Arc;

// Arc, Mutex and VecDeque are wrapped around a generic type T.
// T is required to be Send (a marker trait automatically implemented when
// it is safe to do so) because it denotes types that are safe to move between
// threads, which is the whole point of the ApiQueue.
// For this implementation, T is required to be Copy as well, for simplicity.

/// A generic work queue for work elements which can be trivially copied.
/// Any producer of work can add elements and any worker can consume them.
/// ApiQueue derives Clone so that it can be distributed among threads.
#[derive(Clone)]
struct ApiQueue<T: Send + Copy> {
    inner: Arc<Mutex<VecDeque<T>>>,
}

impl<T: Send + Copy> ApiQueue<T> {
    fn new() -> Self { 
        Self { inner: Arc::new(Mutex::new(VecDeque::new())) } 
    }

    fn get_random_number(&self) -> Option<T> {
        // Try to get a lock on the Mutex. If this fails, there is a 
        // problem with the mutex - it's poisoned, meaning that a thread that
        // held the mutex lock panicked before releasing it. There is no way
        // to guarantee that all its invariants are upheld, so we need to not
        // use it in that case.
        let maybe_queue = self.inner.lock();
        // A lot is going on here. self.inner is an Arc of Mutex. Arc can deref
        // into its internal type, so we can call the methods of that inner
        // type (Mutex) without dereferencing, so this is like 
        //      *(self.inner).lock() 
        // but doesn't look awful. Mutex::lock() returns a 
        // Result<MutexGuard<VecDeque<T>>>. 

        // Unwrapping with if let, we get a MutexGuard, which is an RAII guard
        // that unlocks the Mutex when it goes out of scope.
        if let Ok(mut queue) = maybe_queue {
            // Returns Some(item) or None if there are no more items.
            queue.pop_front()

            // The function has returned, so queue goes out of scope and the
            // mutex unlocks.
        } else {
            // There's a problem with the mutex.
            panic!("ApiQueue::get_work() tried to lock a poisoned mutex");
        }
    }

    fn add_random_number(&self, random_number: T) -> usize {
        // As above, try to get a lock on the mutex. 
        if let Ok(mut queue) = self.inner.lock() {
            // As above, we can use the MutexGuard<VecDeque<T>> to access
            // the internal VecDeque.
            queue.push_back(random_number);

            // Now return the length of the queue.
            queue.len()
        } else {
            panic!("ApiQueue::add_random_number() tried to lock a poisoned mutex");
        }
    }
}

/// FillFlag is not Clone because it should only exist in one place.
struct FillFlag {
    inner: Arc<Mutex<bool>>,
}

impl FillFlag {
    // This function will be used by the controller thread to tell the workers
    // when the tank of values is full.
    /// # Errors
    /// If the underlying mutex is poisoned this may return an error.
    fn set(&mut self, state: bool) -> Result<(), ()> {
        if let Ok(mut v) = self.inner.lock() {
            // The * (deref operator) means assigning to what's inside the
            // MutexGuard, not the guard itself (which would be silly)
            *v = state;
            Ok(())
        } else {
            Err(())
        }
    }

/// Create a new FillFlag that can be used to share a bool across a
/// number of threads.
fn new_fillflag(initial_state: bool) -> (FillFlag) {
    let state = Arc::new(Mutex::new(initial_state));
    let fill_flag = FillFlag { inner: state.clone() };

    return (fill_flag);
}


fn main() {
    let tank = ApiQueue::new();
}
