### Memory Terminology

- Values : doesn’t matter how it is stored in memory. It’s meaning will remain same
- Variables : a place to hold values
- Pointers : value that holds the address of a location that can hold a value

```rust
let x = 42;
let y = 43;
let var1 = &x;
let mut var2 = &x;
var2 = &y;
```

**Where distinction between values, variables and pointers become apparent**

```rust
let string = "Hello world";
```

TODO: create a memory representation after reading chapter 2

### Variables

- High Level : in terms of lifetimes and borrows
    - Don’t think them as places that hold bytes. Think of them as names given to values that are instantiated, moved and used throughout the program.
    - A variable can’t be uninstantiated.  When it is accessed, imagine drawing line from previous access to new access.
    - In terms of code
    
    ```rust
    let mut x;
    
    x = 42 // step 1
    let y = &x; //step 2 
    x = 43; // step 3
    assert_eq(*y, 42); //step 4 
    // parallel flow with mutable access to value. X and Y both have 43 as a value which
    // might not be intended effect.
    // Borrow checker catches it and throws error.
    ```
    

- Low Level: in terms of unsafe code and pointers.
    - Variables are thought as value slot. When one assigns to it, the slot is filled and its old value is dropped and replaced. `let x: usize` , the variable x is name of region of memory on stack that has room for usize’d value(i.e : if your computer is 64 bit then 64bits)
    - **Memory Regions**
        - **Stack**: Stack is segment of memory the program uses as scratch space for function call. Each time a function is called there is a chunk of memory called *frame* allocated at top of stack. the frame stores variables inside its scope. When you return a function allocated on stack, the memory region is acclaimed, Hence it is unsafe to access the variables of returned function. Rust doesn’t allow it, unless you explicitly define the lifetime of that memory segment.
        - **Heap**: Heap is pool of memory, that isn’t tied to stack. Values in heap live until they are explicitly deallocated with `free` . You can allocate something in heap on one function of x thread and later access it on another function of y thread without thinking about the lifetime of that allocation.  You use Heap with `Box::new()` in Rust. One has to free the values in Heap otherwise it will leak.
        - **Static Memory**: When you execute a program the program is loaded into memory.  the place where program is loaded is actually static memory. Values in static memory live for entire execution of program and static memory contains the program’s binary code. Static memory can hold variables you declare with `static` keyword, and constant values in code like `strings` . Note that `strings` aren’t always constants.
            - **Note:** `const` and `static` are different. `const` can be computed at compile time and since `const` doesn’t have any storage associated with it. the values of `const` doesn’t change. `static` on other hand refers to bunch of bytes in memory that might contain different value at different time, just that they live for entire execution for program.
            
            ### Ownership
            
            A *value* must have single *owner*. This means a value is stored in some part of memory once assigned to a variable and when it is time to deallocate, there must be only one *variable* that is capable of deallocating it.  
            
            ```rust
            let song = String::new("Humble"); // new string initialized
            let fav_song = song; // move occurs
            println!("{}",song); // song variable doesn't hold anything since the location 
            // it pointed to is now owned by fav_song. So doesn't compile
            ```
            
            ```rust
            error[E0382]: borrow of moved value: `song`
             --> ownership.rs:4:19
              |
            2 |     let song = String::from("Humble");
              |         ---- move occurs because `song` has type `String`, which does not implement the `Copy` trait
            3 |     let fav_song = song;
              |                    ---- value moved here
            4 |     println!("{}",song);
              |                   ^^^^ value borrowed here after move
              |
              = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
            ```
            
            There is a catch. If a value implements `Copy` *trait(*ability to duplicate the segment of memory so that the value is dup, then the value is not considered to be moved, even if it is reassigned to new memory location. All primitive types `u32`, `i32`, `f32` are usize. 
            
            ### Borrowing and Lifetimes
            
            References are pointers with additional contract on how they can be used. Reference can provide exclusive access to the value it references, or many other references can be created. 
            
            **Shared References :** *multiple reference for the same value might exist as it is read only.*
            
            **Mutable References :** 
            
            - *Exclusive access.*
            - *Main difference between owning a value and having a reference is that owner gets to drop the value but references don’t get it.*
            - When you want to move a value behind a mutable reference, you have to replace it with something. Example:
            
            ```rust
            	  let mut a = 12;
                let c = &mut a;
                // mem replace value of c
                let taken_data = std::mem::take(c);
                *c = 56;
            
            		// std::mem::swap() is also present
                println!("{}",a);   // 56
                println!("{}",taken_data); //12
            ```
            
            ### Interior Mutability
            
            Allows you to mutate a value through a shared reference. 
            
            - These types rely on additional mechanisms (like atomic CPU instructions) or invariants to provide safe mutability without relying on exclusive references.
                - Atomicity : Can’t be further broken down. Reading and writing of a native types with size equal to bus size of computer is atomic. Which means even if there is interrupt it won’t stop till it finishes its task.
                
                [Atomic Instruction](https://stackoverflow.com/questions/1762148/atomic-instruction)
                
                - Some let you get mutable reference through shared reference. `Mutex` and `RefCell`
                - Some let you replace a value though shared reference.
                
            
            ## Lifetimes
            
            ### Generic Lifetimes
            
            Type definition is generic over multiple lifetimes. Just like type definition is generic over multiple types. 
            
            ### Designing Interfaces
