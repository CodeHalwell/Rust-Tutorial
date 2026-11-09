# FFI Basics: Calling C from Rust (and Vice Versa)

## The `extern "C"` Block

Declare foreign functions in an `extern` block; call them inside `unsafe`:

```rust
// C side:  int32_t add(int32_t a, int32_t b);
extern "C" {
    fn add(a: i32, b: i32) -> i32;
}

fn main() {
    let r = unsafe { add(2, 3) };
    println!("{}", r);  // 5
}
```

**Key insight:** The `extern` block is a promise to the linker. The `unsafe` call site acknowledges: "I guarantee this function is safe to call with these arguments."

---

## Type Mapping

| C type | Rust type |
|---|---|
| `int` / `int32_t` | `i32` / `std::os::raw::c_int` |
| `unsigned` | `u32` / `c_uint` |
| `long` | `c_long` (platform-dependent!) |
| `long long` | `i64` / `c_longlong` |
| `size_t` | `usize` |
| `char` | `c_char` (i8 or u8 depending on platform) |
| `float` / `double` | `f32` / `f64` |
| `void*` / `T*` | `*mut c_void` / `*mut T` |
| `const char*` | `*const c_char` |
| `void` return | no return type |
| `bool` | `bool` (C99 `_Bool`) |

Use `std::os::raw::*` or `std::ffi::{c_char, c_void, CString, CStr}` for portability.

```rust
use std::os::raw::{c_char, c_int};
use std::ffi::CString;

extern "C" {
    // C: size_t strlen(const char* s);
    fn strlen(s: *const c_char) -> usize;
}

fn main() {
    let s = CString::new("hello").unwrap(); // no interior NUL allowed
    let len = unsafe { strlen(s.as_ptr()) };
    assert_eq!(len, 5);
}
```

---

## Strings: CString and CStr

- `CString` — owned, NUL-terminated, no interior NULs. `CString::new("...")` returns `Result` (fails on interior NUL).
- `&CStr` — borrowed view. `c"literal"` (Rust 1.77+) creates a `&'static CStr` literal.
- `CStr::to_str()` — UTF-8 check → `Result<&str, Utf8Error>`.

```rust
use std::ffi::{CStr, CString};

let c = CString::new("hello").unwrap();
let s: &str = c.to_str().unwrap();          // Rust &str (borrows the CString)
let raw: &CStr = c.as_c_str();               // C view
assert_eq!(raw.to_bytes(), b"hello");

let lit = c"world";                          // &'static CStr
assert_eq!(lit.to_str().unwrap(), "world");
```

---

## Calling Rust from C

Export Rust functions with `#[no_mangle] extern "C"`:

```rust
#[no_mangle]
pub extern "C" fn rust_add(a: i32, b: i32) -> i32 {
    a + b
}
```

Compile as a static/dynamic library:

```toml
# Cargo.toml
[lib]
crate-type = ["staticlib", "cdylib"]
```

```bash
cargo build --release
# produces librust_lib.a (staticlib) and librust_lib.so (cdylib)
```

Then in C: `extern int32_t rust_add(int32_t, int32_t);` and link against the library. Generate headers with `cbindgen`.

---

## Callbacks: Passing Rust Closures to C

C APIs often take function pointers + a `void* user_data`. The pattern:

```rust
type Callback = extern "C" fn(i32, *mut std::ffi::c_void);

extern "C" {
    fn register(cb: Callback, user_data: *mut std::ffi::c_void);
}

extern "C" fn my_callback(value: i32, _data: *mut std::ffi::c_void) {
    println!("called with {}", value);
}

fn main() {
    unsafe { register(my_callback, std::ptr::null_mut()) };
}
```

For stateful closures, box the closure and pass the box pointer as `user_data`:

```rust
extern "C" fn trampoline<F: FnMut(i32)>(value: i32, data: *mut std::ffi::c_void) {
    let f = unsafe { &mut *(data as *mut F) };
    f(value);
}
```

---

## bindgen: Auto-Generating Bindings

For large C libraries, use [bindgen](https://github.com/rust-lang/rust-bindgen):

```bash
cargo install bindgen-cli
bindgen header.h -o bindings.rs
```

Or in build.rs:

```rust
// build.rs
fn main() {
    bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .unwrap()
        .write_to_file("src/bindings.rs")
        .unwrap();
}
```

---

## Rules for Safe FFI

1. Keep `unsafe` at the boundary; wrap in safe Rust functions immediately.
2. Validate all inputs from C (lengths, nulls, ranges).
3. Never let a Rust panic cross the FFI boundary (aborts; use `catch_unwind` if you must).
4. Document ownership: who frees what? (e.g. `CString::into_raw` / `from_raw` pairs.)
5. Match ABIs exactly — `extern "C"` vs `extern "C-unwind"` vs Rust ABI.

---

## Exercises

- `exercises/03-ffi-bindings.rs` — self-contained FFI exercise (Rust calling Rust-as-C)
