## Topics 

### std::cell

#### Shareable mutable containers.

- Allows shared reference to be mutable. By default one can't have two mutable reference, but through the concept of interior mutability; cell achieves shared mutable reference.
- There is no to get reference to what is stored inside a cell. which means through use of cell, one can change what is inside, one can copy what is inside, one can replace but can't get reference to what is inside.
- Use cell for types that are cheap to copy. 


# std::move

move converts any variables captured by reference or mutable reference to variables captured by value.

```Rust
let data = vec![1, 2, 3];
let closure = move || println!("captured {:?} by value", data);
```