# Testing and Benchmarking Guide

## Unit Testing

### Basic Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[should_panic]
    fn test_panic() {
        panic!("This is expected");
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // Won't run by default
        // Run with: cargo test -- --ignored
    }
}
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_addition

# Run tests matching pattern
cargo test addition

# Show println! output
cargo test -- --nocapture

# Run ignored tests only
cargo test -- --ignored

# Run tests sequentially (not in parallel)
cargo test -- --test-threads=1
```

### Assertion Macros

```rust
assert!(condition);
assert_eq!(a, b);
assert_ne!(a, b);
debug_assert!(condition);  // Only in debug build

// Custom message
assert_eq!(a, b, "Values should be equal");
```

---

## Integration Tests

Place integration tests in `tests/` directory:

```
src/
├── lib.rs
└── main.rs

tests/
├── integration_test.rs
└── common/
    └── mod.rs
```

### Integration Test Example

```rust
// tests/integration_test.rs
use mylib::add;

#[test]
fn test_public_api() {
    assert_eq!(add(2, 3), 5);
}
```

### Shared Test Utilities

```rust
// tests/common/mod.rs
pub fn setup() {
    // Common setup code
}

// tests/integration_test.rs
mod common;

#[test]
fn test_with_setup() {
    common::setup();
    // Your test
}
```

---

## Benchmarking with Criterion

### Setup

```bash
cargo install criterion

# Create benches/my_benchmark.rs
```

### Benchmark Example

```rust
// benches/my_benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mylib::fibonacci;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| {
        b.iter(|| fibonacci(black_box(20)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```

### Cargo.toml Configuration

```toml
[package]
name = "mylib"
version = "0.1.0"

[[bench]]
name = "my_benchmark"
harness = false

[dev-dependencies]
criterion = { version = "0.3", features = ["html_reports"] }
```

### Running Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench fib

# Generate HTML reports
cargo bench -- --save-baseline baseline1
cargo bench -- --baseline baseline1
```

---

## Black Box and Optimization

### What is Black Box?

`black_box()` tells the compiler: "treat this as unknown". Prevents over-optimization.

```rust
// Without black_box (compiler optimizes away)
let result = black_box(vec![1, 2, 3]).len();
// Compiler sees constant 3, no actual vector

// With black_box
let result = black_box(vec![1, 2, 3]).len();
// Compiler treats as unknown, measures actual code
```

### Correct Benchmarking

```rust
// ❌ Bad: compiler optimizes away the work
c.bench_function("allocate", |b| {
    b.iter(|| Vec::with_capacity(1000))
});

// ✅ Good: black_box prevents optimization
c.bench_function("allocate", |b| {
    b.iter(|| Vec::with_capacity(black_box(1000)))
});
```

---

## Property Testing

Using `proptest` for generative testing:

```toml
[dev-dependencies]
proptest = "1.0"
```

### Property Test Example

```rust
#[cfg(test)]
mod tests {
    use proptest::proptest;

    proptest! {
        #[test]
        fn test_add_commutative(a in 0i32..100, b in 0i32..100) {
            assert_eq!(a + b, b + a);
        }

        #[test]
        fn test_len_after_push(mut vec in prop::collection::vec(0i32..100, 0..10)) {
            let initial_len = vec.len();
            vec.push(50);
            assert_eq!(vec.len(), initial_len + 1);
        }
    }
}
```

---

## Fuzz Testing

Using `cargo-fuzz` for security testing:

```bash
cargo install cargo-fuzz

cargo fuzz add parse_input
```

### Fuzz Target Example

```rust
// fuzz/fuzz_targets/parse_input.rs
#![no_main]
use libfuzzer_sys::fuzz_target;
use mylib::parse;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = parse(s);
    }
});
```

---

## Profiling

### Using Perf (Linux)

```bash
# Compile with debug symbols
cargo build --release

# Profile the binary
perf record -g ./target/release/myapp

# View results
perf report

# Generate flamegraph
perf script | stackcollapse-perf.pl | flamegraph.pl > flame.svg
```

### Instrumenting Code

```rust
use std::time::Instant;

fn slow_operation() {
    let start = Instant::now();

    // ... work ...

    let duration = start.elapsed();
    println!("Operation took: {:?}", duration);
}
```

---

## Test Organization Best Practices

### 1. Descriptive Test Names

```rust
#[test]
fn test_parse_valid_json_returns_object() {
    // Clear what's being tested
}

#[test]
fn test_empty_string_returns_error() {
    // Clear what's being tested
}
```

### 2. AAA Pattern (Arrange-Act-Assert)

```rust
#[test]
fn test_search_finds_matching_items() {
    // Arrange
    let mut haystack = vec![1, 2, 3, 4, 5];
    let needle = 3;

    // Act
    let result = haystack.iter().find(|&&x| x == needle);

    // Assert
    assert_eq!(result, Some(&3));
}
```

### 3. One Assertion Per Test (Or Related)

```rust
// ❌ Too many things
#[test]
fn test_calculation() {
    assert_eq!(add(2, 3), 5);
    assert_eq!(multiply(2, 3), 6);
    assert_eq!(divide(6, 3), 2);
    // If first fails, others aren't run
}

// ✅ Better: specific tests
#[test]
fn test_addition() {
    assert_eq!(add(2, 3), 5);
}

#[test]
fn test_multiplication() {
    assert_eq!(multiply(2, 3), 6);
}
```

---

## Continuous Testing

### With Watchexec

```bash
# Auto-run tests on file change
cargo install watchexec-cli
watchexec -e rs cargo test
```

### With Cargo Watch

```bash
cargo install cargo-watch
cargo watch -x test
```

---

## Coverage

Using `tarpaulin`:

```bash
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html
```

---

## Benchmarking Best Practices

### 1. Warmup Iterations

```rust
let mut criterion = Criterion::default()
    .warm_up_time(Duration::from_secs(5));
```

### 2. Multiple Baselines

```rust
// Baseline 1
cargo bench -- --save-baseline v1

// Make changes

// Baseline 2
cargo bench -- --save-baseline v2

// Compare
cargo bench -- --baseline v1 --baseline v2
```

### 3. Statistical Rigor

Criterion uses statistical methods:
- Multiple runs
- Detects regressions
- Confidence intervals
- Chi-square test for variance

### 4. Measure Specific Things

```rust
c.bench_function("parse", |b| {
    let input = black_box(JSON_STRING);
    b.iter(|| parse(input))
});
```

---

## Common Testing Patterns

### Testing Error Cases

```rust
#[test]
fn test_division_by_zero() {
    assert!(divide(10, 0).is_err());
}

#[test]
fn test_error_message() {
    let err = divide(10, 0).unwrap_err();
    assert_eq!(err, "Division by zero");
}
```

### Testing State Changes

```rust
#[test]
fn test_push_increases_len() {
    let mut v = Vec::new();
    assert_eq!(v.len(), 0);

    v.push(42);
    assert_eq!(v.len(), 1);

    v.push(43);
    assert_eq!(v.len(), 2);
}
```

### Testing with Setup/Teardown

```rust
#[test]
fn test_with_fixture() {
    // Setup
    let mut db = Database::new();
    db.insert("key", "value");

    // Test
    assert_eq!(db.get("key"), Some("value"));

    // Teardown (automatic via Drop)
}
```

---

## Test Coverage Goals

- **Core logic:** 90%+
- **Error paths:** 85%+
- **UI/formatting:** 50%+
- **Overall target:** 80%+

Remember: 100% coverage doesn't mean 100% correctness. Focus on critical paths.

---

## Summary Table

| Tool | Purpose | Use When |
|------|---------|----------|
| **cargo test** | Unit/integration tests | Every change |
| **criterion** | Benchmarking | Performance sensitive |
| **proptest** | Property testing | Complex logic |
| **cargo-fuzz** | Fuzzing | Security sensitive |
| **perf** | Profiling | Performance investigation |
| **tarpaulin** | Coverage | Assessing test completeness |

Best practice: **Test as you code. Measure before optimizing.**
