# Project #3: Functions
In this section, we will make area_of and volume_of functions. Functions can appear in any order, feel free to keep main at the top. Function parameters, 

```
fn do_stuff (qty: f64, oz: f64) -> f64 {
  return qty * oz;
}
``` 

This is how you read the code above function `fn`, name (snake case), multiple input separated by commas with type, a return type denoted by the `->`, and the body of the function is inside the `{}`. 
There is also a shorthand for the return statement it is called a tail expression and by dropping `return` and the `;` we can now just have  `qty * oz`. Let's try this in our next example!

For other Data Types visit https://doc.rust-lang.org/book/ch03-02-data-types.html.

## Area Example

- [ ] Make a new project named `functions` using cargo
  - See "cargo help" if you forgot the command.

```shell
$ cargo new functions
```

- [ ] In `/src/main.rs` let's update the main to include variables width, height, and depth
- [ ] Calculate the area for this we can make a new function and call it "area_of"
- [ ] Then print the area using println!

```rust
// src/main.rs
// Silence some warnings so they don't distract from the exercise.
#![allow(unused_variables)]

fn main() {
    let width = 4;
    let height = 7;
    let depth = 10;

    let area = area_of(width, height);

    println!("Area is {}", area);

}

fn area_of(x: i32, y:i32) -> i32 {
  x * y
}
```

- [ ] run it (make sure you are in the newly created project)

```shell
$ cargo run
```

Just to recap what we did... in main we defined height, width, and depth 
and then created a new function that takes in 2x i32 values and returns an i32 value. 
Note: return x * y is the same as x * y. "tail expression" that returns a value without using `return`.

Back in main we then call our newly defined area_of function with the two inputs of width and height. 
and lastly, we print the line just saying "Area is *BLANK" with blank being the return of area.

## Volume Example

- [ ] Still, in `/src/main.rs` let's add a volume function (I would make this function outside of main() just like we did for area_of)
- [ ] Add a print line in main to show the volume (I would add this inside of main under the area println!)

```rust
fn volume_of(width: i32, height: i32, depth: i32) -> i32 {
    width * height * depth
}
```

```rust
println!("Volume is {}", volume_of(width, height, depth));
```

## Final Result
Your final project should look something like this... 

3 functions in total; main, area_of, and volume. Main should call both functions and have a print statement for both.  

```rust
// src/main.rs
// Silence some warnings so they don't distract from the exercise.
#![allow(unused_variables)]

fn main() {
    let width = 4;
    let height = 7;
    let depth = 10;

    let area = area_of(width, height);

    println!("Area is {}", area);

    println!("Volume is {}", volume_of(width, height, depth));

}

fn area_of(x: i32, y:i32) -> i32 {
  x * y
}

fn volume_of(width: i32, height: i32, depth: i32) -> i32 {
    width * height * depth
}

```
There are a few enhancements that can be made... for example the `let area` line could be moved into the `println!` like it is in volume. The variables could also be moved to the top we'd probably just need to make them `const`. Even the print lines could be combined if you really wanted to. Lastly area_of and volume_of could be moved to a new lib.rs file and you could call them using the `project_name` (in Cargo.toml) and in main we'd just `call use project_name::{area_of, volume_of};`
