/// ============================================================================
/// FFI BINDINGS EXERCISE (self-contained: Rust plays both sides)
/// ============================================================================
///
/// To keep this runnable with plain rustc (no C compiler needed), the "C
/// library" below is implemented in Rust with #[no_mangle] extern "C" —
/// exactly how you'd expose Rust TO C. The client side declares the same
/// functions in an extern "C" block — exactly how you'd call a real C lib.
///
/// Run: rustc --edition 2021 03-ffi-bindings.rs -o ffi && ./ffi

use std::ffi::{c_char, c_int, c_void, CStr, CString};
use std::os::raw::c_uint;

// ============================================================================
// PART 1: The "C library" side — Rust functions exported with a C ABI.
// They live in a module so their Rust paths (c_lib::c_add) don't collide
// with the extern "C" declarations below; #[no_mangle] still exports the
// plain symbol names (c_add, ...) that the extern block links against.
// ============================================================================

mod c_lib {
    use super::*;

    #[no_mangle]
    pub extern "C" fn c_add(a: c_int, b: c_int) -> c_int {
        a + b
    }

    /// C: size_t c_strlen(const char* s);
    #[no_mangle]
    pub extern "C" fn c_strlen(s: *const c_char) -> usize {
        if s.is_null() {
            return 0;
        }
        // SAFETY: caller guarantees s is a valid NUL-terminated C string.
        unsafe { CStr::from_ptr(s) }.to_bytes().len()
    }

    /// A C-style opaque handle: we hand out a boxed struct pointer.
    #[no_mangle]
    pub extern "C" fn counter_create(initial: c_int) -> *mut c_void {
        Box::into_raw(Box::new(initial)) as *mut c_void
    }

    #[no_mangle]
    pub extern "C" fn counter_increment(handle: *mut c_void) -> c_int {
        // SAFETY: handle must come from counter_create and not be freed yet.
        unsafe { *(handle as *mut c_int) += 1 };
        unsafe { *(handle as *mut c_int) }
    }

    #[no_mangle]
    pub extern "C" fn counter_free(handle: *mut c_void) {
        if !handle.is_null() {
            // SAFETY: handle came from Box::into_raw in counter_create.
            unsafe {
                drop(Box::from_raw(handle as *mut c_int));
            }
        }
    }
}

// ============================================================================
// PART 2: The "Rust client" side — declaring and calling the C ABI
// ============================================================================

extern "C" {
    fn c_add(a: c_int, b: c_int) -> c_int;
    fn c_strlen(s: *const c_char) -> usize;
    fn counter_create(initial: c_int) -> *mut c_void;
    fn counter_increment(handle: *mut c_void) -> c_int;
    fn counter_free(handle: *mut c_void);
}

pub fn call_c_add(a: i32, b: i32) -> i32 {
    // SAFETY: c_add is a pure function; any i32 args are valid.
    unsafe { c_add(a, b) }
}

pub fn call_c_strlen(s: &CString) -> usize {
    // SAFETY: s.as_ptr() is a valid NUL-terminated string that outlives the call.
    unsafe { c_strlen(s.as_ptr()) }
}

pub fn exercise_opaque_handle() -> i32 {
    // SAFETY: we pass the handle between create/increment/free without
    // freeing twice or using it after free.
    unsafe {
        let h = counter_create(10);
        let a = counter_increment(h);
        let b = counter_increment(h);
        counter_free(h);
        assert_eq!(a, 11);
        b
    }
}

// ============================================================================
// PART 3: repr(C) struct shared across the boundary
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CPoint {
    pub x: f64,
    pub y: f64,
}

mod c_lib2 {
    use super::CPoint;

    #[no_mangle]
    pub extern "C" fn point_distance_squared(p: CPoint) -> f64 {
        p.x * p.x + p.y * p.y
    }
}

extern "C" {
    fn point_distance_squared(p: CPoint) -> f64;
}

pub fn exercise_repr_c_struct() -> f64 {
    let p = CPoint { x: 3.0, y: 4.0 };
    // SAFETY: CPoint is repr(C), passed by value with the C ABI.
    unsafe { point_distance_squared(p) }
}

// ============================================================================
// PART 4: Callback trampoline (C calls back into Rust)
// ============================================================================

type CCallback = extern "C" fn(value: c_int, user_data: *mut c_void);

extern "C" {
    fn c_foreach(start: c_int, end: c_int, cb: CCallback, user_data: *mut c_void);
}

mod c_lib3 {
    use super::*;

    #[no_mangle]
    pub extern "C" fn c_foreach(start: c_int, end: c_int, cb: CCallback, user_data: *mut c_void) {
        for i in start..end {
            cb(i, user_data);
        }
    }
}

extern "C" fn sum_trampoline(value: c_int, user_data: *mut c_void) {
    // SAFETY: user_data is the &mut i32 we passed from do_foreach_sum.
    unsafe {
        *(user_data as *mut c_int) += value;
    }
}

pub fn exercise_callback_sum(start: i32, end: i32) -> i32 {
    let mut sum: c_int = 0;
    // SAFETY: sum outlives the call; trampoline only writes through the
    // pointer we handed it.
    unsafe {
        c_foreach(start, end, sum_trampoline, &mut sum as *mut c_int as *mut c_void);
    }
    sum
}

// Silence unused-import warning for c_uint in case of future edits.
#[allow(dead_code)]
fn _unused(_x: c_uint) {}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("=== FFI BINDINGS EXERCISE ===\n");

    println!("c_add(2, 3) = {}", call_c_add(2, 3));

    let s = CString::new("hello ffi").unwrap();
    println!("c_strlen(\"{}\") = {}", s.to_str().unwrap(), call_c_strlen(&s));

    println!("opaque counter ends at {}", exercise_opaque_handle());
    println!("repr(C) point distance^2 = {}", exercise_repr_c_struct());
    println!("callback sum 0..5 = {}", exercise_callback_sum(0, 5));

    println!("\n=== ALL FFI EXERCISES COMPLETE ===");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c_add() {
        assert_eq!(call_c_add(20, 22), 42);
    }

    #[test]
    fn test_c_strlen() {
        let s = CString::new("rust").unwrap();
        assert_eq!(call_c_strlen(&s), 4);
    }

    #[test]
    fn test_opaque_handle() {
        assert_eq!(exercise_opaque_handle(), 12);
    }

    #[test]
    fn test_repr_c() {
        assert_eq!(exercise_repr_c_struct(), 25.0);
    }

    #[test]
    fn test_callback_sum() {
        assert_eq!(exercise_callback_sum(1, 6), 15); // 1+2+3+4+5
    }
}
