# Zero-Cost Abstractions: Getting Rust Performance

## What is a Zero-Cost Abstraction?

A **zero-cost abstraction** is a language feature that:
- Provides high-level convenience
- Compiles to the same machine code as hand-written low-level code
- Adds no runtime overhead

**Rust's philosophy:** You shouldn't have to sacrifice abstraction for performance.

```rust
// High-level (comfortable)
let even: Vec<i32> = (1..100)
    .filter(|x| x % 2 == 0)
    .map(|x| x * 2)
    .collect();

// Low-level equivalent (fast)
let mut even = Vec::new();
for x in 1..100 {
    if x % 2 == 0 {
        even.push(x * 2);
    }
}

// Both compile to identical assembly!
```

---

## Why Rust Achieves This

### 1. Monomorphization

Generic code is specialized per concrete type at compile-time.

```rust
fn process<T: Display>(x: T) {
    println!("{}", x);
}

// Generates:
fn process_i32(x: i32) { /* specialized */ }
fn process_String(x: String) { /* specialized */ }

// No dynamic dispatch overhead!
```

### 2. Inlining

The compiler aggressively inlines small functions.

```rust
fn add_one(x: i32) -> i32 {
    x + 1
}

let result = add_one(5);

// Compiles to equivalent of:
let result = 5 + 1;
```

### 3. Dead Code Elimination

Unused code is removed entirely.

```rust
fn expensive() {
    // Complex calculation
}

fn main() {
    let x = 5;
    // expensive() never called
}

// expensive() doesn't appear in binary!
```

---

## Examples of Zero-Cost Abstractions

### Iterator Adapters

```rust
// This iterator chain:
(1..=10)
    .filter(|x| x % 2 == 0)
    .map(|x| x * 2)
    .sum::<i32>()

// Compiles to (roughly):
let mut sum = 0;
for x in 1..=10 {
    if x % 2 == 0 {
        sum += x * 2;
    }
}

// No intermediate allocations!
// No function call overhead!
// Just tight loop!
```

### Newtype Pattern

```rust
struct UserId(u32);  // Wrapper around u32

impl UserId {
    fn as_u32(&self) -> u32 {
        self.0
    }
}

let id = UserId(42);
let raw = id.as_u32();

// Compiles to:
let id: u32 = 42;
let raw = id;

// Zero overhead! Just type safety!
```

### RAII (Resource Acquisition Is Initialization)

```rust
{
    let file = File::open("data.txt")?;
    // ... use file ...
}  // file.drop() called (closes file)

// Compiles to:
{
    let file = Fopen("data.txt");
    // ... use file ...
    Fclose(file);
}

// Same as C!
```

### Smart Pointers

```rust
let boxed = Box::new(42);
println!("{}", *boxed);

// Compiles to (essentially):
let ptr = malloc(sizeof(i32));
*ptr = 42;
printf("%d", *ptr);
free(ptr);

// Some abstraction, zero cost!
```

---

## Performance Anti-Patterns to Avoid

### 1. Unnecessary Allocations

```rust
// ❌ Bad: allocates each iteration
let mut result = Vec::new();
for i in 0..1000 {
    let temp = format!("Item {}", i);  // Allocates!
    result.push(temp);
}

// ✅ Good: allocate once
let mut result = Vec::with_capacity(1000);
for i in 0..1000 {
    result.push(format!("Item {}", i));
}
```

### 2. Cloning Instead of Borrowing

```rust
// ❌ Bad: unnecessary clone
fn print_collection(vec: Vec<i32>) {
    for item in vec {
        println!("{}", item);
    }
}

let items = vec![1, 2, 3];
print_collection(items.clone());

// ✅ Good: borrow
fn print_collection(vec: &[i32]) {
    for item in vec {
        println!("{}", item);
    }
}

let items = vec![1, 2, 3];
print_collection(&items);
```

### 3. Premature Boxing

```rust
// ❌ Bad: heap allocate small structs
let point: Box<(i32, i32)> = Box::new((1, 2));

// ✅ Good: keep on stack
let point = (1, 2);
```

### 4. String Concatenation Loop

```rust
// ❌ Bad: allocates each iteration
let mut result = String::new();
for line in lines {
    result = result + &line + "\n";  // O(n²)!
}

// ✅ Good: use join or push
let result = lines.join("\n");

// Or:
let mut result = String::new();
for line in lines {
    result.push_str(&line);
    result.push('\n');  // O(n)
}
```

---

## Measuring Performance

### Using Criterion for Benchmarks

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_iteration(c: &mut Criterion) {
    c.bench_function("vec_iter", |b| {
        b.iter(|| {
            let v = black_box(vec![1, 2, 3, 4, 5]);
            v.iter().sum::<i32>()
        })
    });
}

criterion_group!(benches, bench_iteration);
criterion_main!(benches);
```

**Important:** Use `black_box()` to prevent compiler over-optimization.

---

## Profile-Guided Optimization

Rust can use profiling data to optimize better.

```bash
# Generate profiling data
cargo run --release

# Use profiling data in compilation
rustc -C llvm-args=-pgo-warn-missing-function -O myprogram.rs
```

---

## Checking the Generated Code

### Look at Assembly

```bash
# Generate and view LLVM IR
rustc --emit llvm-ir -C opt-level=3 myfile.rs

# View assembly
cargo rustc --release -- --emit asm

# Use Godbolt's Compiler Explorer for easy viewing
```

### Example: Monomorphization Check

```rust
fn generic<T: std::fmt::Debug>(x: T) {
    println!("{:?}", x);
}

fn main() {
    generic(5i32);
    generic("hello");
}

// Check with:
// cargo rustc --release -- --emit llvm-ir -C opt-level=3
```

---

## Key Rust Features for Performance

### 1. Iterators (No-Allocation Processing)

```rust
(1..1000)
    .filter(|x| is_prime(*x))
    .map(|x| x * 2)
    .take(100)
    .sum()

// Processes lazily - stops at 100!
```

### 2. Inline Hints

```rust
#[inline]
fn add_one(x: i32) -> i32 {
    x + 1
}

#[inline(always)]
fn critical_path(x: i32) -> i32 {
    x * 2  // Must inline
}
```

### 3. Const Functions

```rust
const fn factorial(n: u32) -> u32 {
    match n {
        0 | 1 => 1,
        _ => n * factorial(n - 1),
    }
}

const FACT_5: u32 = factorial(5);  // Computed at compile-time!
```

### 4. SIMD (vectorization)

```rust
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

// Let compiler vectorize
let sum: i32 = large_array
    .iter()
    .sum();
```

---

## Profile Your Code

### Flamegraph

```bash
cargo install flamegraph
cargo flamegraph --release

# Produces flamegraph.svg showing where time is spent
```

### Perf (Linux)

```bash
cargo install perf-tools
perf record ./target/release/myapp
perf report
```

---

## Memory Layout Optimization

### Struct Packing

```rust
// Inefficient: wasted space due to alignment
struct Bad {
    a: u8,   // 1 byte
    b: u64,  // 8 bytes (8 bytes padding before)
    c: u8,   // 1 byte (7 bytes padding after)
}
// Total: 16 bytes (8 wasted)

// Efficient: reorder to minimize padding
struct Good {
    b: u64,  // 8 bytes
    a: u8,   // 1 byte
    c: u8,   // 1 byte
    // 6 bytes padding
}
// Total: 16 bytes (still 6 wasted, unavoidable)

// Most efficient: use only what you need
struct Best {
    b: u64,  // 8 bytes
    a: u8,
    c: u8,
}
// Total: 10 bytes (compactly packed)
```

---

## Summary

| Technique | Cost | Benefit |
|-----------|------|---------|
| **Iterators** | Zero | Elegant, lazy evaluation |
| **Generics** | Compile-time | Type-safe, specialized code |
| **Inlining** | Zero | Function call overhead removed |
| **RAII** | Zero | Automatic cleanup |
| **Smart Pointers** | Zero | Safety without cost |

**Core Principle:** Rust doesn't make you choose between safety and performance. You get both.
