# Rust_Intro
How to install Rust and set up your first few projects.

## Download Rust
Navigate to https://www.rust-lang.org/tools/install and install Rust. 
![image](https://github.com/jvick1/Rust_Intro/assets/32043066/f51906c7-9dfe-4698-ade1-d020feea1226)

## Setup Rust
The download file is an exe. Once downloaded a new terminal should pop up, if not, just run the executable.
![image](https://github.com/jvick1/Rust_Intro/assets/32043066/b6342cff-af8c-46b4-947d-193658882380)

## First Project
Open a new terminal. 

Navigate to your working directory (wd) where you want this project to be created. Here are some commands if you need a refresher:

>**Windows Command Prompt:** Navigate back: `cd..`, Change to a specific path: `cd /c:/path`, List files and directories: `dir`, Change to a different drive: `E:`

>**Linux/macOS Terminal:** Navigate back: `cd ..`, Change to a specific path: `cd /path`, List files and directories: `ls`

Now let's get started! 

Type `cargo new hello`

## Run Code
Enter the project "hello" that we just created `cd hello`

Type `cargo run` to run the project.

Your terminal should print out "Hello, world!"

![image](https://github.com/jvick1/Rust_Intro/assets/32043066/4122c1ec-3eb8-4e0e-a079-1d4bb7491aa0)

A few things here:
1. Cargo is a package manager Pip (to Python) or npm (to JS) but Cargo is also the build system, test runner, docs generator, etc. Type `cargo --version` to see what version you are on.
2. The print command for Hello, world! can be found in `hello/src/main.rs`

# What Next?
Install this Repo and run each of the examples. 

Make sure you check out the Read Me files too! 
