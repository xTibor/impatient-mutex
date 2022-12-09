use std::panic::Location;
use std::sync::{LockResult, Mutex, MutexGuard};
use std::thread;
use std::time::Duration;

/// Usage:
///
/// ```rust
/// use impatient_mutex::ImpatientMutex as Mutex;
/// ```
pub struct ImpatientMutex<T> {
    mutex: Mutex<T>,
    last_locker: Option<Location<'static>>,
}

impl<T> ImpatientMutex<T> {
    pub fn new(t: T) -> Self {
        Self {
            mutex: Mutex::new(t),
            last_locker: None,
        }
    }

    #[track_caller]
    pub fn lock(&self) -> LockResult<MutexGuard<'_, T>> {
        for _ in 0..=10 {
            if let Ok(result) = self.mutex.try_lock() {
                // Unsafe code for keeping .lock() function signatures the same
                unsafe {
                    let s = self as *const Self as *mut Self;
                    (*s).last_locker = Some(*Location::caller());
                }

                return Ok(result);
            }

            thread::sleep(Duration::from_millis(100));
        }

        if let Some(last_locker) = self.last_locker {
            panic!(
                "impatient locker ({}), last locker ({})",
                Location::caller(),
                last_locker
            );
        } else {
            unreachable!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ImpatientMutex;

    #[test]
    #[should_panic]
    fn test_01() {
        let mutex = ImpatientMutex::new("Nintendo 64");

        let oldest_sibling = mutex.lock();
        let youngest_sibling = mutex.lock();
    }
}
