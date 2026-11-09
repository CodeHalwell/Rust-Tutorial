# SIMD and Auto-Vectorization

## Let the Compiler Vectorize First

Modern LLVM auto-vectorizes simple loops. Write clean, iterator-based code and check the assembly:

```rust
fn sum(data: &[f32]) -> f32 {
    data.iter().sum()   // LLVM often vectorizes this with -C target-cpu=native
}

// Check the assembly:
// cargo rustc --release -- --emit asm
// or: cargo asm (cargo-show-asm crate)
```

Build with optimizations that enable vectorization:

```toml
# .cargo/config.toml or RUSTFLAGS
[build]
rustflags = ["-C", "target-cpu=native"]
```

---

## When Auto-Vectorization Fails

It bails on: early `break`/`return` in loops, aliasing uncertainty, complex control flow, non-contiguous access. Fix by:

```rust
// ❌ may not vectorize: early exit
fn find(data: &[u32], needle: u32) -> Option<usize> {
    for (i, &x) in data.iter().enumerate() {
        if x == needle { return Some(i); }
    }
    None
}

// ✅ vectorizes: simple fold
fn contains(data: &[u32], needle: u32) -> bool {
    data.iter().any(|&x| x == needle)   // still may not vectorize...
}

// ✅ definitely vectorizable: chunk-wise compare
fn contains_simd(data: &[u32], needle: u32) -> bool {
    data.chunks_exact(8).any(|c| c.iter().any(|&x| x == needle))
}
```

---

## Explicit SIMD with `std::simd` (nightly) / `portable_simd`

On nightly, `std::simd` gives explicit vector types:

```rust
#![feature(portable_simd)]
use std::simd::prelude::*;

fn sum_simd(data: &[f32]) -> f32 {
    let mut acc = f32x8::splat(0.0);
    let chunks = data.chunks_exact(8);
    let remainder = chunks.remainder();
    for c in chunks {
        acc += f32x8::from_slice(c);
    }
    acc.reduce_sum() + remainder.iter().sum::<f32>()
}
```

On stable, use the **`wide`** crate or `packed_simd`-style crates, or just rely on auto-vectorization.

---

## Alignment for SIMD

`[f32; 8]` loads are fastest when 32-byte aligned. Use `#[repr(align(32))]` wrapper types for buffers you feed to SIMD code:

```rust
#[repr(align(32))]
struct Aligned8([f32; 8]);
```

---

## Rules of Thumb

1. Measure first — auto-vectorization often just works.
2. Keep hot loops simple: no early exits, contiguous slices, no aliasing.
3. Process data in cache-friendly order (SoA for field-wise ops).
4. Use `chunks_exact` to help the compiler see fixed-size blocks.
5. Verify with `cargo asm` / `perf` — never assume.

---

## Exercises

- `benchmarks/vector-iteration.rs` — compare loop styles empirically
