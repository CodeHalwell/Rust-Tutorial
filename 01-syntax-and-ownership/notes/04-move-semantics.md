# Move Semantics: Ownership in Action

## What Is a Move?

A **move** transfers ownership of a value from one binding to another. After a move, the original binding is no longer valid—the compiler will reject any use of it.

```rust
let s1 = String::from("hello");  // s1 owns the heap buffer
let s2 = s1;                      // ownership MOVES to s2
// println!("{}", s1);            // ❌ error: borrow of moved value
println!("{}", s2);               // ✅ only s2 is valid
```

**Key insight:** A move is not a copy. The heap buffer is *not* duplicated; the pointer is transferred, and `s1` is invalidated so the buffer can never be double-freed.

---

## Move vs Copy

Rust splits types into two categories:

| Category | Behavior on assignment | Types |
|----------|----------------------|-------|
| `Copy` | Bits are copied; original stays valid | `i32`, `f64`, `bool`, `char`, `&T`, tuples/arrays of `Copy` |
| Move | Ownership transfers; original invalidated | `String`, `Vec<T>`, `Box<T>`, most structs |

```rust
let x = 5;
let y = x;
println!("{}", x);  // ✅ i32 is Copy, x still valid

let s = String::from("hi");
let t = s;
// println!("{}", s);  // ❌ String is not Copy, s was moved
```

### Implementing Copy

A type can implement `Copy` only if all its fields are `Copy`—and `Copy` requires `Clone`:

```rust
#[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

let p1 = Point { x: 1.0, y: 2.0 };
let p2 = p1;        // copied
println!("{} {}", p1.x, p2.x);  // ✅ both valid
```

**Rule of thumb:** If a bitwise copy is correct and cheap, derive `Copy`. `String` can't be `Copy` because it owns a heap allocation—copying the pointer would create two owners of one buffer (double-free).

---

## Moves in Function Calls

Passing a value to a function moves (or copies) it:

```rust
fn takes_ownership(s: String) {
    println!("{}", s);
}  // s dropped here

fn makes_copy(x: i32) {
    println!("{}", x);
}  // nothing special happens

fn main() {
    let s = String::from("hello");
    takes_ownership(s);        // s moved into the function
    // println!("{}", s);      // ❌ s no longer valid

    let x = 5;
    makes_copy(x);             // x copied
    println!("{}", x);         // ✅ x still valid
}
```

### Returning Ownership

Functions can transfer ownership back via return values:

```rust
fn gives_ownership() -> String {
    String::from("yours")      // moved out to the caller
}

fn takes_and_gives_back(s: String) -> String {
    s                          // moved out to the caller
}

fn main() {
    let s = gives_ownership();            // s owns the String
    let s2 = takes_and_gives_back(s);     // s moved in, s2 owns it now
}
```

---

## Moves in Practice

### Moves out of structs

You can move individual fields out of a struct (if the struct isn't `Copy` and isn't `Drop`):

```rust
struct User {
    name: String,
    age: u32,
}

let user = User { name: String::from("Ada"), age: 36 };
let name = user.name;   // name moved out
// println!("{}", user.name);  // ❌ field moved
println!("{}", user.age);    // ✅ age is Copy, still usable
// println!("{:?}", user);     // ❌ can't use partially-moved struct
```

### Moves in loops and closures

```rust
let words = vec![String::from("a"), String::from("b")];
for w in words {           // each String is moved into w, then dropped
    println!("{}", w);
}
// println!("{:?}", words);  // ❌ words was moved by the for loop

// Borrow instead if you need it afterwards:
for w in &words {
    println!("{}", w);
}
println!("{:?}", words);   // ✅ still valid
```

Closures capture by move with the `move` keyword:

```rust
let s = String::from("hello");
let print_s = move || println!("{}", s);  // s moved into the closure
// println!("{}", s);  // ❌ moved
print_s();
```

---

## Why Moves Matter

1. **Memory safety**: exactly one owner means exactly one free—no double-free, no use-after-free.
2. **Zero cost**: a move is just a bitwise copy of the pointer/length/capacity triple; no deep copy happens.
3. **Clear semantics**: the compiler tells you precisely where data flows.

**Mental model:** Think of a move as handing over a physical key. There is only one key. Once handed over, the previous holder can't use it—and the compiler enforces this.

---

## Common Patterns

```rust
// Clone when you explicitly need a deep copy
let s1 = String::from("hello");
let s2 = s1.clone();   // heap buffer duplicated; both valid

// Use references to avoid moving
fn length(s: &String) -> usize { s.len() }
let s = String::from("hello");
println!("{} {}", length(&s), s);  // both work

// Return tuples to give back multiple values
fn split(s: String) -> (String, String) {
    let mid = s.len() / 2;
    (s[..mid].to_string(), s[mid..].to_string())
}
```

---

## Exercises

Continue in `exercises/`:

- `01-ownership-exercises.rs` — moves, copies, drop order
- `02-borrow-checker-challenges.rs` — fix borrow errors
- `03-lifetime-annotations.rs` — annotate lifetimes by hand
