
# Project #1: Introduction to Rust Project Structure

When you enter the command `cargo new hello` on a machine with Rust installed, it initiates the creation of a new project. The project directory will have the following structure:

hello/

- Cargo.toml

- src/main.rs

## Understanding `Cargo.toml`

In Rust, the `Cargo.toml` file is a crucial configuration file used by Cargo, Rust's package manager and build system. This file contains metadata about the project and its dependencies. Here's a breakdown of the sections within `Cargo.toml`:

1. **Project Information - `[package]` section:**
   - `name`: The name of your project.
   - `version`: The version number of your project.
   - `edition`: Specifies the Rust edition your project is using (e.g., "2018").

2. **Project Dependencies - `[dependencies]` section:**
   - Lists external crates (libraries) your project depends on.
   - Each dependency is specified with the crate name and its version.

3. **Build Configurations - `[[bin]]` section:**
   - If your project contains multiple binaries, each can be specified here.
   - Each `[[bin]]` section can have its own name and path.

...and more.

## Understanding `main.rs`

In Rust, the `fn main()` function serves as the entry point of an executable program. Here's a breakdown of the provided code:

```rust
fn main() {
    println!("Hello, world!");
}
```

1. `fn main() { ... }`:
- Declares the main function, which is the starting point of execution for a Rust program.

2. `println!("Hello, world!");`:
- This line is using the `println!` macro to print the text "Hello, world!" to the console.
- The `!` in `println!` indicates that it's a macro rather than a regular function. Macros in Rust are used for metaprogramming and code generation.

When you run a Rust program containing this main function, it prints "Hello, world!" to the console. The println! macro is commonly used for basic output during development and testing. Understanding these fundamental structures is key to building more complex and sophisticated Rust applications.
