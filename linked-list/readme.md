## LinkedList lib in rust
This is generated with `cargo new linked-list --lib`

### Recurive Types in Rust
As is mostly stack allocated, it needs to know the static size of a data structure. Simply put - rust always need to know the value of the type in memory.

Example for following code - 
```
enum Node {
    Empty,
    NonEmpty{item: u32, next: Node}
}
```

Rust Compiler throws an error - 
```
> cargo check
    Checking linked-list v0.1.0 (/Users/jaydihenkar/work/rust_learning/linked-list)
error[E0072]: recursive type `Node` has infinite size
 --> src/lib.rs:1:1
  |
1 | enum Node {
  | ^^^^^^^^^ recursive type has infinite size
2 |     Empty,
3 |     NonEmpty{item: u32, next: Node}
  |                               ---- recursive without indirection
  |
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to make `Node` representable
  |
3 |     NonEmpty{item: u32, next: Box<Node>}
  |                               ++++    +

For more information about this error, try `rustc --explain E0072`.
error: could not compile `linked-list` due to previous error
```

We need to use `Box`, which is a pointer and hence a well defined size.
