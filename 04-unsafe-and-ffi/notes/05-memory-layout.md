# Memory Layout: `repr(C)`, Padding, and Alignment

## Default Layout Is Unspecified

By default Rust may **reorder struct fields** to minimize size:

```rust
struct Foo {
    a: u8,
    b: u32,
    c: u8,
}
// Rust may lay this out as [b(4 bytes)][a][c] = 6 bytes + padding,
// NOT necessarily in declaration order!
println!("{}", std::mem::size_of::<Foo>());  // 8, not 12
```

**Key insight:** Never assume field order or offsets in default `repr(Rust)` structs — the compiler optimizes for size.

---

## `repr(C)`: C-Compatible Layout

`#[repr(C)]` guarantees declaration-order layout, C-compatible padding/alignment — required for FFI:

```rust
#[repr(C)]
struct Point {
    x: f64,   // offset 0
    y: f64,   // offset 8
}             // size 16, align 8

#[repr(C)]
struct Mixed {
    a: u8,    // offset 0
    b: u32,   // offset 4 (3 bytes padding after a)
    c: u8,    // offset 8
}             // size 12, align 4
```

Inspect layout at compile time:

```rust
use std::mem::{size_of, align_of, offset_of};

#[repr(C)]
struct Mixed { a: u8, b: u32, c: u8 }

fn main() {
    println!("size={} align={}", size_of::<Mixed>(), align_of::<Mixed>());
    println!("offset b = {}", offset_of!(Mixed, b));  // 4  (stable since 1.77)
}
```

---

## Padding and Alignment

- **Alignment** of a type = the address requirement of its most-aligned field (a `u64` needs 8-byte alignment).
- **Padding** fills gaps so each field is properly aligned, plus trailing padding so arrays keep alignment (`size_of::<T>()` is always a multiple of `align_of::<T>()`).

```rust
struct Bad {      // size 12: u8 @0, pad 3, u32 @4, u8 @8, pad 3
    a: u8,
    b: u32,
    c: u8,
}

struct Good {     // size 8: reorder largest-first
    b: u32,
    a: u8,
    c: u8,       // pad 2 at end
}
```

**Rule:** Order fields from largest to smallest to minimize padding.

---

## `repr` Variants

| Attribute | Effect | Use |
|---|---|---|
| `repr(Rust)` (default) | unspecified order, minimal size | general Rust code |
| `repr(C)` | C order/layout | FFI, matching C structs |
| `repr(transparent)` | same layout as the single non-zero-sized field | newtypes, FFI wrappers |
| `repr(packed)` | no padding, align 1 | hardware registers (careful: unaligned refs are UB) |
| `repr(u8)` etc. | fieldless enums get explicit discriminant type | FFI enums, `#[repr(C)]` enums |

```rust
#[repr(transparent)]
struct UserId(u64);   // identical layout to u64 — zero cost

#[repr(packed)]
struct Registers {
    status: u8,
    control: u32,     // may be unaligned!
}
// ⚠️ taking &registers.control is UB if misaligned — copy it out instead:
// let c = registers.control;  // read by value is fine (compiler handles it)
```

---

## Unions

```rust
#[repr(C)]
union Value {
    i: i32,
    f: f32,
}

let v = Value { f: 1.5 };
unsafe {
    println!("{}", v.f);   // reading a union field is unsafe
    println!("{}", v.i);   // reinterpret bits
}
```

---

## Practical Tools

- `std::mem::{size_of, align_of, offset_of}` — compile-time layout queries.
- `cargo-show-asm` / `cargo expand` — inspect generated code.
- [bytemuck](https://crates.io/crates/bytemuck) — safe transmute between `Pod` types (`#[derive(Pod, Copy, Clone)]` with `repr(C)`).
- [memoffset](https://crates.io/crates/memoffset) — `offset_of!` before Rust 1.77.

---

## Cache Locality

Layout affects performance: fields accessed together should be adjacent (same cache line, 64 bytes typically). This is why `struct-of-arrays` (SoA) often beats `array-of-structs` (AoS) for bulk processing.

---

## Exercises

- `06-performance/exercises/01-memory-layout-analysis.rs` — measure layouts of real structs
