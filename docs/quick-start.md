# Quick Start Guide: Get Coding in 10 Minutes

## Installation

### 1. Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 2. Verify Installation
```bash
rustc --version
cargo --version
```

## Your First Program

### Create a New Project
```bash
cargo new my_first_app
cd my_first_app
```

### File Structure
```
my_first_app/
├── Cargo.toml       # Project configuration
└── src/
    └── main.rs      # Your code
```

### Hello, World!
```rust
fn main() {
    println!("Hello, World!");
}
```

### Run It
```bash
cargo run
```

Output:
```
   Compiling my_first_app v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 1.23s
     Running `target/debug/my_first_app`
Hello, World!
```

---

## Essential Commands

```bash
# Run your program
cargo run

# Compile only (no execution)
cargo build

# Compile with optimizations (slower compile, faster runtime)
cargo build --release

# Run tests
cargo test

# Check for errors without compiling
cargo check

# Format your code
cargo fmt

# Lint your code (find issues)
cargo clippy

# Add a dependency
cargo add serde

# Create documentation
cargo doc --open

# Generate a new library
cargo new --lib my_library
```

---

## Basic Syntax Crash Course

### Variables
```rust
let x = 5;              // Immutable
let mut y = 10;         // Mutable
y = 15;                 // OK

const MAX: i32 = 100;   // Constant (compile-time)
```

### Functions
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b  // Return value (no semicolon!)
}

fn greet(name: &str) {
    println!("Hello, {}!", name);
}

let sum = add(5, 3);
greet("Alice");
```

### Control Flow
```rust
// If/else
if x > 0 {
    println!("Positive");
} else if x < 0 {
    println!("Negative");
} else {
    println!("Zero");
}

// Match (pattern matching)
match x {
    1 => println!("One"),
    2 | 3 => println!("Two or three"),
    4..=10 => println!("Four to ten"),
    _ => println!("Something else"),
}

// Loops
for i in 0..5 {
    println!("{}", i);  // 0, 1, 2, 3, 4
}

while x < 10 {
    x += 1;
}

loop {  // Infinite loop
    break;  // Exit
}
```

### Strings and Collections
```rust
// Strings
let s = "hello";        // &str (string slice)
let s = String::from("hello");  // String (owned)
let s = "hello".to_string();

// Vectors (arrays)
let v = vec![1, 2, 3];
let mut v = Vec::new();
v.push(4);

// HashMap
use std::collections::HashMap;
let mut map = HashMap::new();
map.insert("name", "Alice");
```

### Error Handling
```rust
// Option: value might be missing
let x: Option<i32> = Some(5);
match x {
    Some(val) => println!("{}", val),
    None => println!("No value"),
}

// Result: operation might fail
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Division by zero".into())
    } else {
        Ok(a / b)
    }
}

match divide(10, 2) {
    Ok(result) => println!("Result: {}", result),
    Err(e) => println!("Error: {}", e),
}

// ? operator (propagate errors)
fn wrapped_divide(a: i32, b: i32) -> Result<i32, String> {
    let result = divide(a, b)?;  // If Err, return immediately
    Ok(result * 2)
}
```

---

## Project Setup: Choose Your Path

### Path 1: Learning (This Curriculum)
```bash
# Clone the curriculum
git clone https://github.com/CodeHalwell/Rust-Tutorial
cd Rust-Tutorial

# Start with Module 1
cat 01-syntax-and-ownership/notes/01-ownership-model.md

# Do exercises
cargo run --example 01-ownership-exercises
```

### Path 2: CLI Tool
```bash
cargo new my_cli
cd my_cli

# Add dependencies
cargo add clap --features derive
cargo add anyhow
```

**main.rs:**
```rust
use clap::Parser;
use anyhow::Result;

#[derive(Parser)]
#[command(name = "My CLI")]
#[command(about = "What it does")]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    println!("Processing: {}", args.input);
    Ok(())
}
```

### Path 3: Web Server
```bash
cargo new my_server
cd my_server

# Add Tokio and web framework
cargo add tokio --features full
cargo add axum
```

**main.rs:**
```rust
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello!" }));

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

### Path 4: Library
```bash
cargo new --lib my_library
cd my_library
```

**lib.rs:**
```rust
/// Adds two numbers
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }
}
```

---

## Next Steps

1. **Complete this exercise:**
   ```bash
   cargo new guessing_game
   cd guessing_game
   ```

   **main.rs:**
   ```rust
   use std::io;
   use rand::Rng;

   fn main() {
       let secret = rand::thread_rng().gen_range(1..=100);

       loop {
           println!("Guess a number (1-100):");
           let mut guess = String::new();
           io::stdin().read_line(&mut guess).unwrap();
           let guess: u32 = guess.trim().parse().unwrap();

           match guess.cmp(&secret) {
               std::cmp::Ordering::Less => println!("Too small!"),
               std::cmp::Ordering::Greater => println!("Too big!"),
               std::cmp::Ordering::Equal => {
                   println!("You win!");
                   break;
               }
           }
       }
   }
   ```

   Add dependency:
   ```bash
   cargo add rand
   ```

2. **Read Module 1** of this curriculum (2-3 hours)

3. **Join the Rust community:**
   - https://www.rust-lang.org/what/community
   - https://users.rust-lang.org/
   - https://discord.gg/rust-lang

4. **Bookmark these resources:**
   - The Book: https://doc.rust-lang.org/book/
   - Docs: https://docs.rs
   - Playground: https://play.rust-lang.org/

---

## Troubleshooting

### "command not found: cargo"
```bash
source $HOME/.cargo/env
```

### "error: could not compile"
Read the error message carefully! Rust errors are very helpful.

### "I don't understand ownership"
This is normal. It takes time.
- Re-read the ownership section
- Try small examples
- Use the playground to experiment

### "Code doesn't compile but should"
Check:
- Semicolons (Rust requires them)
- Type mismatches
- Missing `mut` keyword
- Lifetime issues

Use `cargo check` for faster feedback.

---

## Common First Mistakes

```rust
// ❌ Missing mut
let mut x = 5;  // Must use mut to modify
x = 10;

// ❌ Returning with semicolon
fn foo() -> i32 {
    42;  // Returns (), not i32!
}

// ✅ Fix:
fn foo() -> i32 {
    42   // No semicolon
}

// ❌ Using moved value
let s = String::from("hello");
let s2 = s;
println!("{}", s);  // s was moved!

// ✅ Fix: Use reference
let s = String::from("hello");
let s2 = &s;
println!("{}", s);
```

---

## Performance Tips (Early)

1. **Use `--release` flag for real tests**
   ```bash
   cargo run --release
   ```

2. **Add to Cargo.toml for faster builds:**
   ```toml
   [profile.dev]
   opt-level = 1
   ```

3. **Use `cargo check` while developing**
   ```bash
   cargo check  # Faster than cargo build
   ```

---

## You're Ready!

That's enough to get started. Rust has a learning curve, but it's worth it.

Next: Read [resources.md](resources.md) for a complete learning path, or dive into Module 1 of this curriculum.

**Remember:** Every Rust developer was confused by ownership at first. Keep going! 🦀
