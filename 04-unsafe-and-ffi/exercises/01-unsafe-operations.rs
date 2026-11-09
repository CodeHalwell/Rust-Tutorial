/// ============================================================================
/// UNSAFE OPERATIONS EXERCISES
/// ============================================================================
///
/// Practice the five things `unsafe` unlocks, and write SAFETY comments for
/// each unsafe block (a requirement for all real unsafe code).
///
/// Run: rustc --edition 2021 01-unsafe-operations.rs -o uo && ./uo

// ============================================================================
// EXERCISE 1: Dereference a raw pointer
// ============================================================================

pub fn exercise_deref_raw_pointer() -> i32 {
    let x = 42;
    let ptr: *const i32 = &x as *const i32;
    // SAFETY: ptr was just created from a live reference to x, is aligned
    // and non-null, and x outlives this read.
    unsafe { *ptr }
}

// ============================================================================
// EXERCISE 2: Call an unsafe function
// ============================================================================

/// An unsafe fn: callers must uphold its contract.
/// Contract: `ptr` must point to a valid, initialized `i32`.
unsafe fn read_i32(ptr: *const i32) -> i32 {
    // SAFETY: upheld by the caller's unsafe block contract.
    unsafe { *ptr }
}

pub fn exercise_call_unsafe_fn() -> i32 {
    let x = 7;
    // SAFETY: &x is a valid, initialized, aligned i32 that outlives the call.
    unsafe { read_i32(&x as *const i32) }
}

// ============================================================================
// EXERCISE 3: Mutate through a *mut pointer
// ============================================================================

pub fn exercise_mutate_through_raw() -> i32 {
    let mut x = 10;
    let ptr: *mut i32 = &mut x as *mut i32;
    // SAFETY: ptr is non-null, aligned, points to initialized memory, and no
    // other reference to x is live during the write.
    unsafe {
        *ptr += 5;
    }
    x
}

// ============================================================================
// EXERCISE 4: A safe wrapper around unsafe (the right pattern!)
// ============================================================================

/// A mini Vec-like: owns a heap buffer, exposes ONLY safe methods.
pub struct RawBuffer {
    ptr: *mut i32,
    len: usize,
}

impl RawBuffer {
    pub fn new(len: usize) -> Self {
        let layout = std::alloc::Layout::array::<i32>(len).unwrap();
        // SAFETY: layout has non-zero size (len >= 1 checked below) and the
        // allocator returns aligned memory or null.
        let ptr = unsafe { std::alloc::alloc_zeroed(layout) as *mut i32 };
        assert!(!ptr.is_null(), "allocation failed");
        RawBuffer { ptr, len }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn set(&mut self, index: usize, value: i32) {
        assert!(index < self.len, "index out of bounds");
        // SAFETY: index < len, ptr is valid for len elements, and &mut self
        // guarantees exclusivity.
        unsafe {
            *self.ptr.add(index) = value;
        }
    }

    pub fn get(&self, index: usize) -> i32 {
        assert!(index < self.len, "index out of bounds");
        // SAFETY: index < len and ptr is valid for len elements.
        unsafe { *self.ptr.add(index) }
    }
}

impl Drop for RawBuffer {
    fn drop(&mut self) {
        let layout = std::alloc::Layout::array::<i32>(self.len).unwrap();
        // SAFETY: ptr was allocated with exactly this layout in new().
        unsafe {
            std::alloc::dealloc(self.ptr as *mut u8, layout);
        }
    }
}

// ============================================================================
// EXERCISE 5: Mutating a static (with addr_of_mut! to avoid &static mut)
// ============================================================================

static mut CALL_COUNT: u32 = 0;

pub fn exercise_static_mut() -> u32 {
    // SAFETY: single-threaded demo; addr_of_mut! avoids creating a reference
    // to the static (which would be UB-prone). Real code should use atomics
    // or a Mutex for shared statics.
    unsafe {
        let p = std::ptr::addr_of_mut!(CALL_COUNT);
        *p += 1;
        *p
    }
}

// ============================================================================
// EXERCISE 6: inline const / transmute-free type punning via to_bits
// ============================================================================

pub fn exercise_float_bits() -> u64 {
    let f: f64 = 1.5;
    f.to_bits() // safe alternative to transmute for inspecting bit patterns
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("=== UNSAFE OPERATIONS EXERCISES ===\n");

    println!("deref raw pointer: {}", exercise_deref_raw_pointer());
    println!("call unsafe fn: {}", exercise_call_unsafe_fn());
    println!("mutate through raw: {}", exercise_mutate_through_raw());

    let mut buf = RawBuffer::new(4);
    buf.set(0, 10);
    buf.set(3, 40);
    println!("RawBuffer: len={}, [0]={}, [3]={}", buf.len(), buf.get(0), buf.get(3));

    println!("static mut counter: {}", exercise_static_mut());
    println!("1.5 as bits: {:#x}", exercise_float_bits());

    println!("\n=== ALL UNSAFE EXERCISES COMPLETE ===");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deref_raw_pointer() {
        assert_eq!(exercise_deref_raw_pointer(), 42);
    }

    #[test]
    fn test_call_unsafe_fn() {
        assert_eq!(exercise_call_unsafe_fn(), 7);
    }

    #[test]
    fn test_mutate_through_raw() {
        assert_eq!(exercise_mutate_through_raw(), 15);
    }

    #[test]
    fn test_raw_buffer() {
        let mut b = RawBuffer::new(3);
        b.set(1, 99);
        assert_eq!(b.get(1), 99);
        assert_eq!(b.get(0), 0); // zeroed
    }

    #[test]
    fn test_static_mut_increments() {
        let a = exercise_static_mut();
        let b = exercise_static_mut();
        assert_eq!(b, a + 1);
    }

    #[test]
    fn test_float_bits() {
        assert_eq!(exercise_float_bits(), 1.5f64.to_bits());
    }
}
