# Mental Models: Building Intuition About Rust

The difference between understanding Rust syntax and mastering Rust is developing the right mental models. This document provides visual and conceptual models for the most important ideas.

## Table of Contents
1. [Ownership as Responsibility](#ownership-as-responsibility)
2. [The Borrow Checker as a Guard](#the-borrow-checker-as-a-guard)
3. [Lifetimes as Relationships](#lifetimes-as-relationships)
4. [Types as Constraints](#types-as-constraints)
5. [Async as State Machines](#async-as-state-machines)
6. [Traits as Contracts](#traits-as-contracts)
7. [Memory Layout Visualization](#memory-layout-visualization)

---

## Ownership as Responsibility

### The Mental Model

Imagine a valuable object (like a library book):

```
OWNER: The person currently holding the book
       - Can read it, lend it to others, or keep it
       - Responsible for returning it when done
       - Cannot use after returning (no longer owner)

MOVE:  Giving the book to someone else
       - Original owner can no longer access it
       - New owner has all responsibility
       - Only one person owns at a time (no sharing)

BORROW: Temporarily lending without transferring ownership
        - Owner still responsible
        - Multiple people can read (immutable borrow)
        - Only one person can write at a time (mutable borrow)
```

### Code Mapping

```rust
let s1 = String::from("hello");     // s1 owns the book
let s2 = s1;                        // s1 gave book to s2
// println!("{}", s1);              // ERROR: s1 no longer owns

let s3 = String::from("world");     // s3 owns its book
let r1 = &s3;                       // Lend to r1 (immutably)
let r2 = &s3;                       // Lend to r2 (immutably)
println!("{} {}", r1, r2);          // Both can read
println!("{}", s3);                 // Owner can still use

let mut s4 = String::from("rust");
let r3 = &mut s4;                   // Lend to r3 (mutably)
r3.push_str("!");                   // r3 can write
// let r4 = &s4;                    // ERROR: can't lend while r3 is writing
```

### Why This Works

**Copy Semantics (Integers):**
```
Books are small (fit in hand) → can copy → multiple people have copies
```

**Move Semantics (String):**
```
Book is large (needs both hands) → can't copy → only one person has it
```

---

## The Borrow Checker as a Guard

### The Mental Model

```
The Borrow Checker is a Security Guard

RULE 1: "Either one person modifying (writer) OR many people reading (readers)"
        └─ Why? Writer might change things; readers expect immutability

RULE 2: "You can't use a reference after it becomes invalid"
        └─ Why? Would be like reading after book was destroyed

RULE 3: "A borrowed item must be returned before owner uses it"
        └─ Why? Ensure only one person actively uses it at a time
```

### Visual Guard Post

```
Library Desk (borrow checker):

  Request: "Can I borrow this book to read?"
  Guard checks: Are there any writers right now?
  ✓ Yes: Let multiple readers have copies
  ✗ No: Keep waiting

  Request: "Can I borrow this book to write?"
  Guard checks: Are there any readers OR writers?
  ✓ Yes: You must wait (someone else is using it)
  ✗ No: Take it, but no one else can touch it until you return
```

### Code Mapping

```rust
let mut data = vec![1, 2, 3];

// Multiple readers (immutable borrows)
let r1 = &data;    // ✓ Guard allows
let r2 = &data;    // ✓ Guard allows
let r3 = &data;    // ✓ Guard allows
println!("{:?}", (r1, r2, r3));  // All can read

// Now try to write while readers exist:
let w = &mut data;    // ✗ Guard blocks (readers still active)
// ERROR: cannot borrow as mutable
```

---

## Lifetimes as Relationships

### The Mental Model

Lifetimes encode **"How long does this reference stay valid?"**

Think of it as a **"Promise"**:

```
&'a String means:
  "I promise this reference will only be used as long as 'a is alive"

If you break the promise:
  Rust compiler says: "You're lying to me"
  compile error!
```

### The Timeline Visualization

```rust
// Example: First word function
fn first_word<'a>(s: &'a String) -> &'a String {
    // The returned reference lives exactly as long as s lives
    &s[..5]
}

// Timeline visualization:

let s = String::from("hello world");
|-------- 's' lifetime starts --------|
|          |
|          first_word(s) called
|          |
|          |------ 'a' lifetime (same as s) ------|
|          |                          |
|          word = &s[..5]  // returns &'a String
|          |                          |
|          |---- 'a' ends when word no longer used
|          |
println!("{}", word);
|          |
|-- s scope ends, s dropped ------

✓ Valid: word was used while s was alive
```

### Why This Matters

```rust
// ERROR: trying to return reference to temporary
fn bad() -> &String {  // ❌ Compile error
    let s = String::from("hello");
    &s
    // s goes out of scope here
    // Returning reference to destroyed value!
}

// Timeline:
// |-- function scope starts
// |    |-- s created
// |    |-- return &s (reference to s)
// |-- s destroyed
// |-- return value leaves function with dangling reference
//     ❌ Memory safety violated
```

### Lifetime Elision

```rust
// Compiler can infer lifetimes in common cases:

fn foo(s: &str) -> &str {
    s
    // Inferred as:
    // fn foo<'a>(s: &'a str) -> &'a str
}

// One input reference → output reference has same lifetime
```

---

## Types as Constraints

### The Mental Model

A type is a **contract** about valid values:

```
Type = What values are valid? What operations are allowed?

i32 says:  "I'm a 32-bit integer, range -2^31 to 2^31-1"
           "You can: add, multiply, compare"
           "You cannot: store 3 billion (overflow)"

String says: "I'm text on the heap"
             "You can: iterate, modify, borrow"
             "You cannot: treat as array without bounds checking"

&T says:   "I'm a temporary read-only loan"
           "You can: read through me"
           "You cannot: modify through me"
```

### Trait as Contract

```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    // "I promise to call next() to get items, one by one
    //  When items run out, I return None"
}

impl Iterator for Range {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        // Your implementation must follow the contract
    }
}

// User can now assume:
// - next() will eventually return None
// - Won't call next() twice for same element
// - Elements come in defined order
```

### Generic Constraint

```rust
fn print<T: Display>(x: T) {
    // Type constraint: T must implement Display
    // This is a contract:
    // "I will only call methods from Display trait"
    // "User must give me something that implements Display"
    println!("{}", x);
}

// ✓ Valid: i32 implements Display
print(42);

// ✗ Invalid: Vec<i32> doesn't implement Display
// print(vec![1,2,3]);  // Compile error
```

---

## Async as State Machines

### The Mental Model

Async code is **hidden state machine**:

```rust
// This code:
async fn fetch_data() {
    let data = http_get("/api").await;  // ← Suspend point
    process(data).await;                  // ← Suspend point
}

// Compiles to (roughly):
enum FetchDataFuture {
    Start,
    Waiting { request: HttpFuture },
    Processing { response: Data, processor: ProcessFuture },
    Done,
}

impl Future for FetchDataFuture {
    fn poll(&mut self) -> Poll {
        loop {
            match self {
                Start => {
                    *self = Waiting { ... };
                    return Poll::Pending;
                }
                Waiting { request } => {
                    if let Poll::Ready(response) = request.poll() {
                        *self = Processing { ... };
                    } else {
                        return Poll::Pending;
                    }
                }
                Processing { processor, ... } => {
                    if processor.poll().is_ready() {
                        *self = Done;
                        return Poll::Ready(...);
                    } else {
                        return Poll::Pending;
                    }
                }
                Done => panic!("Polled after completion"),
            }
        }
    }
}
```

### The Scheduler's View

```
Rust Async Runtime (e.g., Tokio):

┌─────────────────────────────────────┐
│ Queue of Runnable Tasks             │
└─────────────────────────────────────┘

Thread 1: Pick task1 from queue
         │ poll(task1)
         ├─ task1 completes → Done
         ├─ task1 is blocked → return Pending
         │   Register with Waker:
         │   "Call me when file descriptor is ready"
         └─ Loop to next task

Disk I/O completes:
    OS notifies Waker
    Waker puts task1 back in queue
    Eventually Thread N picks it up
    poll(task1) again → now unblocked
```

### Why Pin?

```
Problem: Async functions can reference themselves

async fn needs_self_ref() {
    let s = String::from("hello");
    let ptr = &s as *const _;  // Pointer to s
    await_something().await;    // ← s might move here!
    use_pointer(ptr);           // ← Dangling pointer!
}

Solution: Pin<&mut T> guarantees T won't move
```

---

## Traits as Contracts

### The Mental Model

A trait defines a **set of obligations**:

```
Trait = Interface Contract

pub trait Animal {
    fn speak(&self) -> String;
    fn age(&self) -> u32;
}

"I'm implementing Animal, which means:
 - I must provide speak() method
 - I must provide age() method
 - Users can call these methods on me"
```

### Trait Object as Dynamic Dispatch

```rust
// Static dispatch (compile-time, fast):
fn process<T: Animal>(animal: &T) {
    println!("{}", animal.speak());
    // At compile time: specific type known
    // Code specialized for T
}

// Dynamic dispatch (runtime, flexible):
fn process(animal: &dyn Animal) {
    println!("{}", animal.speak());
    // At compile time: only know it's Animal
    // At runtime: find actual type from vtable
    // Slightly slower, but more flexible
}

// The trade-off:
// Static: ✓ Fast  ✗ Monomorphization  ✗ Less flexible
// Dynamic: ✗ Slower  ✓ Single code path  ✓ More flexible
```

---

## Memory Layout Visualization

### Stack Layout

```
Higher addresses (top of stack)
    ┌─────────────────┐
    │  Local var 3    │  ← Created last
    ├─────────────────┤
    │  Local var 2    │
    ├─────────────────┤
    │  Local var 1    │  ← Created first
    ├─────────────────┤
    │  Return address │
    ├─────────────────┤
    │  Previous frame │
    └─────────────────┘
Lower addresses (bottom)

Drop order: var3, var2, var1 (reverse)
```

### Heap Layout for String

```
String struct (on stack):
    ┌────────────────────┐
    │ ptr: 0x2000      ──┼──→ Heap memory (0x2000):
    │ len: 5           │    ┌─────────────┐
    │ capacity: 10     │    │ h           │
    └────────────────────┘    │ e           │
                              │ l           │
                              │ l           │
                              │ o           │
                              │ (empty) ... │
                              └─────────────┘

Drop: Dereferences ptr, frees heap
      Then stack space freed
```

### Vector Reallocation

```
Initial (capacity 3):
v = vec![1, 2, 3]
    ┌─────────┐      ┌─────────────────┐
    │ ptr ────┼─────→│ 1 │ 2 │ 3 │   │
    │ len: 3  │      └─────────────────┘
    │ cap: 3  │      (heap at 0x1000)
    └─────────┘

After push (capacity 6):
v.push(4)
    ┌─────────┐      ┌──────────────────────────┐
    │ ptr ────┼─────→│ 1 │ 2 │ 3 │ 4 │   │   │
    │ len: 4  │      └──────────────────────────┘
    │ cap: 6  │      (new heap at 0x3000)
    └─────────┘

Old heap at 0x1000: Freed

Key: Reallocation changes address!
     This is why mutable references are required for push()
```

---

## The Ownership Pyramid

```
                    ┌───────────────┐
                    │ Async Systems │
                    │  (State mach) │
                    └───────────────┘
                            ▲
                            │
                    ┌───────────────┐
                    │   Traits &    │
                    │  Generics     │
                    └───────────────┘
                            ▲
                            │
                    ┌───────────────┐
                    │   Borrowing   │
                    │  & mut &      │
                    └───────────────┘
                            ▲
                            │
                    ┌───────────────┐
                    │  Ownership    │
                    │  Move/Drop    │
                    └───────────────┘

Foundation: Each layer assumes lower layers work
Each layer adds capabilities on top of ownership
