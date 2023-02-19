# Docs for integ-test-example crate

This is a project to test `rustdoc`.

[Here is a link!](https://www.rust-lang.org)

// Function signature
pub fn get_process_id() -> u32 {}

// Example
```rust
use integ_test_example;
fn get_id() -> i32 {
let my_pid = get_process_id();
println!("Process id for current process is: {}", my_pid);
}
```