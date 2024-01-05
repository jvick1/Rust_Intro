*v1.0.0* 

*Last Updated: 01/05/24*

# Getting Started

Explore the projects in this repository to deepen your understanding of Rust:

1. "README": Finish this doc to set up rust before moving on to the different projects listed below.
2. "hello": An introduction to Rust. Compare the read-me file with the "hello" project you just created.
3. "variables": Learn about variables with comprehensive explanations and code snippets.
4. "functions": Delve into calculations, functions, and more.
5. "module": Start exploring object-oriented programming concepts.
6. In Progress... Stay tuned for more learning resources and projects.

Happy coding!

# Rust_Intro

Welcome to Rust_Intro, your gateway to learning Rust! This guide will walk you through the process of installing Rust, setting up your initial projects, and provide step-by-step instructions for understanding the basics of Rust programming.

## Installation

Start your Rust journey by installing Rust on your system. Visit [Rust's official website](https://www.rust-lang.org/tools/install) and follow the installation instructions. The images below illustrate the installation process:

![Rust Installation](https://github.com/jvick1/Rust_Intro/assets/32043066/f51906c7-9dfe-4698-ade1-d020feea1226)

## Setup

Once the installation is complete, you'll find a new terminal pop-up. If not, run the executable manually. The image below demonstrates this process:

![Rust Setup](https://github.com/jvick1/Rust_Intro/assets/32043066/b6342cff-af8c-46b4-947d-193658882380)

## Your First Project

Let's dive into creating your first Rust project. Open a new terminal, navigate to your desired working directory, and execute the following commands:

### Windows Command Prompt:

- Navigate back: `cd..`
- Change to a specific path: `cd /c:/path`
- List files and directories: `dir`
- Change to a different drive: `E:`

### Linux/macOS Terminal:

- Navigate back: `cd ..`
- Change to a specific path: `cd /path`
- List files and directories: `ls`

Now, create a new project named "hello" by typing:

```bash
cargo new hello
```

## Running Your Code
Enter the project directory with:

```bash
cd hello
```

Run the project:

```bash
cargo run
```

Your terminal should display "Hello, world!" as the output:

![image](https://github.com/jvick1/Rust_Intro/assets/32043066/4122c1ec-3eb8-4e0e-a079-1d4bb7491aa0)

## Understanding Cargo

1. `Cargo:` Think of it as Rust's package manager, similar to Pip for Python or npm for JavaScript. But Cargo is more—it's also the build system, test runner, docs generator, and more. Check the version with:

```bash
cargo --version
```
   
2. `Hello, world! Code:` You'll find the code responsible for printing "Hello, world!" in `hello/src/main.rs`.

