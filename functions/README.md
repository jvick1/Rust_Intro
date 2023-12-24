# Functions
## Area

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

## Volume

- [ ] Still, in `/src/main.rs` let's add a volume function (I would make this function outside of main() just like we did for area_of)
- [ ] Add a print line in main to show the volume (I would add this inside of main under the area println!)

```rust
fn volume(width: i32, height: i32, depth: i32) -> i32 {
    width * height * depth
}
```

```rust
println!("Volume is {}", volume(width, height, depth));
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

    println!("Volume is {}", volume(width, height, depth));

}

fn area_of(x: i32, y:i32) -> i32 {
  x * y
}

fn volume(width: i32, height: i32, depth: i32) -> i32 {
    width * height * depth
}

```
