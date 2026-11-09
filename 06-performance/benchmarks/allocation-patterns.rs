/// ============================================================================
/// BENCHMARK: Allocation Patterns
/// ============================================================================
///
/// Shows the cost of allocations in hot paths: Vec growth vs with_capacity,
/// and clone vs borrow. Rough timing with Instant — use criterion for
/// publication-grade numbers.
///
/// Run: rustc --edition 2021 -O allocation-patterns.rs -o ap && ./ap

use std::time::Instant;

fn bench<F: FnMut() -> usize>(name: &str, mut f: F) -> usize {
    std::hint::black_box(f()); // warmup
    let t = Instant::now();
    let n = f();
    println!("{:<40} {:>10.2?}  ({} items)", name, t.elapsed(), n);
    n
}

fn main() {
    const N: usize = 1_000_000;
    println!("=== ALLOCATION PATTERNS BENCHMARK (N = {}) ===\n", N);

    // 1. Vec::push without capacity — repeated reallocations
    let r1 = bench("push without capacity", || {
        let mut v = Vec::new();
        for i in 0..N {
            v.push(i);
        }
        v.len()
    });

    // 2. with_capacity — one allocation
    let r2 = bench("with_capacity up front", || {
        let mut v = Vec::with_capacity(N);
        for i in 0..N {
            v.push(i);
        }
        v.len()
    });

    // 3. collect from iterator — usually reserves exactly
    let r3 = bench("(0..N).collect()", || (0..N).collect::<Vec<_>>().len());

    // 4. clone a big String in a loop vs borrow
    let big = "x".repeat(1000);
    let r4 = bench("clone String x 10_000 in loop", || {
        let mut total = 0;
        for _ in 0..10_000 {
            let s = big.clone(); // heap alloc every iteration
            total += s.len();
        }
        total
    });

    let r5 = bench("borrow &String x 10_000 in loop", || {
        let mut total = 0;
        for _ in 0..10_000 {
            let s = &big; // no allocation
            total += s.len();
        }
        total
    });

    println!();
    assert_eq!(r1, N);
    assert_eq!(r2, N);
    assert_eq!(r3, N);
    assert_eq!(r4, r5);
    println!("correctness checks passed (clone and borrow give {})", r4);
    println!("\nLesson: prefer with_capacity / iterators; avoid clone in hot loops.");
}
