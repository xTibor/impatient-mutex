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
}

impl<T> ImpatientMutex<T> {
    pub fn new(t: T) -> Self {
        Self {
            mutex: Mutex::new(t),
        }
    }

    pub fn lock(&self) -> LockResult<MutexGuard<'_, T>> {
        for _ in 0..=10 {
            if let Ok(result) = self.mutex.try_lock() {
                return Ok(result);
            }

            thread::sleep(Duration::from_millis(100));
        }

        panic!("impatient mutex");
    }
}

#[cfg(test)]
mod tests {
    use super::ImpatientMutex;

    #[test]
    fn test_01() {
        let mutex = ImpatientMutex::new(true);

        let x = mutex.lock();
        let y = mutex.lock();
    }
}
