# Module Systems

In our last section, functions, we had some functions, area_of and volume_of, at the bottom of `src/main.rs`. In this section let's modify the code by moving out the non-main functions to a new `src/lib.rs` file.

For this, we will need to reference 3 files `src/main.rs`, `src/lib.rs`, and `Cargo.toml`. 

In our last section, I gave some examples of how to improve the code, this was my final result and where we can pick up in this section. We moved the variables out and made them `const`, 
for the main we did the print all on one line using `/n` to add a new line, we also called the functions in the print line, and I also cleaned up the functions with general inputs. 

```rust
// src/main.rs

const WIDTH: i32 = 4;
const HEIGHT: i32 = 7;
const DEPTH: i32 = 10;

fn main() {

    println!("Area is {} \nVolume is {}", area_of(WIDTH, HEIGHT), volume_of(WIDTH, HEIGHT, DEPTH));

}

fn area_of(x: i32, y:i32) -> i32 {
  x * y
}

fn volume_of(x: i32, y: i32, z: i32) -> i32 {
    x * y * z
}
```

Now, let's move area_of and volume_of to a new file `src/lib.rs`.

## Example 1: How to Mod

- [ ] Star in you IDE, I'm using VScode, create a new file `src/lib.rs`

![image](https://github.com/jvick1/Rust_Intro/assets/32043066/90408e2b-b2fb-4e23-9b5d-496b9bab2182)

- [ ] Cut and paste the `area_of` and `volume_of` functions into `src/lib.rs`
- [ ] Make them public `pub` this will allow you to call them in `src/main.rs`.

![image](https://github.com/jvick1/Rust_Intro/assets/32043066/0b3027f0-d012-4474-8d9e-59f4ac0480c8)

- [ ] Navigate to your `Cargo.toml` file and find your project name (yours might be different than mine). Mine is "module".

![image](https://github.com/jvick1/Rust_Intro/assets/32043066/362434c1-1faf-48bb-8a7e-33abe5b1700e)

- [ ] Back in `src/main.rs` we'll now call `use` on our project name followed by `::` and the functions we want to read in.

![image](https://github.com/jvick1/Rust_Intro/assets/32043066/cadcbfde-135d-4bda-9267-761b3aeca8ac)

Now in the terminal type the following:

```shell
$ cargo run
```

## Final Result

Here are what the 3 files should look like once done. Note pay attention to the package name in the toml file this is what you `use`.

```rust
// src/main.rs
use module::{area_of,volume_of};

const WIDTH: i32 = 4;
const HEIGHT: i32 = 7;
const DEPTH: i32 = 10;

fn main() {

    println!("Area is {} \nVolume is {}", area_of(WIDTH, HEIGHT), volume_of(WIDTH, HEIGHT, DEPTH));

}
```

```rust
// src/lib.rs
pub fn area_of(x: i32, y:i32) -> i32 {
    x * y
  }
  
pub fn volume_of(x: i32, y: i32, z: i32) -> i32 {
    x * y * z
}
```

```toml
# Cargo.toml

[package]
name = "module"
version = "0.0.1"
# ...
```


