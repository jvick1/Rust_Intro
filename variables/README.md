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
