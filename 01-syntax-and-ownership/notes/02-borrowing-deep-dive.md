# Borrowing: Temporary Loans with Compile-Time Guarantees

## Table of Contents
1. [What is Borrowing?](#what-is-borrowing)
2. [Immutable Borrows](#immutable-borrows)
3. [Mutable Borrows](#mutable-borrows)
4. [The Borrow Checker Rules](#the-borrow-checker-rules)
5. [Borrowing in Functions](#borrowing-in-functions)
6. [Aliasing and Mutation](#aliasing-and-mutation)
7. [Visual Walkthrough](#visual-walkthrough)
8. [Common Patterns](#common-patterns)
9. [Mistakes and How to Fix Them](#mistakes-and-how-to-fix-them)

---

## What is Borrowing?

Borrowing is **temporary access** to a value without taking ownership.

```rust
let s1 = String::from("hello");
let s2 = &s1;  // Borrow s1 (don't take ownership)

println!("{} {}", s1, s2);  // ✅ Both still valid
// s1 still owns the data
```

### Why Borrow?

1. **Keep ownership**: Original owner can still use the value
2. **Multiple readers**: Many code paths can read the same data
3. **Flexibility**: Pass data without transferring ownership

### Borrow vs Move

```rust
// Move: ownership transfers
let s1 = String::from("hello");
let s2 = s1;  // s1 moved to s2, s1 invalid
// Can only have s2

// Borrow: owner can still use
let s1 = String::from("hello");
let s2 = &s1;  // s1 lends to s2
let s3 = &s1;  // s1 can lend to multiple borrowers
println!("{} {} {}", s1, s2, s3);  // ✅ All valid
```

---

## Immutable Borrows

An **immutable borrow** (`&T`) grants temporary read-only access.

### Creating Immutable Borrows

```rust
let s = String::from("hello");
let r = &s;  // Immutable borrow

println!("{}", r);  // ✅ Can read
// *r = String::from("world");  // ❌ Can't mutate
```

### Multiple Immutable Borrows

You can have **unlimited** immutable borrows to the same value.

```rust
let s = String::from("hello");
let r1 = &s;
let r2 = &s;
let r3 = &s;

println!("{} {} {}", r1, r2, r3);  // ✅ All can read
```

**Why unlimited?** Reading is safe—no risk of race conditions.

```rust
// Visual:
Stack:
  s ──┐
      ├──→ Heap: "hello"
  r1 ─┤
  r2 ─┤
  r3 ─┘

All three references point to same data.
No one can mutate, so safe!
```

### Dereferencing

Use `*` to dereference and access the actual value:

```rust
let s = String::from("hello");
let r = &s;

println!("{}", r);      // ✅ Works (auto-deref in println!)
println!("{}", *r);     // ✅ Explicit dereference
println!("{}", r.len());  // ✅ Method call auto-derefs
```

---

## Mutable Borrows

A **mutable borrow** (`&mut T`) grants temporary read-write access.

### Creating Mutable Borrows

```rust
let mut s = String::from("hello");  // Must be mutable
let r = &mut s;  // Mutable borrow

r.push_str(" world");
println!("{}", s);  // ✅ s is updated
```

### The Mutable Borrow Rule: Only One At A Time

You can have **only one** mutable borrow at a time.

```rust
let mut s = String::from("hello");
let r1 = &mut s;
let r2 = &mut s;  // ❌ ERROR: cannot borrow s as mutable twice

println!("{} {}", r1, r2);
```

**Why?** Mutable references allow changes. Two mutable references could:
- Cause data races
- Invalidate each other's assumptions
- Break iterator safety

```rust
let mut v = vec![1, 2, 3];
let r1 = &mut v;
let r2 = &mut v;  // ❌ ERROR

// Without this rule, what if r1 does this?
r1.push(4);  // This might reallocate the vector!
// r2's pointer is now invalid (dangling pointer)
// ❌ Memory safety violated
```

### Mutable + Immutable: Cannot Mix

When a mutable borrow exists, immutable borrows are forbidden.

```rust
let mut s = String::from("hello");
let r1 = &mut s;  // Mutable borrow
let r2 = &s;      // ❌ ERROR: cannot borrow s as immutable
                  //    while r1 is a mutable borrow

println!("{} {}", r1, r2);
```

**Why?** The immutable borrower expects the value to not change. If a mutable borrow exists, that guarantee is violated.

```rust
let mut s = String::from("hello");
let r1 = &mut s;
r1.push_str(" world");  // Mutable borrow modifies s

let r2 = &s;  // Immutable borrow
// What does r2 point to? The old value or new value?
// The guarantee is broken!

println!("{}", r2);  // Which string are we printing?
```

---

## The Borrow Checker Rules

Rust's borrow checker enforces these rules at compile-time:

### Rule 1: References Must Not Outlive the Value

```rust
fn dangling_reference() -> &String {  // ❌ ERROR
    let s = String::from("hello");
    &s  // Reference to a local variable
}  // s is dropped here
// Returning reference to dropped value!
```

The reference outlives the value it refers to. This would be a **dangling pointer**.

```rust
// ✅ Correct: Return owned value
fn safe_function() -> String {
    let s = String::from("hello");
    s  // Return ownership
}
```

### Rule 2: Only One Mutable Borrow Exists at a Time

```rust
let mut x = 5;
let r1 = &mut x;
let r2 = &mut x;  // ❌ ERROR

// But this is OK:
let r1 = &mut x;
println!("{}", r1);
let r2 = &mut x;  // ✅ OK: r1 is no longer used
println!("{}", r2);
```

### Rule 3: Either Many Immutable OR One Mutable

```rust
let mut s = String::from("hello");

let r1 = &s;
let r2 = &s;
let r3 = &mut s;  // ❌ ERROR: can't mix immutable and mutable

println!("{} {} {}", r1, r2, r3);
```

---

## Borrowing in Functions

### Function Parameters as Borrows

Instead of taking ownership, take a reference:

```rust
// Takes ownership
fn takes_ownership(s: String) {
    println!("{}", s);
}  // s dropped here

// Borrows immutably
fn borrows_immutably(s: &String) {
    println!("{}", s);
}  // s not dropped, still owned by caller

// Borrows mutably
fn borrows_mutably(s: &mut String) {
    s.push_str("!");
}  // s not dropped, still owned by caller

let s = String::from("hello");

takes_ownership(s);        // s moved
// Can't use s anymore

let mut s = String::from("hello");
borrows_immutably(&s);     // s borrowed
println!("{}", s);         // ✅ Still valid

borrows_mutably(&mut s);   // s mutably borrowed
println!("{}", s);         // ✅ Still valid (and modified)
```

### Return Values from Borrows

When returning a borrow, the reference must come from a parameter:

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];  // ✅ Reference to parameter
        }
    }
    &s[..]
}

let s = String::from("hello world");
let word = first_word(&s);
println!("{}", word);  // "hello"
```

Cannot return reference to local variable:

```rust
fn bad_function() -> &String {  // ❌ ERROR
    let s = String::from("hello");
    &s  // Reference to local
}  // s dropped, reference is dangling
```

---

## Aliasing and Mutation

This is the core rule: **You cannot have aliasing (multiple references) and mutation at the same time.**

```
ALIASING + MUTATION = ❌ Data Race / Memory Unsafety

The rule prevents:
- Two mutable references to same data (race condition)
- One mutable + one immutable (broken guarantee)
```

### Why This Rule Matters

```rust
let mut v = vec![1, 2, 3];

let r1 = &mut v[0];  // Mutable reference to first element
let r2 = &mut v;     // Mutable reference to whole vector

*r1 = 10;  // Modifies through r1
v.push(4);  // Reallocates the vector through r2!

// r1 now points to freed memory!
println!("{}", r1);  // Use-after-free ❌
```

With the borrow rules, this doesn't compile:

```rust
let mut v = vec![1, 2, 3];
let r1 = &mut v[0];
let r2 = &mut v;  // ❌ ERROR: r1 is still in use
// Compiler prevents the use-after-free!
```

---

## Visual Walkthrough

### Example 1: Multiple Immutable Borrows

```rust
let s = String::from("hello");

let r1 = &s;
let r2 = &s;
let r3 = &s;

println!("{} {} {}", r1, r2, r3);
drop(r3);
drop(r2);
drop(r1);  // All dropped
println!("{}", s);  // ✅ s still valid

// Stack visualized:
//
// Initially:
//   s: String @ 0x1000 ───┐
//   r1: &s ────────────────┼──→ Heap
//   r2: &s ────────────────┤   "hello"
//   r3: &s ────────────────┘
//
// All references point to same data
// No one can mutate → Safe!
```

### Example 2: Mutable Then Immutable

```rust
let mut s = String::from("hello");

let r1 = &mut s;
r1.push_str(" world");

// r1 scope ends here (last use above)

let r2 = &s;  // ✅ Now mutable borrow is done
println!("{}", r2);

// Stack:
//
// With r1:
//   s: String @ 0x1000
//   r1: &mut s ──→ Can modify
//
// After r1:
//   s: String @ 0x1000
//   r2: &s ──→ Can read (but not modify)
```

### Example 3: Dangling Reference

```rust
fn bad() -> &String {  // ❌ Compile error
    let s = String::from("hello");
    &s
}
// Error: lifetime mismatch
// The reference outlives the String

// Stack visualized at drop:
//   Inside function:
//     s: String @ 0x1000 ──→ Heap: "hello"
//     Return: &s
//
//   At return:
//     s is dropped
//     Heap: "hello" is freed
//     But we're returning a reference to freed memory!

// ✅ Correct:
fn good() -> String {
    let s = String::from("hello");
    s  // Return owned value, not reference
}
```

---

## Common Patterns

### Pattern 1: Borrowing in a Loop

```rust
let mut vec = vec![1, 2, 3];

for item in &mut vec {
    *item *= 2;
}

println!("{:?}", vec);  // [2, 4, 6]
```

### Pattern 2: Borrowing in Closures

```rust
let s = String::from("hello");

let print_it = || println!("{}", s);  // Borrows s
print_it();
print_it();  // ✅ Can call multiple times

println!("{}", s);  // ✅ s still valid
```

### Pattern 3: Mutable Borrowing

```rust
let mut s = String::from("hello");

let modify = || s.push_str(" world");  // Mutably borrows
modify();
// modify dropped here

println!("{}", s);  // ✅ Modified string
```

### Pattern 4: Iterator Borrows

```rust
let vec = vec![1, 2, 3];

for item in &vec {  // Borrow each item
    println!("{}", item);
}

println!("{:?}", vec);  // ✅ vec still valid

for item in &mut vec {  // Mutable borrow
    *item *= 2;
}

for item in vec {  // Consuming (ownership transfer)
    println!("{}", item);
}
// vec is moved, no longer valid
```

---

## Mistakes and How to Fix Them

### Mistake 1: Double Mutable Borrow

```rust
let mut s = String::from("hello");
let r1 = &mut s;
let r2 = &mut s;  // ❌ ERROR

// Fix: Use r1 first, then r2
let mut s = String::from("hello");
let r1 = &mut s;
r1.push_str(" world");
drop(r1);  // Explicitly end the borrow

let r2 = &mut s;
r2.push_str("!");
```

### Mistake 2: Mixing Mutable and Immutable

```rust
let mut s = String::from("hello");
let r1 = &mut s;
let r2 = &s;  // ❌ ERROR

// Fix: Don't use immutable borrow while mutable exists
let mut s = String::from("hello");
{
    let r1 = &mut s;
    r1.push_str(" world");
}  // r1 dropped

let r2 = &s;  // ✅ Now OK
println!("{}", r2);
```

### Mistake 3: Dangling Reference

```rust
fn bad() -> &String {  // ❌ ERROR
    let s = String::from("hello");
    &s
}

// Fix 1: Return owned value
fn good1() -> String {
    String::from("hello")
}

// Fix 2: Return &'static str
fn good2() -> &'static str {
    "hello"  // String literal has static lifetime
}

// Fix 3: Accept lifetime parameter (advanced)
fn good3<'a>(s: &'a String) -> &'a str {
    &s[..]
}
```

### Mistake 4: Modifying Through Immutable Borrow

```rust
let s = String::from("hello");
let r = &s;
// s.push_str("!");  // ❌ ERROR: can't mutate through immutable ref

// Fix: Make a mutable borrow
let mut s = String::from("hello");
let r = &mut s;
r.push_str("!");  // ✅ OK
```

---

## Summary

| Borrow Type | Notation | Limit | Use Case |
|-------------|----------|-------|----------|
| **Immutable** | `&T` | Unlimited | Read-only access |
| **Mutable** | `&mut T` | One at a time | Modify data |

**The Rule:** The compiler ensures:
1. References don't outlive their values
2. At most one mutable borrow exists
3. Mutable and immutable borrows don't mix

This system prevents entire categories of bugs at **compile-time** with **zero runtime cost**.
