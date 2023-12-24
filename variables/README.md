# Variables

- [ ] Make a new project named `variables` using cargo
  - See "cargo help" if you forgot the command.

```shell
$ cargo new variables
```

- [ ] Open `Cargo.toml`
  - [ ] Change the version number to `2.3.4` and save the file.  Keep an eye out for that version number in cargo's output when you run it!
```toml
# Cargo.toml

[package]
name = "variables"
version = "2.3.4"
# ...
```

- [ ] In `src/main.rs` at the start of the `main()` function:
  - [ ] Declare the variable `missiles` and initialize it to `8`
  - [ ] Declare the variable `ready` and initialize it to `2`

```rust
// src/main.rs

fn main() {
    let missiles = 8;
    let ready = 2;
    println!("Firing {} of my {} missiles...", ready, missiles);
}

```

- [ ] Run your program using cargo

```shell
$ cargo run
```

 Some common errors you may hit:
  - Forgot to use `let` to bind a variable
  - Forgot a semicolon `;` at the end of a line

- [ ] After the `println!(...)`, subtract `ready` from `missiles` like this:
  - `missiles = missiles - ready;`
- [ ] Add a second `println!(...)` to the end:
  - `println!("{} missiles left", missiles);`
- [ ] Run your program again using cargo


```rust
const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    let mut missiles = STARTING_MISSILES;
    let ready = READY_AMOUNT;
    println!("Firing {} of my {} missiles...", ready, missiles);
    missiles = missiles - ready;
    println!("{} missiles left", missiles);
}
```

  - Did you run into an error about mutability? Go back and add `mut` at the right spot on the line where you declare `missiles`.
- [ ] Declare a constant named `STARTING_MISSILES` and set it to `8` (the type is `i32`).
- [ ] Declare a constant named `READY_AMOUNT` and set it to `2` (also `i32`).
- [ ] Use the constants to initialize `missiles` and `ready`
  - Where did you put the constants?  If you put them inside the `main()` function, try moving them up above `main()` to module scope! 
- [ ] Nice. Congratulate yourself on a job well done!  You are a Rust programmer now!
