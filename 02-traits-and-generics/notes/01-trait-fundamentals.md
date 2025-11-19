# Traits: Defining Behavior and Contracts

## What is a Trait?

A **trait** is a collection of methods that define a set of capabilities. It's Rust's way of defining interfaces and contracts.

```rust
pub trait Animal {
    fn speak(&self) -> String;
    fn age(&self) -> u32;
}
```

**Key insight:** Traits allow different types to share common behavior, enabling abstraction without inheritance.

---

## Defining Traits

### Basic Trait Definition

```rust
pub trait Reader {
    fn read(&mut self) -> Result<String, io::Error>;
    fn is_empty(&self) -> bool;
}
```

### Trait Methods

Methods in traits can be:
1. **Required** (abstract): no implementation
2. **Provided** (default): have implementation

```rust
pub trait Animal {
    // Required method - must implement in concrete types
    fn name(&self) -> &str;

    // Provided method - has default implementation
    fn describe(&self) -> String {
        format!("This is a {}", self.name())
    }
}
```

---

## Implementing Traits

### Basic Implementation

```rust
struct Dog {
    name: String,
}

impl Animal for Dog {
    fn name(&self) -> &str {
        &self.name
    }
    // describe() uses default implementation
}

let dog = Dog { name: "Buddy".to_string() };
println!("{}", dog.describe());  // "This is a Buddy"
```

### Multiple Trait Implementations

A type can implement multiple traits:

```rust
pub trait Drawable {
    fn draw(&self);
}

pub trait Resizable {
    fn resize(&mut self, width: u32, height: u32);
}

struct Shape {
    width: u32,
    height: u32,
}

impl Drawable for Shape {
    fn draw(&self) {
        println!("Drawing shape");
    }
}

impl Resizable for Shape {
    fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
}

let mut shape = Shape { width: 10, height: 20 };
shape.draw();
shape.resize(30, 40);
```

---

## Trait Bounds

### Generic Functions with Trait Bounds

```rust
fn print_animal<T: Animal>(animal: T) {
    println!("{}", animal.describe());
}

// Multiple bounds (AND)
fn process<T: Reader + Iterator>(item: T) {
    // item implements both Reader and Iterator
}

// Or using where clause
fn process<T>(item: T)
where
    T: Reader + Iterator,
{
    // Same thing, more readable for complex bounds
}
```

### Trait Bound Syntax Variations

```rust
// Bounds on a type
fn foo<T: Display>(x: T) { }

// Multiple bounds
fn bar<T: Display + Debug>(x: T) { }

// Lifetime bounds
fn baz<'a, T: 'a>(x: &'a T) { }

// Trait bound on where clause
fn qux<T>(x: T)
where
    T: Display + Debug,
{ }
```

---

## Associated Types

Associated types are types defined within a trait, allowing the trait to have type parameters.

### Without Associated Types

```rust
// Verbose - type specified at call site
pub trait Iterator {
    fn next(&mut self) -> Option<???>;  // What type?
}
```

### With Associated Types

```rust
pub trait Iterator {
    type Item;  // Associated type

    fn next(&mut self) -> Option<Self::Item>;
}

impl Iterator for Range {
    type Item = i32;  // Concrete type

    fn next(&mut self) -> Option<i32> {
        // ...
    }
}
```

**Why useful?** Each implementation defines its own item type. The trait method signature remains clean.

### Example: Generic Container

```rust
pub trait Container {
    type Item;

    fn len(&self) -> usize;
    fn get(&self, index: usize) -> Option<&Self::Item>;
}

impl Container for Vec<i32> {
    type Item = i32;

    fn len(&self) -> usize {
        Vec::len(self)
    }

    fn get(&self, index: usize) -> Option<&i32> {
        Vec::get(self, index)
    }
}

// Now Iterator can be generic
impl Iterator for Vec<String> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        // ...
    }
}
```

---

## Trait Objects (Dynamic Dispatch)

### Static Dispatch (Monomorphization)

```rust
fn print_animal<T: Animal>(animal: &T) {
    println!("{}", animal.describe());
}

let dog = Dog { name: "Buddy".to_string() };
let cat = Cat { name: "Whiskers".to_string() };

print_animal(&dog);   // Generates specialized code for Dog
print_animal(&cat);   // Generates specialized code for Cat
```

**Cost:** Larger binary (code specialized for each type)
**Benefit:** Fast! No runtime overhead.

### Dynamic Dispatch (Trait Objects)

```rust
fn print_animal(animal: &dyn Animal) {
    println!("{}", animal.describe());
}

let animals: Vec<Box<dyn Animal>> = vec![
    Box::new(Dog { name: "Buddy".to_string() }),
    Box::new(Cat { name: "Whiskers".to_string() }),
];

for animal in &animals {
    print_animal(animal);  // Dynamically dispatches to correct implementation
}
```

**Cost:** Slightly slower (virtual method lookup)
**Benefit:** Single code path, more flexible

### Trait Objects Work Only with Object-Safe Traits

A trait is **object-safe** if:
1. The return type is not `Self`
2. There are no generic type parameters
3. No static methods

```rust
pub trait Drawable {
    fn draw(&self);  // ✅ OK
}

pub trait Clone {
    fn clone(&self) -> Self;  // ❌ NOT object-safe (returns Self)
}

pub trait Factory<T> {  // ❌ NOT object-safe (generic parameter)
    fn create() -> T;
}
```

---

## Common Traits to Implement

### Debug and Display

```rust
use std::fmt;

#[derive(Debug)]
struct Point { x: i32, y: i32 }

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

let p = Point { x: 1, y: 2 };
println!("{:?}", p);     // Debug: Point { x: 1, y: 2 }
println!("{}", p);       // Display: (1, 2)
```

### Clone and Copy

```rust
impl Clone for Point {
    fn clone(&self) -> Self {
        Point { x: self.x, y: self.y }
    }
}

impl Copy for Point { }  // Requires Clone
```

### Eq and Ord

```rust
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.x.cmp(&other.x).then_with(|| self.y.cmp(&other.y))
    }
}
```

### Iterator

```rust
impl Iterator for CountUp {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        if self.current < self.max {
            let result = self.current;
            self.current += 1;
            Some(result)
        } else {
            None
        }
    }
}
```

---

## Trait Composition

### Extending Traits

```rust
pub trait Animal {
    fn speak(&self) -> String;
}

pub trait Pet: Animal {
    fn get_owner(&self) -> &str;
}

// Pet requires Animal too
struct Dog;

impl Animal for Dog {
    fn speak(&self) -> String { "woof".to_string() }
}

impl Pet for Dog {
    fn get_owner(&self) -> &str { "Alice" }
}
```

### Multiple Trait Bounds

```rust
fn describe<T: Animal + Pet>(animal: &T) {
    println!("{} is owned by {}", animal.speak(), animal.get_owner());
}
```

---

## Generic Trait Methods

### Methods with Generic Parameters

```rust
pub trait Processor {
    fn process<T: Display>(&self, item: T) -> String {
        format!("{}", item)
    }
}

impl Processor for MyProcessor { }

let p = MyProcessor;
p.process(42);
p.process("hello");
```

---

## Trait Variance

Traits with generic parameters have **variance rules**.

### Covariance Example

```rust
trait Processor<T> {
    fn process(&self, item: T);
}

let p: Box<dyn Processor<&'static str>> = Box::new(MyProcessor);
// Cannot use for shorter lifetime - contravariant
```

---

## Summary

| Concept | Purpose | Example |
|---------|---------|---------|
| **Trait** | Define interface/contract | `pub trait Animal { fn speak(&self); }` |
| **Implementation** | Provide concrete behavior | `impl Animal for Dog { fn speak(&self) { } }` |
| **Trait Bound** | Constrain generic types | `fn foo<T: Animal>(x: T)` |
| **Associated Type** | Type defined by trait | `type Item` in Iterator |
| **Trait Object** | Dynamic dispatch | `&dyn Trait` |
| **Default Methods** | Provided implementation | `fn describe() { "Animal" }` |

Traits are the foundation of Rust's abstraction system. Master them, and you unlock powerful composition patterns.
