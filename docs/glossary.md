# Rust Systems Programming Glossary

A comprehensive glossary of Rust and systems programming concepts used throughout this curriculum.

## A

### Associated Types
Type placeholders in traits that concrete implementations define.
```rust
pub trait Iterator {
    type Item;  // Associated type
}

impl Iterator for MyType {
    type Item = i32;  // Concrete type
}
```
**Why:** Allows traits to define relationships between types without lifetime parameters.

### Atomic
Type providing thread-safe access to shared data using CPU-level operations.
```rust
use std::sync::atomic::{AtomicUsize, Ordering};
let counter = AtomicUsize::new(0);
counter.fetch_add(1, Ordering::SeqCst);
```
**Use:** When you need lock-free concurrency for small data.

---

## B

### Borrow
Temporary access to a value through a reference, without taking ownership.
```rust
let s = String::from("hello");
let r = &s;  // Borrow s
```
**Key:** Owner still owns; borrower has temporary access.

### Borrow Checker
Rust's compile-time mechanism that enforces borrowing rules.
- One mutable borrow OR many immutable borrows (not both)
- References cannot outlive what they reference
- Prevents data races and use-after-free

### Box<T>
Heap-allocated pointer to a single value (single ownership).
```rust
let b = Box::new(5);
let x: i32 = *b;
```
**Use:** When you need heap allocation or trait objects.

---

## C

### Closure
Anonymous function that captures variables from its environment.
```rust
let x = 5;
let f = || println!("{}", x);  // Captures x
f();
```
**Types:**
- `Fn`: borrows immutably
- `FnMut`: borrows mutably
- `FnOnce`: takes ownership

### Copy
Trait allowing types to be implicitly duplicated.
```rust
let x = 5;
let y = x;  // x is copied, both valid
```
**Applies to:** Integers, floats, bools, tuples of Copy types.

### Covariance
Type relationship where subtype can be used where supertype expected.
```rust
let cat: Box<Cat> = Box::new(Cat);
let animal: Box<dyn Animal> = cat;  // Covariant
```

---

## D

### Derive
Macro automatically implementing standard traits.
```rust
#[derive(Debug, Clone, PartialEq)]
struct Point { x: i32, y: i32 }
```
**Common:** Debug, Clone, Default, Eq, Ord, Hash

### Drop
Trait called when a value goes out of scope, enabling cleanup.
```rust
impl Drop for MyType {
    fn drop(&mut self) {
        // Cleanup code
    }
}
```
**Automatic:** Rust calls drop() when variable scope ends.

---

## E

### Enum
Type with multiple variants, only one can be active.
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
**Use:** Representing alternatives; Option and Result are enums.

### Error Trait
Standard trait for error types providing error chain.
```rust
pub trait Error: Debug + Display {
    fn source(&self) -> Option<&(dyn Error + 'static)>;
}
```

---

## F

### FFI (Foreign Function Interface)
Calling C/C++ code from Rust or vice versa.
```rust
extern "C" {
    fn c_function(x: i32) -> i32;
}

unsafe { c_function(42) }
```

### Future
Trait representing asynchronous computation.
```rust
pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output>;
}
```
**Use:** Foundation of async/await; can be polled to completion.

---

## G

### Generic
Type parameter allowing code to work with any type.
```rust
fn largest<T: Ord>(list: &[T]) -> &T {
    // Works with any T that implements Ord
}
```
**Benefit:** Code reuse with type safety via monomorphization.

---

## H

### HRTB (Higher-Ranked Trait Bound)
Trait bound quantified over lifetimes.
```rust
fn takes_func<F>(f: F)
where
    F: for<'a> Fn(&'a str) -> bool,
{
    f("hello");
}
```
**Use:** Specifying that closure works with any lifetime.

### Heap
Dynamic memory where size unknown at compile time.
```rust
let s = String::from("hello");  // Heap allocation
```
**vs Stack:** Slower, flexible size, explicit/automatic cleanup.

---

## I

### Interior Mutability
Mutation through shared (immutable) reference.
```rust
let data = RefCell::new(5);
*data.borrow_mut() = 10;  // Mutate through & ref
```
**Types:** RefCell (single-threaded), Mutex (multi-threaded)

### Invariance
Type relationship where subtype cannot be used for supertype.
```rust
let mut v: Vec<i32> = vec![1, 2, 3];
let r: &mut Vec<i32> = &mut v;
// Cannot treat as &mut Vec<Number> - invariant
```

### Item
Element yielded by an iterator.
```rust
impl Iterator for MyIter {
    type Item = i32;  // Items are i32
}
```

---

## L

### Lifetime
Duration that a reference is valid.
```rust
fn first_word<'a>(s: &'a str) -> &'a str {
    &s[..s.len()]
}
```
**Elision:** Compiler infers lifetimes in common cases.

### Lifetime Elision
Rules allowing lifetime omission in common patterns.
```rust
fn foo(s: &str) -> &str {  // Lifetimes elided
    s
}
// Inferred as: fn foo<'a>(s: &'a str) -> &'a str
```

### Lint
Compiler warning about style or potential issues.
**Tool:** `cargo clippy` for additional lints.

---

## M

### Macro
Code that generates code at compile-time.
```rust
vec![1, 2, 3]  // vec! macro expands to Vec::new() + push calls
```
**Types:** Declarative (macro_rules!), Procedural (derive, attr, func)

### Monomorphization
Compiler generating separate code for each generic type.
```rust
fn generic<T>(x: T) { }

generic(5);      // Generates generic::<i32>
generic("hi");   // Generates generic::<&str>
```
**Cost:** Larger binary, faster execution (specialized code).

### Mutex
Mutual exclusion lock for thread-safe interior mutability.
```rust
let data = Mutex::new(vec![1, 2, 3]);
data.lock().unwrap().push(4);
```
**Cost:** Blocking, OS-level synchronization.

---

## O

### Option<T>
Represents optional value: Some(T) or None.
```rust
fn find(list: &[i32]) -> Option<i32> {
    if list.is_empty() { None } else { Some(list[0]) }
}
```
**Use:** When value might not exist (not an error).

### Ownership
Responsibility for value cleanup; transferred via move.
```rust
let s1 = String::from("hello");
let s2 = s1;  // Ownership moved, s1 invalid
```
**Three rules:**
1. Each value has one owner
2. Ownership can be transferred
3. Value dropped when owner goes out of scope

---

## P

### Pattern Matching
Destructuring values by shape.
```rust
match x {
    Some(v) => println!("{}", v),
    None => println!("Nothing"),
}
```

### Pin<P>
Wrapper guaranteeing value won't move in memory.
```rust
fn poll(self: Pin<&mut Self>) { }  // self cannot move
```
**Use:** Required for async/await to safely use self-referential structs.

### Poison
State of a Mutex indicating a panic occurred during lock.
```rust
let data = Mutex::new(5);
match data.lock() {
    Ok(guard) => { /* ... */ }
    Err(e) => { /* Mutex was poisoned */ }
}
```

### Procedural Macro
Macro operating on token streams to generate code.
```rust
#[derive(Debug)]  // Procedural macro
struct Point { x: i32, y: i32 }
```

### Projection
Accessing sub-value of a pinned value.
```rust
let mut x = Box::pin(5);
let _: Pin<&mut i32> = x.as_mut();
```

---

## R

### Rc<T>
Reference-counted pointer for shared single-threaded ownership.
```rust
let rc1 = Rc::new(5);
let rc2 = Rc::clone(&rc1);  // Shared ownership
```
**Limit:** Single-threaded only; use Arc for multi-threaded.

### Repr
Attribute controlling struct/enum memory layout.
```rust
#[repr(C)]  // C-compatible layout
struct Point { x: i32, y: i32 }
```
**Types:** C (C layout), transparent, packed, align(n)

### Result<T, E>
Represents success (Ok) or error (Err).
```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 { Err("divide by zero".into()) } else { Ok(a/b) }
}
```
**Use:** When operation can fail with recoverable error.

---

## S

### Safe Rust
Code guaranteed memory-safe by compiler.
- No data races
- No use-after-free
- No buffer overflows

### Slice
Reference to contiguous sequence.
```rust
let v = vec![1, 2, 3];
let s = &v[1..];  // Slice of v
```
**Types:** &[T] (immutable), &mut [T] (mutable)

### Smart Pointer
Pointer with additional metadata/behavior.
```rust
Box<T>, Rc<T>, Arc<T>, Mutex<T>
```

### Stack
Fixed-size memory, allocated at compile-time.
```rust
let x = 5;  // Stored on stack
```
**vs Heap:** Faster, limited size, automatic cleanup.

### Trait
Set of methods that types can implement.
```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```
**Use:** Defining interfaces; trait objects for dynamic dispatch.

---

## U

### Unsafe Rust
Code that compiler doesn't verify for memory safety.
```rust
unsafe {
    let x = ptr as *mut T;
    *x = value;
}
```
**Use:** Low-level operations; must verify safety manually.

### UB (Undefined Behavior)
Program behavior not specified by language.
```rust
// Examples: dereference null, data race, out-of-bounds
```
**In Rust:** Unsafe code can cause UB; safe code cannot.

---

## V

### Variance
Type relationship rules for generics.
- **Covariance:** Subtype OK where supertype needed
- **Contravariance:** Reversed relationship
- **Invariance:** No relationship
```rust
&T is covariant in T
&mut T is invariant in T
fn(&T) is contravariant in T
```

---

## W

### Waker
Notification system for async runtimes.
```rust
pub trait Wake {
    fn wake(self: Arc<Self>);
}
```
**Use:** Async runtime knows when to poll Future again.

### Where Clause
Alternative syntax for trait bounds.
```rust
fn foo<T>(x: T) where T: Display {
    println!("{}", x);
}
```

---

## Y

### Yield
(Not applicable in Rust; see async/await instead)

---

## Z

### Zero-Cost Abstraction
Abstraction with no runtime overhead.
```rust
let v: Vec<i32> = (1..=3).collect();
// Compiles to equivalent hand-written code
```
**Rust Goal:** Safe + Fast = Zero-cost abstractions

---

## Common Acronyms

| Acronym | Meaning |
|---------|---------|
| **RAII** | Resource Acquisition Is Initialization |
| **FFI** | Foreign Function Interface |
| **HRTB** | Higher-Ranked Trait Bound |
| **UB** | Undefined Behavior |
| **UPS** | Universal Pointer Size |
| **DST** | Dynamically Sized Type |
| **RFC** | Request For Comments (Rust proposals) |
| **MIR** | Mid-level Intermediate Representation |
| **LLVM** | Low-Level Virtual Machine |

