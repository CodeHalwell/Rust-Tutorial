/// ============================================================================
/// BORROW CHECKER CHALLENGES: Fix the Broken Code
/// ============================================================================
///
/// Each exercise below is intentionally broken. Your job:
/// 1. Read the comment describing what the code should do
/// 2. Predict WHY it doesn't compile
/// 3. Fix it (solutions are shown in comments + working functions below)
///
/// Run: rustc --edition 2021 02-borrow-checker-challenges.rs -o bc && ./bc

// ============================================================================
// CHALLENGE 1: Two Mutable Borrows
// ============================================================================

/// Broken:
/// ```rust
/// let mut v = vec![1, 2, 3];
/// let a = &mut v;
/// let b = &mut v;        // ❌ cannot borrow as mutable more than once
/// a.push(4);
/// b.push(5);
/// ```
///
/// Fix: use the borrows sequentially, not simultaneously.

pub fn challenge_1_two_mutable_borrows() {
    let mut v = vec![1, 2, 3];
    {
        let a = &mut v;
        a.push(4);
    } // a's borrow ends here
    let b = &mut v;
    b.push(5);
    assert_eq!(v, vec![1, 2, 3, 4, 5]);
    println!("Challenge 1 fixed: {:?}", v);
}

// ============================================================================
// CHALLENGE 2: Mutable + Immutable Borrow
// ============================================================================

/// Broken:
/// ```rust
/// let mut v = vec![1, 2, 3];
/// let first = &v[0];
/// v.push(4);             // ❌ cannot borrow as mutable while immutable borrow lives
/// println!("{}", first);
/// ```
///
/// Fix: use the immutable borrow BEFORE the mutation (NLL: non-lexical lifetimes
/// end the borrow at last use).

pub fn challenge_2_mixed_borrows() {
    let mut v = vec![1, 2, 3];
    let first = v[0]; // copy the value out instead of holding a reference
    v.push(4);
    assert_eq!(first, 1);
    assert_eq!(v.len(), 4);
    println!("Challenge 2 fixed: first={}, v={:?}", first, v);
}

// ============================================================================
// CHALLENGE 3: Returning a Reference to a Local
// ============================================================================

/// Broken:
/// ```rust
/// fn dangle() -> &String {
///     let s = String::from("hello");
///     &s   // ❌ returns reference to data dropped at end of scope
/// }
/// ```
///
/// Fix: return ownership instead of a reference.

pub fn dangle_fixed() -> String {
    let s = String::from("hello");
    s // ownership moves out; caller owns the String
}

pub fn challenge_3_dangling_reference() {
    let s = dangle_fixed();
    assert_eq!(s, "hello");
    println!("Challenge 3 fixed: {}", s);
}

// ============================================================================
// CHALLENGE 4: Use After Move
// ============================================================================

/// Broken:
/// ```rust
/// let s1 = String::from("hello");
/// let s2 = s1;
/// println!("{}", s1);   // ❌ value used after move
/// ```
///
/// Fix: clone if you need both, or borrow.

pub fn challenge_4_use_after_move() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // deep copy: both valid
    assert_eq!(s1, "hello");
    assert_eq!(s2, "hello");
    println!("Challenge 4 fixed: {} / {}", s1, s2);
}

// ============================================================================
// CHALLENGE 5: Mutating While Iterating
// ============================================================================

/// Broken:
/// ```rust
/// let mut v = vec![1, 2, 3];
/// for x in &v {
///     v.push(*x);        // ❌ cannot borrow v as mutable in the loop body
/// }
/// ```
///
/// Fix: collect the values first, or iterate over indices, or drain.

pub fn challenge_5_mutate_while_iterating() {
    let mut v = vec![1, 2, 3];
    let snapshot: Vec<i32> = v.clone();
    for x in &snapshot {
        v.push(*x);
    }
    assert_eq!(v, vec![1, 2, 3, 1, 2, 3]);
    println!("Challenge 5 fixed: {:?}", v);
}

// ============================================================================
// CHALLENGE 6: Dangling Slice from Function
// ============================================================================

/// Broken:
/// ```rust
/// fn first_word(s: &str) -> &str { ... }  // fine
/// fn bad() -> &str {
///     let s = String::from("hi there");
///     &s[..2]            // ❌ slice borrows local s which is dropped
/// }
/// ```
///
/// Fix: return an owned String or a slice tied to a parameter's lifetime.

pub fn first_two_owned(s: &str) -> String {
    s.chars().take(2).collect()
}

pub fn challenge_6_dangling_slice() {
    let s = String::from("hi there");
    let prefix = first_two_owned(&s);
    assert_eq!(prefix, "hi");
    println!("Challenge 6 fixed: {}", prefix);
}

// ============================================================================
// CHALLENGE 7: Borrow in a Loop Body
// ============================================================================

/// Broken:
/// ```rust
/// let mut v = vec![1, 2, 3];
/// for i in 0..v.len() {
///     v.push(i);         // ❌ v borrowed immutably by len() in condition + mutably in body
/// }
/// ```
///
/// Fix: capture the length once.

pub fn challenge_7_borrow_in_loop() {
    let mut v = vec![1, 2, 3];
    let n = v.len();
    for i in 0..n {
        v.push(i);
    }
    assert_eq!(v, vec![1, 2, 3, 0, 1, 2]);
    println!("Challenge 7 fixed: {:?}", v);
}

// ============================================================================
// CHALLENGE 8: Split Borrow of Struct Fields
// ============================================================================

/// This one actually COMPILES — the borrow checker tracks fields separately:
/// ```rust
/// struct Point { x: i32, y: i32 }
/// let mut p = Point { x: 1, y: 2 };
/// let a = &mut p.x;
/// let b = &mut p.y;      // ✅ different fields = disjoint borrows
/// ```

pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub fn challenge_8_split_borrows() {
    let mut p = Point { x: 1, y: 2 };
    let a = &mut p.x;
    let b = &mut p.y;
    *a += 10;
    *b += 20;
    assert_eq!((p.x, p.y), (11, 22));
    println!("Challenge 8: split borrows work -> {:?}", (p.x, p.y));
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("=== BORROW CHECKER CHALLENGES ===\n");
    challenge_1_two_mutable_borrows();
    challenge_2_mixed_borrows();
    challenge_3_dangling_reference();
    challenge_4_use_after_move();
    challenge_5_mutate_while_iterating();
    challenge_6_dangling_slice();
    challenge_7_borrow_in_loop();
    challenge_8_split_borrows();
    println!("\n=== ALL CHALLENGES PASSED ===");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_mutable_borrows_sequential() {
        let mut v = vec![1];
        v.push(2);
        v.push(3);
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn test_dangle_fixed_returns_owned() {
        assert_eq!(dangle_fixed(), "hello");
    }

    #[test]
    fn test_first_two_owned() {
        assert_eq!(first_two_owned("hello"), "he");
    }

    #[test]
    fn test_split_borrows() {
        let mut p = Point { x: 0, y: 0 };
        let a = &mut p.x;
        let b = &mut p.y;
        *a = 1;
        *b = 2;
        assert_eq!((p.x, p.y), (1, 2));
    }
}
