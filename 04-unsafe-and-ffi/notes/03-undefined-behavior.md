# Undefined Behavior: Categories and Gotchas

## What UB Actually Means

**Undefined Behavior** is not "a crash" and not "garbage output". UB means: *the compiler is allowed to assume it never happens*. If it happens anyway, any behavior is permitted — including the program appearing to work, then corrupting data, or the optimizer deleting your code entirely.

```rust
fn example(flag: bool) -> i32 {
    let x: i32;
    if flag {
        x = 1;
    }
    x + 1  // If flag can be false, this reads uninitialized memory = UB.
          // LLVM may assume flag is always true and constant-fold to 2.
}
```

---

## UB Categories in Rust

The Rustonomicon lists the main categories of UB in `unsafe` code:

1. **Dereferencing null, dangling, or misaligned pointers**
   ```rust
   let p: *const i32 = std::ptr::null();
   unsafe { *p };  // UB
   ```
2. **Reading uninitialized memory** (including partially initialized structs)
3. **Violating pointer alignment** — a `*const u64` must be 8-byte aligned:
   ```rust
   let bytes = [0u8; 8];
   let p = bytes.as_ptr() as *const u64;  // likely misaligned → UB on read
   unsafe { *p };  // UB if not 8-aligned
   ```
4. **Creating invalid values** — e.g. `bool` must be 0 or 1, `char` must be a valid Unicode scalar, references must be non-null and aligned:
   ```rust
   let b: bool = unsafe { std::mem::transmute(3u8) };  // UB the moment it's created
   ```
5. **Data races** — two threads accessing the same memory, at least one writing, without synchronization
6. **Violating aliasing rules** — e.g. `&mut` aliasing (LLVM `noalias`), or mutating through `*const` that points to `&`-borrowed data... actually mutating through `*const` derived from a shared reference is UB if the underlying data is immutable.
7. **Out-of-bounds pointer arithmetic** — `ptr.add(n)` leaving the allocation
8. **Invalid FFI calls** — wrong ABI, passing invalid values to `extern "C"` fns, panicking across FFI (aborts since Rust 1.81 it's a defined abort; before, UB)
9. **Executing invalid machine code / jumping to bad addresses** (inline asm)

---

## Subtle Gotchas

### `mem::uninitialized` / `MaybeUninit`
`std::mem::uninitialized()` is deprecated because it instantly creates UB for most types. Use `MaybeUninit<T>`:

```rust
use std::mem::MaybeUninit;
let mut buf: [MaybeUninit<u8>; 4] = MaybeUninit::uninit().assume_init_safe();
// ... write into buf ...
// Only then: let arr: [u8; 4] = unsafe { std::mem::transmute(buf) };
```

### `transmute` size mismatch
```rust
let x: u64 = 1;
let y: u32 = unsafe { std::mem::transmute(x) };  // ❌ UB — sizes differ (compile error actually, transmute checks size)
```
`transmute` does check sizes at compile time, but type validity is still your problem (the `bool` example above).

### Panics in `extern "C"`
Since Rust 1.81, a panic unwinding into an `extern "C"` fn is an automatic abort (defined behavior). Use `extern "C-unwind"` if you need unwinding. Before 1.81 it was UB.

### `&mut` from raw parts
Creating a `&mut` from a raw pointer is only sound if the pointer is unique, aligned, non-null, and points to initialized data — that's why `slice::from_raw_parts_mut` is unsafe.

---

## Detecting UB: Miri

[Miri](https://github.com/rust-lang/miri) interprets your code and flags UB:

```bash
rustup component add miri
cargo +nightly miri test
```

Miri catches: dangling pointer derefs, alignment violations, data races (with its own scheduler), invalid values, UB in FFI stubs. It cannot catch everything (it doesn't model all real hardware), but it's the best UB detector available.

---

## Sanitizers

```bash
RUSTFLAGS="-Zsanitizer=address" cargo +nightly run   # AddressSanitizer
RUSTFLAGS="-Zsanitizer=memory"  cargo +nightly run   # MemorySanitizer (uninit reads)
```

---

## Rules of Thumb

1. Minimize `unsafe` blocks — small, well-audited.
2. Document the safety contract of every `unsafe fn`: `// SAFETY: ...`
3. Never expose `unsafe` in a public API without a safe wrapper.
4. Run Miri in CI for unsafe-heavy code.
5. When in doubt, the answer is "that's UB" — ask Miri.

---

## Exercises

See `exercises/` — including identifying UB in broken samples.
