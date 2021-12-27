use std::cell::UnsafeCell;

pub struct Cell<T> { 
    value: UnsafeCell<T>, 
}



impl<T> Cell<T> {
    fn new(value:T ) -> Self {
        Cell {value: UnsafeCell::new(value)}
    }

    pub fn set(&self, value: T) {
        unsafe { *self.value.get()  = value };
    }

    pub fn get(&self) -> T where T: Copy{
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

        let x  = Arc::new(Cell::new(42));

        let x1 = Arc::clone(&x);

        thread::spawn(move || {
            x1.set(43);
        });

        let x2 = Arc::clone(&x);

        thread::spawn(move || {
            x2.set(44);
        });
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
