/// ============================================================================
/// BENCHMARK: Vector Iteration Styles
/// ============================================================================
///
/// Compares loop styles on a large vector. This is a *rough* benchmark using
/// std::time::Instant — for rigorous stats use criterion (see
/// notes/05-benchmarking-with-criterion.md).
///
/// Run: rustc --edition 2021 -O vector-iteration.rs -o vi && ./vi

use std::time::Instant;

fn make_data(n: usize) -> Vec<u64> {
    (0..n as u64).map(|i| i * 2654435761 % 1_000_000_007).collect()
}

fn bench<F: FnMut() -> u64>(name: &str, mut f: F) -> u64 {
    // warmup
    std::hint::black_box(f());
    let t = Instant::now();
    let result = f();
    let elapsed = t.elapsed();
    println!("{:<28} {:>10.2?}  (checksum {})", name, elapsed, result);
    result
}

fn main() {
    let data = make_data(10_000_000);
    println!("=== VECTOR ITERATION BENCHMARK ({} elements) ===\n", data.len());

    // 1. iterator chain (often auto-vectorized or at least well-optimized)
    let r1 = bench("iter().map().sum()", || {
        data.iter().map(|x| x % 97).sum::<u64>()
    });

    // 2. explicit for loop over slice
    let r2 = bench("for loop over &data", || {
        let mut acc = 0u64;
        for x in &data {
            acc += x % 97;
        }
        acc
    });

    // 3. indexed loop (usually slower: bounds checks)
    let r3 = bench("indexed loop data[i]", || {
        let mut acc = 0u64;
        for i in 0..data.len() {
            acc += data[i] % 97;
        }
        acc
    });

    // 4. chunks_exact — fixed-size blocks, vectorization-friendly
    let r4 = bench("chunks_exact(8) fold", || {
        let mut acc = 0u64;
        for chunk in data.chunks_exact(8) {
            for x in chunk {
                acc += x % 97;
            }
        }
        acc += data.chunks_exact(8).remainder().iter().map(|x| x % 97).sum::<u64>();
        acc
    });

    // 5. parallel with scoped threads (std only) — split into 4 ranges
    let r5 = bench("4 threads (thread::scope)", || {
        let n = data.len();
        let data = &data; // reborrow so scoped threads can share it
        std::thread::scope(|s| {
            let mut handles = Vec::new();
            for t in 0..4 {
                let start = n * t / 4;
                let end = n * (t + 1) / 4;
                handles.push(s.spawn(move || data[start..end].iter().map(|x| x % 97).sum::<u64>()));
            }
            handles.into_iter().map(|h| h.join().unwrap()).sum::<u64>()
        })
    });

    println!();
    assert_eq!(r1, r2);
    assert_eq!(r2, r3);
    assert_eq!(r3, r4);
    assert_eq!(r4, r5);
    println!("all checksums agree: {} — results are correct", r1);
}
