# Intro

If you type `cargo new hello` on your machine with Rust installed this new project will be created. 

The dir will look somethinglike this:

hello/

- Cargo.toml

- src/main.rs

## Cargo.toml
In Rust, the Cargo.toml file is a configuration file used by Cargo, which is Rust's package manager and build system. 
The `Cargo.toml` file is an essential part of a Rust project, and it contains metadata about the project and its dependencies.

1. Project Information:

**[package] section:**
- name: The name of your project.
- version: The version number of your project.
- edition: The Rust edition your project is using (e.g., "2018").

2. Project Dependencies:

**[dependencies] section:**
- Lists the external crates (libraries) your project depends on.
- Each dependency is specified with the crate name and its version.

3. Build Configurations:

**[[bin]] section:**

- If your project contains multiple binaries, each can be specified here.
- Each [[bin]] section can have its own name and path.

**and more...**

## Main.rs
In Rust, the fn main() function is the entry point of an executable program. When you run a Rust program, the execution starts from the main function. Let's break down the code you provided:

`fn main() {println!("Hello, world!");}`

1. `fn main() { ... }`:
- This declares the main function. The main function is special in Rust; it serves as the starting point of execution for a Rust program.

2. `println!("Hello, world!");`:
- This line is using the println! macro to print the text "Hello, world!" to the console.
- The ! in println! indicates that it's a macro rather than a regular function. Macros in Rust are used for metaprogramming and code generation.

So, when you run a Rust program containing this main function, it will print "Hello, world!" to the console. The println! macro is commonly used for basic output during development and testing.
