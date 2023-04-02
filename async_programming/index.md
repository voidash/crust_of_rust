# Async Programming 

> Donot communicate by sharing memory but share memory by communicating. 

```rust
use std::sync::mpsc;
fn main() {
    let (tx,rx) = mpsc::channel();
}
```