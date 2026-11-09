# Memory Layout Optimization

## Why Layout Matters

CPUs fetch memory in **cache lines** (typically 64 bytes). A struct that spans two cache lines costs two fetches. Padding, field order, and alignment directly affect how much data fits in cache — and cache misses often dominate runtime.

```rust
// 12 bytes of fields, but 16 bytes of size due to alignment:
struct Bad { a: u8, b: u32, c: u8 }   // offsets 0, 4, 8 → size 12? no: align 4 → size 12
// Reorder to kill padding:
struct Good { b: u32, a: u8, c: u8 }  // offsets 0, 4, 5 → size 8
```

---

## Measuring Layout

```rust
use std::mem::{size_of, align_of, offset_of};

#[repr(C)]
struct Packet {
    kind: u8,
    flags: u16,
    length: u32,
    payload: u64,
}

fn main() {
    println!("size: {}  align: {}", size_of::<Packet>(), align_of::<Packet>());
    println!("offset flags: {}", offset_of!(Packet, flags));   // 2
    println!("offset length: {}", offset_of!(Packet, length)); // 4
    println!("offset payload: {}", offset_of!(Packet, payload)); // 8
}
```

---

## Struct of Arrays vs Array of Structs

```rust
// AoS: each particle is contiguous — good for per-particle work
struct Particle { pos: [f32; 3], vel: [f32; 3] }        // 24 bytes

// SoA: positions contiguous — good for bulk SIMD over one field
struct Particles {
    pos: Vec<[f32; 3]>,
    vel: Vec<[f32; 3]>,
}
```

**Rule:** AoS when you process whole objects; SoA when you process one field across many objects (vectorization-friendly).

---

## Hot/Cold Splitting

Split frequently-used ("hot") fields from rarely-used ("cold") ones:

```rust
struct Request {
    // hot: touched on every request
    id: u64,
    method: u8,
    // cold: only for logging/debugging
    debug_trace: Option<Box<Trace>>,   // boxed → keeps the hot struct small
}
```

---

## Small Types, Big Wins

- `bool`/`enum` discriminants are 1 byte; `Option<Box<T>>` is pointer-sized (niche optimization).
- `Option<&T>`, `Option<Box<T>>`, `Option<NonNull<T>>` are all the size of the pointer — **free** discriminant via the null niche.
- `u64` ids in hot paths: consider `u32` if the range allows — halves memory bandwidth.

```rust
// niche optimization: Option<Box<T>> is 8 bytes, not 16
println!("{}", std::mem::size_of::<Option<Box<u8>>>());  // 8
```

---

## False Sharing

Two threads writing to *different* fields of the same struct can still contend if the fields share a cache line:

```rust
use std::sync::atomic::{AtomicU64, Ordering};

struct Counters {
    a: AtomicU64,
    b: AtomicU64,   // same cache line as `a` → false sharing!
}

#[repr(align(64))]  // pad to a full cache line
struct PaddedCounter(AtomicU64);

struct GoodCounters {
    a: PaddedCounter,
    b: PaddedCounter,  // each on its own cache line
}
```

---

## Exercises

- `exercises/01-memory-layout-analysis.rs` — measure and optimize real structs
- `benchmarks/vector-iteration.rs` — AoS vs iterator vs chunks timing
