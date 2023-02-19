## Basic Cargo and toml configs

When having multiple binary files configure in the cargo.toml - we need to specify which binary to run - 
```sh
> cargo run
error: `cargo run` could not determine which binary to run. Use the `--bin` option to specify a binary, or the `default-run` manifest key.
available binaries: new-first-program, second-program

> cargo run --bin second-program
   Compiling first-program v0.1.0 (/Users/jaydihenkar/work/rust_learning/rust-systems-programming/first-program)
    Finished dev [unoptimized + debuginfo] target(s) in 0.39s
     Running `target/debug/second-program`
Hello - Second source file -> bin second-program! :)

```
Or configure the defaults - 

```toml
[package]
name = "first-program"
version = "0.1.0"
edition = "2021"

default-run = "new-first-program"

[[bin]]
name = "new-first-program"
path = "src/main.rs"

[[bin]]
name = "second-program"
path = "src/second.rs"
```

then with defaults it runs `new-first-program`:
```
> cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/new-first-program`
Hello, world!
```

## Local Library Specifications

Can mention in cargo toml - 
```toml
[lib]
name = "some_library"
path = "lib/lib.rs"
```

and public function - 
```rust
pub fn hello_from_lib(message: &str) {
    println!("Print message from {} library...", message);
}
```

and can use it in binary with `use some_library::hello_from_lib;`.

## Version Specification in `Cargo.toml`

Mentioning version as - 
```toml
[dependencies]
chrono = "0.4.0"
```

Can use the latest minor version of the crate that is published. In this case it'd be `0.4.23` at the time.

Once can lock the minor version as follows - 
```toml
[dependencies]
chrono = ">= 0.4.22, < 0.4.23"
```

This will use the version `0.4.22` given the conditional requirements.

## gitignore for rust
https://raw.githubusercontent.com/github/gitignore/main/Rust.gitignore

