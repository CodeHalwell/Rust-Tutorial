# Ownership Model: The Foundation of Rust's Safety

## Table of Contents
1. [Why Ownership?](#why-ownership)
2. [The Three Rules](#the-three-rules)
3. [Stack vs Heap](#stack-vs-heap)
4. [Moving Values](#moving-values)
5. [Copy Types](#copy-types)
6. [Drop Trait](#drop-trait)
7. [Mental Models](#mental-models)
8. [Common Pitfalls](#common-pitfalls)

---

## Why Ownership?

### The Problem

Every programming language must answer: **Who is responsible for freeing memory?**

**Garbage Collection Approach** (Python, Java, JavaScript):
- Runtime periodically frees unused memory
- Simple for programmers
- Cost: Performance penalty, unpredictable latency, overhead

**Manual Memory Management** (C, C++):
- Programmer explicitly allocates and frees
- Full control and maximum performance
- Cost: Bugs, memory leaks, use-after-free, double-free

**Rust's Approach: Ownership**:
- Compiler automatically frees memory based on scope
- Safety guarantees + performance of C
- Cost: Learning curve, strict rules

### The Insight

Memory safety can be guaranteed at **compile-time** without garbage collection.

```rust
// Rust figures out this is safe:
{
    let s = String::from("hello");
    println!("{}", s);
    // s goes out of scope
    // Compiler calls s.drop() automatically
    // Memory freed here
}
// s is not accessible here - compiler prevents use-after-free

// This doesn't compile:
{
    let s = String::from("hello");
}
println!("{}", s);  // ❌ ERROR: s is not in scope
```

---

## The Three Rules

### Rule 1: Each Value Has One Owner

Every value in Rust has **exactly one owner**—the variable that holds it.

```rust
let s1 = String::from("hello");  // s1 is the owner

let s2 = s1;  // Ownership TRANSFERS to s2
// s1 is no longer the owner - it's "moved"

println!("{}", s1);  // ❌ COMPILE ERROR: s1 is no longer owner
println!("{}", s2);  // ✅ OK: s2 is the owner
```

**Why one owner?**
- Guarantees exactly one code path is responsible for cleanup
- Prevents double-free errors
- Makes ownership tracking deterministic

### Rule 2: Ownership Can Be Transferred (Moved)

When you assign a value to another variable, ownership **transfers**.

```rust
let s1 = String::from("hello");
let s2 = s1;  // s1 -> s2 (move)
// Now s2 owns the data, s1 is empty

// With simple types (Copy semantics):
let x = 5;
let y = x;  // Both x and y are valid (different mechanism)
println!("{} {}", x, y);  // ✅ Both work
```

**Move semantics prevent double-free:**

```rust
// Without Rust (C):
char* s1 = malloc(10);
strcpy(s1, "hello");
char* s2 = s1;  // Both point to same memory
free(s1);  // Free the memory
free(s2);  // ❌ DOUBLE FREE - Crash!

// With Rust:
let s1 = String::from("hello");
let s2 = s1;  // s1 no longer owns data
// When s2 goes out of scope, free happens once
// s1 going out of scope does nothing (it doesn't own data)
```

### Rule 3: Ownership Ends When Variable Goes Out of Scope

When a variable goes out of scope, Rust automatically calls `drop()` on its value.

```rust
{
    let s = String::from("hello");
    println!("{}", s);
}  // s goes out of scope here
   // drop(s) is called automatically
   // Memory is freed

// s is not accessible here
```

**This applies to all scopes:**

```rust
fn main() {
    let s1 = String::from("hello");  // s1 is owner

    if true {
        let s2 = String::from("world");  // s2 is owner
        println!("{} {}", s1, s2);
    }  // s2 goes out of scope, drop(s2) called

    println!("{}", s1);  // s1 still valid
}  // s1 goes out of scope, drop(s1) called
```

---

## Stack vs Heap

Understanding ownership requires understanding stack and heap.

### Stack: Fixed-Size Data

- Fixed-size data: integers, floats, booleans, references
- Super fast: just incrementing a pointer
- Limited size: usually a few MB
- Freed automatically when scope ends

```rust
let x = 5;        // stack: [5]
let y = true;     // stack: [true]
let z = 3.14;     // stack: [3.14]

// All freed when scope ends (instant, zero cost)
```

### Heap: Dynamic-Size Data

- Variable-size data: String, Vec, HashMap, custom structs
- Slower: requires allocation, indirection through pointer
- Large capacity: depends on system memory
- Must be explicitly freed (or Rust frees via Drop)

```rust
let s = String::from("hello");
// Stack:  [ptr to heap, capacity, length]
// Heap:   [h][e][l][l][o]
//         ^
//         ptr points here

// When s goes out of scope:
// 1. Drop is called
// 2. Pointer tells us where heap data is
// 3. Heap memory is freed
// 4. Stack space is freed
```

### Visual Example

```
Stack Memory:
┌────────────────────┐
│ s1 (String)        │
│ ├─ ptr: 0x1000  ───┼──→ Heap Memory
│ ├─ len: 5          │      ┌─────────────┐
│ └─ capacity: 5     │      │ [h][e][l][l]│
└────────────────────┘      │ [o][?][?][?]│
                            └─────────────┘

After move (let s2 = s1):
┌────────────────────┐
│ s1 (invalid)       │
│ (cannot use)       │
└────────────────────┘

┌────────────────────┐
│ s2 (String)        │
│ ├─ ptr: 0x1000  ───┼──→ Same Heap Memory
│ ├─ len: 5          │      ┌─────────────┐
│ └─ capacity: 5     │      │ [h][e][l][l]│
└────────────────────┘      │ [o][?][?][?]│
                            └─────────────┘
```

---

## Moving Values

### What is a Move?

A move is when ownership transfers from one variable to another. The original variable becomes invalid.

```rust
let s1 = String::from("hello");  // s1 owns the data
let s2 = s1;                      // Ownership moves to s2
                                 // s1 is now invalid

// These are errors:
println!("{}", s1);  // ❌ s1 was moved
let _s3 = s1;        // ❌ s1 was moved
```

### Moves in Function Calls

When you pass a value to a function, ownership transfers to the function.

```rust
fn takes_ownership(s: String) {
    println!("{}", s);
}  // s goes out of scope, drop(s) called

let s = String::from("hello");
takes_ownership(s);  // Ownership moves into function
println!("{}", s);   // ❌ ERROR: s was moved
```

To keep ownership, use **borrowing** (next chapter).

### Moves in Assignments

Assignment moves the value.

```rust
let s1 = String::from("hello");
let s2 = s1;  // s1 moved to s2

let vec1 = vec![1, 2, 3];
let vec2 = vec1;  // vec1 moved to vec2
```

### Why Moves Are Important

Moves prevent use-after-free bugs:

```rust
// C++:
std::string s1 = "hello";
std::string s2 = s1;  // Both own data!
// Destructor runs twice when both go out of scope
// ❌ Double-free bug

// Rust:
let s1 = String::from("hello");
let s2 = s1;  // Compile error or move
// Only one path to free
// ✅ Safe
```

---

## Copy Types

Some types are **safe to implicitly copy** because they're small and don't own heap data.

### Copy Semantics

```rust
let x = 5;
let y = x;  // x is COPIED to y, not moved
println!("{} {}", x, y);  // ✅ Both valid
```

This works because integers:
- Are fixed-size (known at compile time)
- Live on the stack
- Have no pointers to heap data
- Copying is cheap (just copying bits)

### Copy Types

Rust makes these types Copy automatically:
- All integers: `i32`, `u64`, `isize`, etc.
- All floats: `f32`, `f64`
- Booleans: `bool`
- Characters: `char`
- Tuples of Copy types: `(i32, bool)`

```rust
let x = (5, true, 3.14);  // Tuple of Copy types
let y = x;  // Implicitly copied
println!("{:?} {:?}", x, y);  // ✅ Both valid
```

### Non-Copy Types

These types **cannot be implicitly copied**:
- `String`: owns heap data
- `Vec<T>`: owns heap data
- `HashMap`: owns heap data
- Custom structs without Copy

```rust
#[derive(Clone)]
struct Person {
    name: String,  // Owns heap data -> not Copy
    age: u32,      // Copy type
}

let p1 = Person { name: String::from("Alice"), age: 30 };
let p2 = p1;  // Move, not copy

println!("{:?}", p1);  // ❌ ERROR: p1 was moved
```

### Clone: Explicit Copy

For non-Copy types, use `.clone()` to make an explicit copy:

```rust
let s1 = String::from("hello");
let s2 = s1.clone();  // Explicit copy

println!("{} {}", s1, s2);  // ✅ Both valid
```

**Note:** `.clone()` is explicit and potentially expensive (heap allocation). Moves are free.

---

## Drop Trait

The `Drop` trait defines what happens when a value goes out of scope.

### Standard Drop Behavior

For `String`:
1. Drop is called automatically
2. Heap memory is freed
3. Stack space is freed

```rust
{
    let s = String::from("hello");
    // s owns heap data
}  // drop(s) is called
   // Heap is freed, then stack space
```

### Implementing Drop

You can implement custom cleanup:

```rust
struct MyFile {
    path: String,
    handle: i32,  // File descriptor
}

impl Drop for MyFile {
    fn drop(&mut self) {
        println!("Closing file: {}", self.path);
        // Close the file handle
        // This runs automatically when the value goes out of scope
    }
}

fn main() {
    let f = MyFile {
        path: "/tmp/data".to_string(),
        handle: 3,
    };
    println!("File opened");
    // File will be closed automatically here
}
```

### RAII Pattern

**Resource Acquisition Is Initialization**: Resources are acquired in `new()` and released in `drop()`.

```rust
struct DatabaseConnection {
    connection: i32,
}

impl DatabaseConnection {
    fn new(db_name: &str) -> Self {
        println!("Connecting to {}", db_name);
        DatabaseConnection { connection: 42 }
    }
}

impl Drop for DatabaseConnection {
    fn drop(&mut self) {
        println!("Disconnecting from database");
    }
}

fn main() {
    {
        let _db = DatabaseConnection::new("postgres");
        // Do database operations
    }  // Drop is called, connection closed
    // Guaranteed cleanup even if error occurs!
}
```

---

## Mental Models

### Model 1: Ownership as "Responsibility"

```
Ownership = Responsibility for cleanup

When you own a value:
- You can read it
- You can modify it
- You're responsible for freeing it when done

When you give away ownership:
- You no longer have any responsibility
- The new owner takes over
- You cannot use the value anymore
```

### Model 2: Ownership as "Keys"

```
Only the current owner has the "key" to the value

let s1 = String::from("hello");
// s1 has the key

let s2 = s1;
// s2 now has the key
// s1's key is taken away

// Compiler enforces: only one key exists at a time
```

### Model 3: Ownership Chain

```
Ownership can transfer through a chain:

let s1 = String::from("hello");     // s1 owns
let s2 = s1;                        // s2 owns (s1 loses)
let s3 = s2;                        // s3 owns (s2 loses)
function_takes_ownership(s3);       // Function owns (s3 loses)
// Function's local variable goes out of scope
// Memory is freed

Exactly one free() call happens!
```

---

## Common Pitfalls

### Pitfall 1: Using After Move

```rust
let s1 = String::from("hello");
let s2 = s1;
println!("{}", s1);  // ❌ ERROR: value used after move
```

**Solution:** Clone if you need both, or use borrowing.

```rust
let s1 = String::from("hello");
let s2 = s1.clone();
println!("{} {}", s1, s2);  // ✅ Works
```

### Pitfall 2: Passing to Function Loses Ownership

```rust
fn print_string(s: String) {
    println!("{}", s);
}

let s = String::from("hello");
print_string(s);
println!("{}", s);  // ❌ ERROR: s was moved
```

**Solution:** Use references (borrowing).

```rust
fn print_string(s: &String) {
    println!("{}", s);
}

let s = String::from("hello");
print_string(&s);
println!("{}", s);  // ✅ Works - s not moved
```

### Pitfall 3: Assuming Copy Semantics

```rust
let s1 = String::from("hello");
let s2 = s1;  // ❌ Move, not copy
let s3 = s1;  // ❌ ERROR: s1 already moved
```

**Solution:** Check if type is Copy. Integers and simple types are.

```rust
let x = 5;
let y = x;  // ✅ Copy semantics
let z = x;  // ✅ Still valid
```

### Pitfall 4: Forgetting Drop

```rust
let f = open_file("data.txt");
if some_condition {
    return;  // File not closed!
}
```

**Solution:** Rust's Drop trait guarantees cleanup even on early return.

```rust
let f = open_file("data.txt");  // Implements Drop
if some_condition {
    return;  // f is dropped before returning
}  // File guaranteed to be closed
```

---

## Summary

| Concept | Key Idea | Example |
|---------|----------|---------|
| **Ownership** | One owner per value | `let s = String::from("hi")` |
| **Move** | Ownership transfers | `let s2 = s1; // s1 invalid` |
| **Copy** | Implicit duplication | `let y = x; // x still valid` |
| **Drop** | Automatic cleanup | Scope ends → `drop()` called |
| **Stack** | Fast, fixed-size | `let x = 5;` |
| **Heap** | Variable-size, slower | `let s = String::from("...");` |

The ownership system is Rust's killer feature. Understand it deeply, and everything else will fall into place.
