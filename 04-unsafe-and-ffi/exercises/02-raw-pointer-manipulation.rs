/// ============================================================================
/// RAW POINTER MANIPULATION EXERCISES
/// ============================================================================
///
/// Pointer arithmetic, slices from raw parts, swaps, and NonNull.
/// Every unsafe block carries a SAFETY comment.
///
/// Run: rustc --edition 2021 02-raw-pointer-manipulation.rs -o rp && ./rp

// ============================================================================
// EXERCISE 1: Pointer arithmetic over an array
// ============================================================================

pub fn exercise_pointer_arithmetic() -> i32 {
    let arr = [10, 20, 30, 40];
    let base = arr.as_ptr();
    unsafe {
        // SAFETY: base is valid for 4 elements; add(0..=3) stays in bounds.
        *base.add(0) + *base.add(1) + *base.add(2) + *base.add(3)
    }
}

// ============================================================================
// EXERCISE 2: Build a slice from raw parts
// ============================================================================

pub fn exercise_slice_from_raw_parts() -> i32 {
    let arr = [5, 6, 7, 8];
    let ptr = arr.as_ptr();
    let len = arr.len();
    // SAFETY: ptr is valid for len elements, properly aligned, and the
    // returned slice borrows arr which outlives the call.
    let slice: &[i32] = unsafe { std::slice::from_raw_parts(ptr, len) };
    slice.iter().sum()
}

// ============================================================================
// EXERCISE 3: Swap two values through raw pointers
// ============================================================================

pub fn swap_via_raw(a: &mut i32, b: &mut i32) {
    let pa: *mut i32 = a as *mut i32;
    let pb: *mut i32 = b as *mut i32;
    // SAFETY: pa and pb are non-null, aligned, and point to distinct,
    // initialized i32s (a and b are distinct &mut borrows).
    unsafe {
        let tmp = *pa;
        *pa = *pb;
        *pb = tmp;
    }
}

// ============================================================================
// EXERCISE 4: Reverse an array in place with two pointers
// ============================================================================

pub fn reverse_in_place(arr: &mut [i32]) {
    if arr.len() < 2 {
        return;
    }
    let base = arr.as_mut_ptr();
    let len = arr.len();
    let mut lo = 0usize;
    let mut hi = len - 1;
    while lo < hi {
        unsafe {
            // SAFETY: lo < hi < len, so both offsets are in bounds and the
            // pointers never alias (distinct indices).
            let p_lo = base.add(lo);
            let p_hi = base.add(hi);
            let tmp = *p_lo;
            *p_lo = *p_hi;
            *p_hi = tmp;
        }
        lo += 1;
        hi -= 1;
    }
}

// ============================================================================
// EXERCISE 5: NonNull — a non-null pointer with ownership flavor
// ============================================================================

use std::ptr::NonNull;

pub struct Node {
    pub value: i32,
    pub next: Option<NonNull<Node>>,
}

pub fn exercise_nonnull_list() -> i32 {
    let mut a = Box::new(Node { value: 1, next: None });
    let mut b = Box::new(Node { value: 2, next: None });
    // Link a -> b using NonNull.
    a.next = NonNull::new(b.as_mut() as *mut Node);
    // Walk the list.
    let mut sum = 0;
    let mut cur = NonNull::new(a.as_mut() as *mut Node);
    while let Some(node) = cur {
        // SAFETY: nodes are live Boxes owned by this scope; we only read.
        sum += unsafe { node.as_ref() }.value;
        cur = unsafe { node.as_ref() }.next;
    }
    sum
}

// ============================================================================
// EXERCISE 6: offset_from — distance between pointers
// ============================================================================

pub fn exercise_offset_from() -> isize {
    let arr = [1, 2, 3, 4, 5];
    let p = arr.as_ptr();
    unsafe {
        // SAFETY: both pointers are within the same array allocation.
        p.add(4).offset_from(p)
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("=== RAW POINTER MANIPULATION ===\n");

    println!("pointer arithmetic sum: {}", exercise_pointer_arithmetic());
    println!("slice from raw parts sum: {}", exercise_slice_from_raw_parts());

    let mut x = 1;
    let mut y = 2;
    swap_via_raw(&mut x, &mut y);
    println!("after swap: x={}, y={}", x, y);

    let mut v = vec![1, 2, 3, 4, 5];
    reverse_in_place(&mut v);
    println!("reversed: {:?}", v);

    println!("NonNull list sum: {}", exercise_nonnull_list());
    println!("offset_from: {}", exercise_offset_from());

    println!("\n=== ALL RAW POINTER EXERCISES COMPLETE ===");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pointer_arithmetic() {
        assert_eq!(exercise_pointer_arithmetic(), 100);
    }

    #[test]
    fn test_slice_from_raw_parts() {
        assert_eq!(exercise_slice_from_raw_parts(), 26);
    }

    #[test]
    fn test_swap() {
        let mut a = 1;
        let mut b = 2;
        swap_via_raw(&mut a, &mut b);
        assert_eq!((a, b), (2, 1));
    }

    #[test]
    fn test_reverse() {
        let mut v = vec![1, 2, 3];
        reverse_in_place(&mut v);
        assert_eq!(v, vec![3, 2, 1]);
        let mut empty: Vec<i32> = vec![];
        reverse_in_place(&mut empty);
        assert!(empty.is_empty());
    }

    #[test]
    fn test_nonnull_list() {
        assert_eq!(exercise_nonnull_list(), 3);
    }

    #[test]
    fn test_offset_from() {
        assert_eq!(exercise_offset_from(), 4);
    }
}
