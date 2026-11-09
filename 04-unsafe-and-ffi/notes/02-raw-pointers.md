# Raw Pointers: `*const T` and `*mut T`

## What Raw Pointers Are

Raw pointers are addresses without ownership or lifetime guarantees:

```rust
let x = 5;
let r: *const i32 = &x as *const i32;   // immutable raw pointer
let mut y = 10;
let m: *mut i32 = &mut y as *mut i32;   // mutable raw pointer
```

**Key differences from references:**

| | `&T` / `&mut T` | `*const T` / `*mut T` |
|---|---|---|
| Nullable? | No | Yes |
| Dereference | Always safe | Requires `unsafe` |
| Ownership | Borrows | No borrow — any aliasing allowed |
| Lifetime | Tracked | Untracked |

---

## Creating Raw Pointers (safe)

Creating a raw pointer from a reference is safe; **dereferencing** is not:

```rust
let x = 42;
let ptr: *const i32 = &x;          // ✅ safe coercion
// let v = *ptr;                   // ❌ needs unsafe

let mut y = 0;
let mptr: *mut i32 = &mut y;       // ✅ safe
unsafe {
    *mptr = 7;                      // ✅ dereference in unsafe block
}
assert_eq!(y, 7);
```

You can also create them from integers (for FFI / hardware registers):

```rust
let addr: usize = 0x1000;
let ptr = addr as *const u8;       // ✅ creation is safe
// unsafe { *ptr }                  // ❌ dereference may be UB!
```

---

## Dereferencing and Pointer Arithmetic (unsafe)

```rust
let arr = [10, 20, 30, 40];
let base = arr.as_ptr();           // *const i32

unsafe {
    let first = *base;             // 10
    let third = *base.add(2);      // 30 — pointer arithmetic
    assert_eq!(first, 10);
    assert_eq!(third, 30);
}
```

`ptr.add(n)` advances by `n * size_of::<T>()` bytes. It must stay within the same allocated object (or one past the end) — violating this is UB.

- `ptr.offset(n)` — like `add` but allows negative; same rules.
- `ptr.wrapping_add(n)` / `wrapping_offset(n)` — wrapping arithmetic, useful for address math where you intentionally compute out-of-bounds addresses (e.g. hardware).
- `ptr.sub(n)`, `ptr.offset_from(other)`, `ptr.byte_add(n)` (stable since 1.75).

---

## `*mut T` vs `&mut T`

Raw mutable pointers allow **aliasing** — multiple `*mut T` to the same data. That's exactly what makes them dangerous with the compiler's assumptions: Rust assumes `&mut T` is unique, so aliasing `*mut T` can invalidate optimizations (UB via LLVM `noalias`).

```rust
let mut x = 1;
let p1: *mut i32 = &mut x;
let p2: *mut i32 = &mut x;         // both point at x — allowed for raw ptrs
unsafe {
    *p1 += 1;
    *p2 += 1;                       // x is now 3 — fine HERE, but if the compiler
}                                   // had assumed uniqueness, this could be UB.
```

---

## Common Safe Wrappers

Prefer these over hand-rolling raw pointer logic:

| Type | Use |
|---|---|
| `Box<T>` | heap allocation with ownership |
| `Rc<T>` / `Arc<T>` | shared ownership |
| `Vec<T>` / `&[T]` | contiguous buffers — use `.as_ptr()` / `.as_mut_ptr()` |
| `NonNull<T>` | non-null raw pointer with ownership semantics |
| `Unique<T>` (unstable) / `NonNull` | unique raw pointer |

```rust
let mut v = vec![1, 2, 3];
let ptr = v.as_mut_ptr();
unsafe {
    *ptr.add(1) = 99;               // mutate through the raw pointer
}
assert_eq!(v[1], 99);
```

---

## When You Actually Need Raw Pointers

1. **FFI**: C APIs hand you `void*` / opaque handles.
2. **Building safe abstractions**: `Vec`, `Box`, `Rc` are all built on raw pointers internally — you write `unsafe` once, expose a safe API.
3. **Unsafe data structures**: intrusive linked lists, arenas, lock-free structures.
4. **Hardware / OS**: memory-mapped registers at fixed addresses.

**Rule:** Reach for raw pointers only when references can't express what you need. Wrap them in a safe API as soon as possible.

---

## Exercises

- `exercises/01-unsafe-operations.rs`
- `exercises/02-raw-pointer-manipulation.rs`
- `exercises/03-ffi-bindings.rs`
