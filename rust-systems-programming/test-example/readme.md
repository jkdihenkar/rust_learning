# Unit Tests

## Program and Unit Tests

Once can write the unit tests with following macro and asserts - 
```rust
#[test]
fn test_if_process_id_is_returned() {
    assert!(get_process_id() > 0);
}
```

Running Program and Unit tests:
```bash
❯❯❯ cargo run
   Compiling test-example v0.1.0 (/Users/jaydihenkar/work/rust_learning/rust-systems-programming/test-example)
    Finished dev [unoptimized + debuginfo] target(s) in 0.19s
     Running `target/debug/test-example`
Process ID :: 48450

❯❯❯ cargo test
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/main.rs (target/debug/deps/test_example-ded7a0399ed48ed4)

running 1 test
test test_if_process_id_is_returned ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

The example shown in the `main.rs` writes the test which are also compiled with the source code.

## cfg(test) and mod for tests restructure

In the `second.rs` example tests are written inside the tests module and has a macro `#[cfg(test)]` which tells the compiler to compile and run tests only with test command.

```bash
❯ cargo test --bin second-test-example
   Compiling test-example v0.1.0 (/Users/jaydihenkar/work/rust_learning/rust-systems-programming/test-example)
    Finished test [unoptimized + debuginfo] target(s) in 0.24s
     Running unittests src/second.rs (target/debug/deps/second_test_example-9c8c172d77eec382)

running 1 test
test tests::test_if_process_id_is_nonzero ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

❯ cargo run --bin second-test-example
   Compiling test-example v0.1.0 (/Users/jaydihenkar/work/rust_learning/rust-systems-programming/test-example)
    Finished dev [unoptimized + debuginfo] target(s) in 0.19s
     Running `target/debug/second-test-example`
Process ID :: 49375
```

# Integration Test

Example of running the integration test by re-writing the `process_lib` as lib instead of bin.
We can run the tests by only selecting the specific tests to be run.

Output - 
```bash
❯ cargo test -- integration_test1
...

running 1 test
test integration_test1 ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
   Doc-tests process_lib
...
...
```

Writing integration test in above example emulates how any user of the lib would include the crate and use it's public interfaces.

## Ignoring tests

Once can also ignore running tests by using the `#[ignore]` macro.
```rust
#[test]
#[ignore]
fn process_test1() {
    assert!(true);
}
```

Output - 
```
     Running tests/integration-test1.rs (target/debug/deps/integration_test1-f8a0830ec7c81c89)

running 2 tests
test process_test1 ... ignored
test integration_test1 ... ok
```

Once can use the `--ignored` flag to run only the ignored tests - 

```
❯ cargo test -- --ignored 
...
...
     Running tests/integration-test1.rs (target/debug/deps/integration_test1-f8a0830ec7c81c89)

running 1 test
test process_test1 ... ok

```

## Running tests in parallel

Example usage -
```
cargo test -- --test-threads=5
cargo test -- --test-threads=1
```

# Documenting the Project and DocTests

One can run `cargo doc --open` to open the documentation.

Once can also write the docs in MD files but unfortunately they're not natively supported by cargo yet and needs to be manually compiled to HTML via `rustdoc` - 
```
❯❯❯ rustdoc doc/itest.md
❯❯❯ ls -1 doc
itest.html
itest.md
```

## Doc Tests

One can write the doc tests as mentioned in the `lib/lib.rs` file.

Running the doc test - 
```
❯ cargo test --doc
   Compiling test-example v0.1.0 (/Users/jaydihenkar/work/rust_learning/rust-systems-programming/test-example)
    Finished test [unoptimized + debuginfo] target(s) in 0.19s
   Doc-tests process_lib

running 1 test
test lib/lib.rs - get_process_id (line 10) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.90s
```

One can also try failing the doc test and see how it shows errors on doc tests.

Also natively - `cargo test` also runs doc tests by default.

Output - 
```
     Running tests/integration-test1.rs (target/debug/deps/integration_test1-f8a0830ec7c81c89)

running 2 tests
test process_test1 ... ignored
test integration_test1 ... ok

test result: ok. 1 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests process_lib

running 1 test
test lib/lib.rs - get_process_id (line 10) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.28s
```

