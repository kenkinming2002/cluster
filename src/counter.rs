use thread_local::ThreadLocal;

use std::cell::Cell;

use std::ops::Add;
use std::iter::Sum;

/// A multi-threaded counter.
#[derive(Default)]
pub struct Counter<T: Send>(ThreadLocal<Cell<T>>);

impl<T: Send> Counter<T> {
    /// Add value to a thread local counter initialized by `Default::default' if it does not exist.
    pub fn add<U>(&self, value : U)
    where
        T: Default,
        T: Add<U, Output = T>,
    {
        let total = self.0.get_or_default();
        total.set(total.take() + value);
    }

    /// Take and sum up values from all thread local counter leaving behind `Default::default()`.
    pub fn sum<S>(&mut self) -> S
    where
        T: Default,
        S: Sum<T>
    {
        self.0.iter_mut().map(|total| total.take()).sum()
    }
}
