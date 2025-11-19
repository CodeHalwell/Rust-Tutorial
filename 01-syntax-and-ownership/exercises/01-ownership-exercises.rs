/// ============================================================================
/// OWNERSHIP EXERCISES: Master the Move, Copy, and Drop
/// ============================================================================
///
/// Work through these exercises to build deep understanding of Rust's ownership.
/// Try solving them BEFORE looking at the solutions!
///
/// How to use this file:
/// 1. Read the exercise description
/// 2. Write your solution in the function
/// 3. Run: cargo run --example 01-ownership-exercises
/// 4. Compare with the solution below

// ============================================================================
// EXERCISE 1: Identify Moves vs Copies
// ============================================================================

/// Exercise 1.1: Which of these lines will compile?
/// Predict before running!
///
/// ```rust
/// let x = 5;
/// let y = x;
/// let z = x;
/// println!("{} {} {}", x, y, z);  // Will this compile?
/// ```
///
/// Answer: YES - integers are Copy types
/// - x is copied to y (x still valid)
/// - x is copied to z (x still valid)
/// - All three can be printed

pub fn exercise_1_1_copy_types() {
    let x = 5;
    let y = x;      // Copy: x is NOT moved
    let z = x;      // Copy: x is still valid
    println!("Copy types: {} {} {}", x, y, z);  // ✅ All valid
}

/// Exercise 1.2: Which of these will compile?
///
/// ```rust
/// let s1 = String::from("hello");
/// let s2 = s1;
/// let s3 = s1;
/// println!("{} {} {}", s1, s2, s3);  // Will this compile?
/// ```
///
/// Answer: NO - String is not Copy
/// - s1 is MOVED to s2 (s1 becomes invalid)
/// - s1 cannot be moved to s3 (already moved)
/// - Compiler error on the s3 line

pub fn exercise_1_2_move_types() {
    let s1 = String::from("hello");
    let s2 = s1;                       // Move: s1 moved to s2
    // let s3 = s1;                    // ❌ ERROR: value used after move

    println!("Move types: {}", s2);    // ✅ Only s2 valid
}

// ============================================================================
// EXERCISE 2: Understand When Drop is Called
// ============================================================================

/// Exercise 2.1: In what order are drop calls made?
///
/// ```rust
/// let s1 = String::from("first");
/// let s2 = String::from("second");
/// let s3 = String::from("third");
/// ```
///
/// When this scope ends, in what order are drop(s1), drop(s2), drop(s3) called?
///
/// Answer: REVERSE order of creation
/// - drop(s3) - most recently created
/// - drop(s2)
/// - drop(s1) - first created
///
/// This is LIFO (Last In, First Out)

pub fn exercise_2_1_drop_order() {
    {
        let s1 = String::from("first");
        println!("Created s1");

        let s2 = String::from("second");
        println!("Created s2");

        let s3 = String::from("third");
        println!("Created s3");

        // When scope ends, drop called in reverse:
        // drop(s3)
        // drop(s2)
        // drop(s1)
    }
    println!("All dropped in reverse order");
}

/// Exercise 2.2: Understanding Move in Drop Order
///
/// When a value is moved, the original variable's drop does nothing.
///
/// ```rust
/// let s1 = String::from("hello");
/// let s2 = s1;  // s1 moved to s2
/// // When scope ends:
/// // - drop(s2) frees the heap
/// // - drop(s1) does nothing (s1 doesn't own the data anymore)
/// ```

pub fn exercise_2_2_moved_drop_order() {
    {
        let s1 = String::from("hello");
        println!("s1 owns the string");

        let s2 = s1;  // Ownership moved
        println!("s2 owns the string now");

        // At end of scope:
        // - drop(s2) frees the heap memory
        // - drop(s1) does nothing (doesn't own)
        // Result: Memory freed exactly once ✅
    }
    println!("Memory freed once");
}

// ============================================================================
// EXERCISE 3: Heap vs Stack
// ============================================================================

/// Exercise 3.1: Identify which data lives on stack vs heap
///
/// For each variable, say: Stack or Heap?
///
/// let x = 5;                     // Stack (fixed-size i32)
/// let s = String::from("hi");    // Stack (String struct) + Heap (data)
/// let v = vec![1, 2, 3];         // Stack (Vec struct) + Heap (data)
/// let b = true;                  // Stack (bool)

pub fn exercise_3_1_stack_vs_heap() {
    println!("Stack vs Heap Analysis:");

    let x = 5;  // ✅ Stack only
    println!("x = 5: {} bytes on stack", std::mem::size_of_val(&x));

    let s = String::from("hello");  // ✅ String struct on stack, data on heap
    println!("String 'hello': {} bytes for String struct on stack",
             std::mem::size_of_val(&s));

    let v = vec![1, 2, 3];  // ✅ Vec struct on stack, data on heap
    println!("Vec [1,2,3]: {} bytes for Vec struct on stack",
             std::mem::size_of_val(&v));

    let b = true;  // ✅ Stack only
    println!("true: {} bytes on stack", std::mem::size_of_val(&b));
}

// ============================================================================
// EXERCISE 4: Function Ownership Transfer
// ============================================================================

/// Helper function that takes ownership
fn takes_ownership(s: String) {
    println!("Function received: {}", s);
}  // s dropped here

/// Helper function that returns ownership
fn returns_ownership() -> String {
    String::from("returned value")
}

/// Exercise 4.1: Understanding function ownership transfer
///
/// When you pass a value to a function, ownership transfers to the function.
/// When the function ends, the value is dropped.

pub fn exercise_4_1_function_ownership() {
    let s = String::from("hello");
    println!("Before function call: {}", s);

    takes_ownership(s);  // Ownership moves into function
    // println!("{}", s);  // ❌ ERROR: s was moved

    // After the function call, s is no longer valid
    // The String was dropped inside the function
}

/// Exercise 4.2: Function returns ownership
///
/// To use a value after a function call, the function must return ownership.

pub fn exercise_4_2_return_ownership() {
    let s1 = returns_ownership();
    println!("Returned: {}", s1);

    let s2 = String::from("hello");
    let s3 = returns_owned_string_or_passed(s2);
    // s2 was moved to function, but we get ownership back
    println!("Returned: {}", s3);
}

fn returns_owned_string_or_passed(s: String) -> String {
    // Do something with s
    s  // Return ownership back to caller
}

// ============================================================================
// EXERCISE 5: Copy vs Non-Copy
// ============================================================================

/// Exercise 5.1: Which types are Copy?
///
/// Copy types (safely copyable, no ownership issues):
/// - All numeric: i32, u64, f32, f64, etc.
/// - bool, char
/// - Tuples of Copy types: (i32, bool)
///
/// Non-Copy types (own resources, must be moved):
/// - String
/// - Vec<T>
/// - HashMap, etc.

pub fn exercise_5_1_copy_behavior() {
    // Copy types
    let x: i32 = 5;
    let y = x;
    println!("Copy: x={}, y={}", x, y);  // ✅ Both valid

    // Tuple of Copy types
    let t1 = (5, true, 3.14);
    let t2 = t1;
    println!("Tuple copy: {:?}", (t1, t2));  // ✅ Both valid
}

/// Exercise 5.2: Why are Copy types safe to copy?
///
/// Answer: They don't own any resources that need cleanup
/// - No heap allocations
/// - Fixed size (compiler knows it's safe to bitwise copy)
/// - Implementing Copy allows implicit duplication

#[derive(Copy, Clone)]  // Copy requires Clone
struct Point {
    x: i32,
    y: i32,
}

pub fn exercise_5_2_custom_copy() {
    let p1 = Point { x: 0, y: 5 };
    let p2 = p1;  // Implicitly copied (Copy trait)

    println!("Points: {:?} {:?}", p1, p2);  // ✅ Both valid
}

// ============================================================================
// EXERCISE 6: Understanding Ownership Chains
// ============================================================================

/// Exercise 6.1: Track ownership through assignments
///
/// ```
/// let s1 = String::from("hello");  // s1 owns
/// let s2 = s1;                     // s1 → s2
/// let s3 = s2;                     // s2 → s3
/// function_call(s3);               // s3 → function
/// // Function's scope ends, s3 dropped
/// // Only one drop() call happens!
/// ```

pub fn exercise_6_1_ownership_chain() {
    let s1 = String::from("hello");
    println!("s1 owns: {}", s1);

    let s2 = s1;  // Ownership transfers s1 → s2
    println!("s2 owns: {}", s2);

    let s3 = s2;  // Ownership transfers s2 → s3
    println!("s3 owns: {}", s3);

    function_takes_string(s3);  // Ownership transfers s3 → function

    // All variables out of scope, but memory freed only once!
    println!("Chain complete");
}

fn function_takes_string(s: String) {
    println!("Function received: {}", s);
    // s dropped here
}

// ============================================================================
// EXERCISE 7: Clone vs Move
// ============================================================================

/// Exercise 7.1: When should you use clone()?
///
/// - Use clone() when you need both the original AND a copy
/// - clone() is explicit (you see the cost)
/// - clone() works for non-Copy types
/// - Downside: clone() is expensive for large strings/vectors

pub fn exercise_7_1_clone_vs_move() {
    let s1 = String::from("hello");

    // Option 1: Move (s1 becomes invalid)
    let s2 = s1;
    // println!("{}", s1);  // ❌ s1 invalid

    // Option 2: Clone (both valid, explicit cost)
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("Both valid: {} {}", s1, s2);  // ✅ Both work
}

/// Exercise 7.2: Cloning in loops
///
/// Be careful with clone in tight loops - it's expensive!

pub fn exercise_7_2_clone_in_loop() {
    let template = String::from("Item");
    let mut results = Vec::new();

    for i in 0..3 {
        results.push(format!("{} {}", template, i));
        // Note: format! creates a new String each time
    }

    println!("Results: {:?}", results);
}

// ============================================================================
// EXERCISE 8: Real-World Ownership Scenario
// ============================================================================

struct FileHandle {
    name: String,
    data: Vec<u8>,
}

impl Drop for FileHandle {
    fn drop(&mut self) {
        println!("Closing file: {}", self.name);
    }
}

/// Exercise 8.1: Ownership of complex types
///
/// When you create a struct with owned fields, the struct owns them.
/// When the struct is dropped, all fields are dropped.

pub fn exercise_8_1_complex_ownership() {
    {
        let file1 = FileHandle {
            name: "file1.txt".to_string(),
            data: vec![1, 2, 3, 4, 5],
        };
        println!("File: {}", file1.name);

        let file2 = FileHandle {
            name: "file2.txt".to_string(),
            data: vec![10, 20, 30],
        };
        println!("File: {}", file2.name);

        // When scope ends:
        // drop(file2) - closes file2.txt
        // drop(file1) - closes file1.txt
    }
    println!("All files closed");
}

/// Exercise 8.2: Moving ownership of struct
///
/// When you move a struct, all its owned fields move too.

pub fn exercise_8_2_move_struct() {
    let file = FileHandle {
        name: "data.txt".to_string(),
        data: vec![1, 2, 3],
    };

    process_file(file);  // file moved here
    // println!("{}", file.name);  // ❌ file was moved

    println!("File processed");
}

fn process_file(f: FileHandle) {
    println!("Processing: {}", f.name);
    // f dropped here, file closed
}

// ============================================================================
// MAIN DEMO
// ============================================================================

fn main() {
    println!("=== OWNERSHIP EXERCISES ===\n");

    println!("1. Copy Types:");
    exercise_1_1_copy_types();
    println!();

    println!("2. Move Types:");
    exercise_1_2_move_types();
    println!();

    println!("3. Drop Order:");
    exercise_2_1_drop_order();
    println!();

    println!("4. Move in Drop:");
    exercise_2_2_moved_drop_order();
    println!();

    println!("5. Stack vs Heap:");
    exercise_3_1_stack_vs_heap();
    println!();

    println!("6. Function Ownership:");
    exercise_4_1_function_ownership();
    println!();

    println!("7. Return Ownership:");
    exercise_4_2_return_ownership();
    println!();

    println!("8. Copy Types Deep:");
    exercise_5_1_copy_behavior();
    println!();

    println!("9. Custom Copy Type:");
    exercise_5_2_custom_copy();
    println!();

    println!("10. Ownership Chain:");
    exercise_6_1_ownership_chain();
    println!();

    println!("11. Clone vs Move:");
    exercise_7_1_clone_vs_move();
    println!();

    println!("12. Clone in Loop:");
    exercise_7_2_clone_in_loop();
    println!();

    println!("13. Complex Ownership:");
    exercise_8_1_complex_ownership();
    println!();

    println!("14. Move Struct:");
    exercise_8_2_move_struct();

    println!("\n=== ALL EXERCISES COMPLETE ===");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy_types() {
        let x = 5;
        let y = x;
        assert_eq!(x, 5);
        assert_eq!(y, 5);
    }

    #[test]
    fn test_custom_copy() {
        let p1 = Point { x: 1, y: 2 };
        let p2 = p1;
        assert_eq!(p1.x, p2.x);
    }

    #[test]
    fn test_returns_ownership() {
        let s = returns_ownership();
        assert_eq!(s, "returned value");
    }
}
