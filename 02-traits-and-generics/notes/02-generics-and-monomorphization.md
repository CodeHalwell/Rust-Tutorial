# Generics: Write Once, Specialize for Each Type

## What Are Generics?

Generics allow you to write code that works with any type, while the compiler specializes it for each concrete type you use.

```rust
fn largest<T: Ord>(list: &[T]) -> &T {
    // Works with any T that implements Ord
    // Compiler generates specialized code for i32, String, etc.
}

largest(&[1, 2, 3]);              // Generates largest::<i32>
largest(&["a", "b", "c"]);        // Generates largest::<&str>
```

---

## Generic Types

### Functions

```rust
fn first<T>(list: &[T]) -> Option<&T> {
    list.first()
}

let nums = vec![1, 2, 3];
let words = vec!["hello", "world"];

first(&nums);      // T = i32
first(&words);     // T = &str
```

### Structs

```rust
struct Point<T> {
    x: T,
    y: T,
}

let int_point = Point { x: 1, y: 2 };
let float_point = Point { x: 1.5, y: 2.5 };

// Different types, different in-memory representations
// int_point: [1: i32][2: i32]
// float_point: [1.5: f64][2.5: f64]
```

### Enums

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}

enum Option<T> {
    Some(T),
    None,
}

let int_result: Result<i32, String> = Ok(42);
let string_result: Result<String, String> = Ok("success".to_string());
```

### Implementations

```rust
struct Container<T> {
    item: T,
}

impl<T> Container<T> {
    fn new(item: T) -> Self {
        Container { item }
    }

    fn get(&self) -> &T {
        &self.item
    }
}

// Specialized for different types
let int_container = Container::new(42);      // Container::<i32>
let string_container = Container::new("hi"); // Container::<&str>
```

---

## Monomorphization: The Secret Behind Generics

**Monomorphization** is the compiler creating specialized copies of generic code for each concrete type used.

### How It Works

```rust
fn add<T: std::ops::Add>(a: T, b: T) -> T {
    a + b
}

fn main() {
    add(5i32, 10i32);        // Call with i32
    add(5.5f64, 10.5f64);    // Call with f64
}
```

**What the compiler generates:**

```rust
fn add_i32(a: i32, b: i32) -> i32 {
    a + b
}

fn add_f64(a: f64, b: f64) -> f64 {
    a + b
}

fn main() {
    add_i32(5, 10);
    add_f64(5.5, 10.5);
}
```

### Trade-off: Speed vs Binary Size

| Aspect | Cost | Benefit |
|--------|------|---------|
| **Runtime** | Zero cost! Same as hand-written specialized code | Maximum performance |
| **Binary Size** | Can grow (duplicated code for each type) | -|
| **Compile Time** | Slower (more code to generate) | -|

### Example: Binary Size Impact

```rust
fn process<T: Display>(x: T) -> String {
    format!("{}", x)
}

// With 10 different types used:
// Generated 10 different versions of process::<i32>
// Generated 10 different versions of process::<String>
// etc.
```

**Typical binary size:** 10-20% larger than equivalent C++ templates
**Trade-off:** worth it for the zero-cost abstraction

---

## Generic Constraints (Trait Bounds)

### Single Bound

```rust
fn print<T: Display>(x: T) {
    println!("{}", x);
}
```

### Multiple Bounds

```rust
fn process<T: Clone + Display>(x: T) {
    let copy = x.clone();
    println!("{}", copy);
}
```

### Where Clause

```rust
fn complex<T, U>(x: T, y: U)
where
    T: Clone + Display,
    U: Iterator,
    U::Item: Display,
{
    // Complex bounds expressed clearly
}
```

**When to use where clause:**
- Multiple bounds per type
- Bounds referencing associated types
- Very many bounds (readability)

### Lifetime Bounds

```rust
fn foo<'a, T: 'a>(x: &'a T) {
    // T must live at least 'a
}

// More realistic:
struct Parser<'a, T: 'a> {
    input: &'a str,
    items: Vec<T>,
}
```

---

## Generic Associated Types (Advanced)

### Associated Types in Traits

```rust
pub trait Iterator {
    type Item;  // Associated type

    fn next(&mut self) -> Option<Self::Item>;
}
```

### Generic Associated Types (GAT)

```rust
pub trait Iterable {
    type Item<'a>;  // Generic associated type (lifetime parameter)

    fn iter(&mut self) -> Box<dyn Iterator<Item = Self::Item>>;
}

// Now Item can depend on lifetime
struct StringVec {
    strings: Vec<String>,
}

impl Iterable for StringVec {
    type Item<'a> = &'a String;

    fn iter(&mut self) -> Box<dyn Iterator<Item = &String>> {
        Box::new(self.strings.iter())
    }
}
```

---

## Generic Lifetimes

### Lifetime Parameters in Generics

```rust
struct References<'a, 'b, T> {
    x: &'a T,
    y: &'b T,
}

// 'a and 'b can be different lifetimes
let r = References {
    x: &1,
    y: &2,
};
```

### Generic with Lifetime Constraint

```rust
fn foo<'a, 'b: 'a, T: 'a>(x: &'a T, y: &'b T) -> &'a T {
    // 'b: 'a means 'b outlives 'a
    if std::mem::size_of_val(x) > 0 { x } else { y }
}
```

---

## Generic Implementation Blocks

### Implement for Specific Types

```rust
impl Container<i32> {
    fn special_method(&self) {
        println!("Special for i32");
    }
}

impl<T> Container<T> {
    fn general_method(&self) {
        println!("General for any T");
    }
}

// Container::<i32> gets both methods
// Container::<String> gets only general_method
```

### Conditional Implementations

```rust
// Only implement Clone if T implements Clone
impl<T: Clone> Clone for Container<T> {
    fn clone(&self) -> Self {
        Container {
            item: self.item.clone(),
        }
    }
}

// Only implement Display if T implements Display
impl<T: Display> Display for Container<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Container({})", self.item)
    }
}
```

---

## SPECIALIZATION (Advanced/Unstable)

This is an unstable feature that allows more specific implementations to override generic ones.

```rust
#![feature(specialization)]

trait Foo {
    fn foo(&self);
}

// Generic implementation
impl<T> Foo for T {
    fn foo(&self) {
        println!("Generic");
    }
}

// Specialized implementation for String
impl Foo for String {
    fn foo(&self) {
        println!("Specialized for String");
    }
}

// String gets specialized implementation
// Other types get generic implementation
```

---

## Phantom Types

### What is a Phantom Type?

A type parameter that doesn't appear in fields, used only for compile-time checking.

```rust
use std::marker::PhantomData;

struct Database<T> {
    items: Vec<String>,
    _phantom: PhantomData<T>,  // Doesn't use T, but tracks it
}

// Different databases with different type tags
struct User;
struct Product;

let users: Database<User> = Database {
    items: vec![],
    _phantom: PhantomData,
};

let products: Database<Product> = Database {
    items: vec![],
    _phantom: PhantomData,
};

// Compiler won't let you mix them up!
// users and products are different types
```

### Why Use Phantom Types?

1. **Type safety without overhead:** No runtime cost
2. **Encode state in type system:** Database\<Connected\> vs Database\<Disconnected\>
3. **Prevent misuse:** Builder patterns, type-state pattern

---

## Generic Constants

```rust
struct Array<T, const N: usize> {
    items: [T; N],
}

impl<T, const N: usize> Array<T, N> {
    fn len(&self) -> usize {
        N
    }
}

let arr: Array<i32, 10> = Array { items: [0; 10] };
println!("{}", arr.len());  // 10
```

---

## Performance Implications

### Monomorphization Explosion

```rust
fn process<T: Debug>(x: T) { }

process(1i32);
process(1i64);
process(1.0f32);
process(1.0f64);
process("string");
process(true);

// Generates 6 different versions!
```

**Impact:**
- Compile time: slow
- Binary size: large
- Runtime: fast (no virtual calls)

### Mitigation Strategies

**1. Use trait objects for many types:**
```rust
fn process(x: &dyn Debug) { }  // Single version
```

**2. Abstract over shared behavior:**
```rust
fn process<T: Display>(x: T) { }  // Simpler bound = simpler code
```

**3. Split implementation:**
```rust
// Generic interface
impl<T> Container<T> {
    fn len(&self) -> usize { /* ... */ }
}

// Concrete implementation for common types
impl Container<String> {
    fn special(&self) { /* ... */ }
}
```

---

## Summary

| Feature | When to Use | Trade-off |
|---------|-------------|-----------|
| **Generics** | Flexible, type-safe code | Monomorphization cost |
| **Trait Bounds** | Constrain what types work | More verbose |
| **Where Clause** | Complex bounds | Slightly more code |
| **Trait Objects** | Many different types | Small runtime cost |
| **Phantom Types** | Compile-time tracking | Less obvious |

**Core principle:** Rust generics give you C++ templates power with Java generics ease, plus guaranteed memory safety.
