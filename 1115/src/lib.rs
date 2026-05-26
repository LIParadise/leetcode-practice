pub struct Solution;

use std::sync::{Condvar, Mutex};
struct FooBar {
    n: usize,
    mtx: Mutex<FooBarEnum>,
    cv: Condvar,
}

#[derive(Copy, Clone)]
enum FooBarEnum {
    Foo,
    Bar,
}
impl FooBarEnum {
    fn toggle(&mut self) {
        *self = match self {
            FooBarEnum::Bar => FooBarEnum::Foo,
            FooBarEnum::Foo => FooBarEnum::Bar,
        }
    }
}

impl FooBar {
    fn new(n: usize) -> Self {
        FooBar {
            n,
            mtx: Mutex::new(FooBarEnum::Foo),
            cv: Condvar::new(),
        }
    }

    fn foo<F>(&self, print_foo: F)
    where
        F: Fn(),
    {
        for _ in 0..self.n {
            let mut guard = self.mtx.lock().unwrap();
            while let FooBarEnum::Bar = &*guard {
                guard = self.cv.wait(guard).unwrap();
            }
            guard.toggle();
            print_foo();
            // Condvars are prone to deadlocks if not treated properly.
            //
            // Either the other thread had taken the lock and decided to wait on the condvar,
            // or the other thread haven't taken the lock.
            //
            // In both cases, since we know we're the only two threads,
            // notifying (even if that notification might got lost) one thread suffices.
            self.cv.notify_one();
        }
    }

    fn bar<F>(&self, print_bar: F)
    where
        F: Fn(),
    {
        for _ in 0..self.n {
            let mut guard = self.mtx.lock().unwrap();
            while let FooBarEnum::Foo = &*guard {
                guard = self.cv.wait(guard).unwrap();
            }
            guard.toggle();
            print_bar();
            self.cv.notify_one();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_soln() {
        todo!()
    }
}

macro_rules! lprintln {
    // Match when a format string and additional arguments are provided
    ($fmt:expr, $($arg:tt)*) => {{
        if cfg!(feature = "local_test") {
            println!($fmt, $($arg)*);
        }
    }};

    // Match when only a format string is provided
    ($fmt:expr) => {{
        if cfg!(feature = "local_test") {
            println!($fmt);
        }
    }};
}
