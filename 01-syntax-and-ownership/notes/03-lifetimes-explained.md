# Lifetimes: Encoding Relationships Between References

## What is a Lifetime?

A **lifetime** is a name for a scope. It tells the compiler how long a reference is valid.

```rust
fn foo<'a>(s: &'a str) -> &'a str {
    s
}

// Read as:
// "foo takes a reference with lifetime 'a
//  and returns a reference with the same lifetime 'a"
```

**Key insight:** Lifetimes are **not about runtime duration**; they're about **compile-time relationships**.

---

## The Problem Without Lifetimes

```rust
fn returns_ref() -> &String {  // ❌ ERROR
    let s = String::from("hello");
    &s  // Reference to local variable
}

// What would happen at runtime?
// 1. Function creates String on stack
// 2. Function returns reference to stack-allocated String
// 3. Function returns, stack unwound, String destroyed
// 4. Caller has reference to destroyed memory (use-after-free!)

// Rust compiler: "I can't let you do that!"
```

---

## Lifetime Syntax

### Basic Lifetime Annotation

```rust
&i32         // A reference to i32 (lifetime elided)
&'a i32      // A reference to i32 with lifetime 'a

fn foo(x: &'a i32) -> &'a i32 {
    x
}
// Lifetime 'a is a parameter like T
// It can be any scope
```

### Multiple Lifetimes

```rust
fn two_refs<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32 {
    x  // Return has lifetime 'a (from first parameter)
}

// 'a and 'b can be different scopes
// Output lifetime is 'a (not 'b)
```

### Lifetime Bounds

```rust
fn longer<'a, 'b: 'a>(x: &'a i32, y: &'b i32) -> &'a i32
where
    'b: 'a,  // 'b outlives 'a
{
    if x > y { x } else { y }
}

// 'b: 'a means: "everything valid in 'a is also valid in 'b"
// More concretely: "y lives at least as long as x"
```

---

## Lifetime Elision

Rust infers lifetimes in common patterns.

### Rule 1: Each Reference Gets Its Own Lifetime

```rust
fn foo(x: &i32, y: &i32) -> &i32 {  // ❌ ERROR: which lifetime?
    x  // Or y? Compiler doesn't know!
}

// Explicit:
fn foo<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32 {
    x
}
```

### Rule 2: If One Parameter, Output Lifetime is That Parameter's

```rust
fn foo(x: &i32) -> &i32 {  // ✅ Elided
    x
}

// Explicit:
fn foo<'a>(x: &'a i32) -> &'a i32 {
    x
}
```

### Rule 3: If &mut self, Output Lifetime is self's

```rust
impl Foo {
    fn bar(&mut self) -> &i32 {  // ✅ Elided
        &self.data
    }
}

// Explicit:
impl Foo {
    fn bar<'a>(&'a mut self) -> &'a i32 {
        &self.data
    }
}
```

### When Elision Fails

```rust
fn foo(x: &i32, y: &i32) -> &i32 {
    x  // ❌ ERROR: ambiguous output lifetime
}

// Fix: Specify lifetime explicitly
fn foo<'a>(x: &'a i32, y: &i32) -> &'a i32 {
    x
}
```

---

## Common Patterns

### Pattern 1: Borrow and Return Same

```rust
fn first_word(s: &str) -> &str {
    // Elided, really:
    // fn first_word<'a>(s: &'a str) -> &'a str
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];  // Same lifetime as input
        }
    }
    &s[..]
}

let s = String::from("hello world");
let word = first_word(&s);
println!("{}", word);  // "hello"
// word's lifetime is tied to s's lifetime
```

### Pattern 2: &mut self

```rust
struct Buffer {
    data: Vec<u8>,
}

impl Buffer {
    fn as_slice(&self) -> &[u8] {
        &self.data
    }

    fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.data
    }
}
```

### Pattern 3: Struct With Borrowed Data

```rust
struct Parser<'a> {
    input: &'a str,  // Borrows input
    position: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Parser { input, position: 0 }
    }

    fn parse(&mut self) -> &'a str {
        // Returns a lifetime from input
        let start = self.position;
        // ... move position ...
        &self.input[start..self.position]
    }
}

let input = "hello world";
let mut parser = Parser::new(input);  // Parser borrows input
let word = parser.parse();  // word's lifetime from input
println!("{}", word);
```

---

## Variance

Lifetimes have **variance rules**.

### Covariance

```rust
let r1: &'static str = "hello";
let r2: &'a str = r1;  // ✅ Works!

// 'static is longer than 'a
// So something that lives 'static can be used where 'a expected
```

### Contravariance in Functions

```rust
fn takes_fn<F>(f: F) where F: Fn(&'a i32) {
    // f can accept references with lifetime 'a or longer
}

// If you have a function that accepts longer lifetime:
fn takes_longer(x: &'static i32) { }

// But that function accepts longer → reject
// takes_fn(&takes_longer);  // ❌ Won't work

// But if shorter lifetime accepted:
fn takes_shorter<'b>(x: &'b i32) { }
takes_fn(&takes_shorter);  // ✅ Works
```

---

## Lifetime Bounds in Traits

### Trait Bounds with Lifetimes

```rust
struct Container<'a> {
    data: &'a [u8],
}

impl<'a> Clone for Container<'a> {
    fn clone(&self) -> Self {
        Container { data: self.data }
    }
}
```

### Lifetime Parameters on Traits

```rust
trait Reader<'a> {
    fn read(&mut self) -> &'a str;
}

struct LineReader<'a> {
    input: &'a str,
}

impl<'a> Reader<'a> for LineReader<'a> {
    fn read(&mut self) -> &'a str {
        // Return string with lifetime 'a from input
        "..."
    }
}
```

---

## Mutable References and Lifetimes

```rust
fn foo<'a>(x: &'a mut i32) {
    *x = 5;
    *x = 10;  // ✅ Multiple mutations OK
}

let mut n = 5;
foo(&mut n);

// Compare to immutable:
fn bar<'a>(x: &'a i32) {
    // println!("{}", x);  // Can read multiple times
}
```

---

## The Three Rules

### Rule 1: Reference Can't Outlive Its Referent

```rust
fn bad() -> &String {  // ❌ ERROR
    let s = String::from("hello");
    &s  // Reference outlives s
}

// Graphically:
// s scope:     |---- created ----- dropped ---|
// return ref:  |---- created -----------|
//                              ↑ Outlives!
```

### Rule 2: Can't Return Reference to Function Argument If It Might Become Invalid

```rust
fn bad(s: &str) -> &str {  // ✅ Actually OK
    s
}

// Why OK? Because s's scope is at least as long as function exists
```

### Rule 3: Mutable and Immutable Can't Coexist

```rust
let mut x = 5;
let r1 = &x;
let r2 = &mut x;  // ❌ ERROR

// Lifetimes don't overlap, so immutable borrow is still active
```

---

## Lifetime Elision Walkthrough

### Example 1: Two Parameters

```rust
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

let s1 = String::from("hello");
let s2 = "world";

// Call: longest(&s1, &s2)
// - s1 has lifetime 'a (from s1's scope)
// - s2 has lifetime 'b (from "world"'s scope, which is longer)
// - Result has lifetime 'a (lifetime of x)
// - Result must be used within s1's scope!

let result = longest(&s1, &s2);
drop(s1);  // ❌ Can't drop s1 if result still exists
```

### Example 2: Struct Methods

```rust
struct Database<'a> {
    connection: &'a str,
}

impl<'a> Database<'a> {
    fn query(&self, sql: &str) -> &str {
        // Elided fully:
        // fn query<'b>(&'a self, sql: &'b str) -> &'a str
        self.connection  // Lifetime from self
    }
}
```

---

## Debugging Lifetime Errors

### Error: "Lifetime Mismatch"

```rust
fn foo<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    y  // ❌ ERROR: y has lifetime 'b, need 'a
}

// Fix: Use correct parameter
fn foo<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    x  // ✅ OK
}
```

### Error: "Borrowed Value Doesn't Live Long Enough"

```rust
fn bad() {
    let r;
    {
        let s = String::from("hello");
        r = &s;  // ❌ s's lifetime ends here
    }
    println!("{}", r);  // r points to destroyed value
}

// Fix: Extend s's lifetime
fn good() {
    let s = String::from("hello");
    let r = &s;
    println!("{}", r);  // ✅ s lives long enough
}
```

### Error: "Cannot Infer Appropriate Lifetime"

```rust
fn ambiguous(x: &i32, y: &i32) -> &i32 {  // ❌ Which lifetime?
    x
}

// Fix: Make explicit
fn clear<'a>(x: &'a i32, y: &i32) -> &'a i32 {
    x  // ✅ Clear
}
```

---

## Summary Table

| Pattern | Lifetime Rule |
|---------|---------------|
| `&'a T` | Borrow with lifetime 'a |
| `&'a mut T` | Mutable borrow with lifetime 'a |
| `fn foo<'a>(x: &'a T) -> &'a T` | Return same lifetime as input |
| `&self` | Borrow from self lifetime |
| `&mut self` | Mutable borrow from self lifetime |
| `'a: 'b` | 'a outlives 'b |
| `'static` | Entire program duration |

---

## Practice

Try to infer lifetimes without looking:

```rust
fn foo(x: &String) -> &str { &x[..] }
// Answer: fn foo<'a>(x: &'a String) -> &'a str

fn bar<'a>(x: &'a mut Vec<i32>, y: &Vec<i32>) -> &'a i32 { &x[0] }
// Answer: y's lifetime not used in return type

struct S<'a> { s: &'a str }
impl<'a> S<'a> {
    fn get(&self) -> &str { self.s }
}
// Answer: fn get<'b>(&'b self) -> &'b str (tied to self lifetime)
```
