use std::cell::UnsafeCell;

pub struct Cell<T> { 
    value: UnsafeCell<T>, 
}
// unsafe impl<T> Sync for Cell<T> {}

impl<T> Cell<T> {
    fn new(value:T ) -> Self {
        Cell {value: UnsafeCell::new(value)}
    }

    pub fn set(&self, value: T) {
        // SAFETY: no one else id concurrenlty mutating self.value because !sync is implemented.
        // SAFETY: we arent' invalidating any references because we never give any out.
        unsafe { *self.value.get()  = value };
    }

    pub fn get(&self) -> T where T: Copy {
        // SAFETY: No one is modifying this value because, only single thread can be mutated
        unsafe { *self.value.get() }
    }
}

#[cfg(test)]
mod test {
    use super::Cell;
    use std::{sync::Arc, thread};

    #[test]
    fn bad() {
        // two threads mutating the same reference at a same time.
        let x  = Arc::new(Cell::new([0; 1024]));

        let x1 = Arc::clone(&x);

        let a1 = thread::spawn(move || {
            x1.set([1;1024]);
        });

        let x2 = Arc::clone(&x);

        let a2 = thread::spawn(move || {
            x2.set([2; 1024]);
        });
        a1.join().unwrap();
        a2.join().unwrap();
    }

    #[test]
    fn bad2() {
        //why get() should always return a copy and not a reference.
        let x = Cell::new(String::from("hello"));
        //get 42
        let first = x.get();
        // now change X to empty vector
        x.set(String::new());
        x.set(String::from("world"));
        // what is first pointing  , if first was not copied and was merely holding a reference. 
        eprintln!("{}",first);
    }
}
