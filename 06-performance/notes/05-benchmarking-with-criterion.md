# Benchmarking with Criterion

## Why Criterion

`#[bench]` (nightly-only, unstable) is painful. [Criterion](https://crates.io/crates/criterion) gives:

- Statistical rigor (confidence intervals, outlier detection)
- Regression detection (compares against previous runs, saved in `target/criterion/`)
- Warmup, measurement time control, plotting

```toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "my_bench"
harness = false
```

```rust
// benches/my_bench.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```

```bash
cargo bench
```

---

## The `black_box` Trick

The compiler can optimize away benchmarked code it can see. `black_box(x)` is an opaque barrier:

```rust
b.iter(|| process(black_box(&input)))
```

Without it, you might benchmark an empty loop.

---

## Benchmark Groups and Comparisons

```rust
fn bench_compare(c: &mut Criterion) {
    let mut group = c.benchmark_group("sorting");
    for size in [100, 1_000, 10_000] {
        let mut data: Vec<u32> = (0..size).rev().collect();
        group.bench_with_input(format!("sort_{}", size), &data, |b, d| {
            b.iter(|| {
                let mut v = d.clone();
                v.sort_unstable();
                black_box(v);
            })
        });
    }
    group.finish();
}
```

Criterion automatically compares against the previous run and reports "change: +12.3% (95% confidence: ...)".

---

## Methodology Rules

1. **Warmup**: criterion does this automatically.
2. **One change per measurement**.
3. **Randomize/rotate input order** to avoid cache-order bias.
4. **Use realistic data sizes**.
5. **Pin CPU frequency** if comparing across machines (`taskset`, disable turbo) — or just compare on the same machine.
6. **Never benchmark debug builds.**

---

## Quick Benchmarks Without Criterion

For a rough check, `std::time::Instant` works (see `benchmarks/` in this module):

```rust
use std::time::Instant;
let t = Instant::now();
// ... work ...
println!("took {:?}", t.elapsed());
```

---

## Exercises

- `benchmarks/vector-iteration.rs`
- `benchmarks/allocation-patterns.rs`
