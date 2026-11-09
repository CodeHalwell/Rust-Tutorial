# Associated Types: Encoding Relationships in Traits

## The Problem with Type Parameters

Generic traits force callers to specify the type at the *call site*, even when it's determined by the trait itself:

```rust
// With a type parameter, the caller must say what Item is:
trait Container<A> {
    fn get(&self, index: usize) -> Option<&A>;
}

fn first<C: Container<u32>>(c: &C) -> Option<&u32> { c.get(0) }
// What if we want the same for Container<i32>? We need another function.
```

If a type implements `Container<A>` for *multiple* `A`s, ambiguity arises. Associated types fix this by saying: **each implementation picks exactly one associated type**.

---

## Associated Types

```rust
trait Container {
    type Item;                       // associated type — chosen per impl

    fn get(&self, index: usize) -> Option<&Self::Item>;
}

struct IntVec { data: Vec<i32> }

impl Container for IntVec {
    type Item = i32;                 // this impl's associated type
    fn get(&self, index: usize) -> Option<&i32> {
        self.data.get(index)
    }
}
```

**Key insight:** `type Item` inside a trait is a placeholder the implementor fills in. Callers use `Self::Item` / `<C as Container>::Item` without naming it.

---

## Associated Types vs Type Parameters

| | Type parameter `trait Container<A>` | Associated type `trait Container { type Item; }` |
|---|---|---|
| One type can implement the trait... | many times, for different `A` | exactly once |
| Caller must specify... | the parameter | nothing |
| Use when... | the type genuinely varies per use | the type is fixed by the impl |

**Rule of thumb:** If a type can only implement the trait one way, use an associated type. Real-world example: `Iterator::Item` — a `Vec<i32>` iterator yields `i32`, always.

```rust
// Iterator is defined with an associated type:
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    // ... dozens of provided methods using Self::Item
}
```

---

## Fully Qualified Syntax

When ambiguity exists, disambiguate with fully qualified syntax:

```rust
fn describe<C: Container>(c: &C) -> String {
    // Shorthand: C::Item (only works if unambiguous)
    fn helper<C: Container>(c: &C) -> Option<&C::Item> { c.get(0) }

    // Fully qualified — always works:
    fn helper_fq<C: Container>(c: &C) -> Option<&<C as Container>::Item> {
        c.get(0)
    }
    format!("{:?}", helper(c))
}
```

---

## Associated Types with Bounds

Constrain associated types in the trait definition or at use sites:

```rust
trait Graph {
    type Node;
    type Edge;

    fn edges(&self, node: &Self::Node) -> Vec<&Self::Edge>;
}

// Use-site bound: we only care about graphs whose nodes are printable
fn print_edges<G>(g: &G)
where
    G: Graph,
    G::Node: std::fmt::Debug,
{
    // ...
}

// Bound in the trait itself:
trait Summarizable {
    type Output: std::fmt::Display;   // associated type must be Display
    fn summarize(&self) -> Self::Output;
}
```

---

## Associated Consts

Traits can also have associated constants:

```rust
trait Defaults {
    const DEFAULT_NAME: &'static str;
    const MAX_RETRIES: u32 = 3;   // can have a default
}

struct HttpClient;
impl Defaults for HttpClient {
    const DEFAULT_NAME: &'static str = "http";
}
// HttpClient::MAX_RETRIES == 3
```

---

## Design Example: A Type-Safe Units System

```rust
trait Quantity {
    type Unit;
    fn value(&self) -> f64;
    fn unit(&self) -> Self::Unit;
}

struct Meters(f64);
struct Seconds(f64);

struct Length(f64);
impl Quantity for Length {
    type Unit = Meters;
    fn value(&self) -> f64 { self.0 }
    fn unit(&self) -> Meters { Meters(self.0) }
}

// Only Length + Length is allowed — compile-time dimensional analysis:
fn add_lengths(a: &Length, b: &Length) -> Length {
    Length(a.value() + b.value())
}
```

**Why this matters:** Associated types let the type system encode *relationships* ("this container yields these items", "this quantity is measured in these units") that a single type parameter can't express safely.

---

## Exercises

- `exercises/01-trait-implementations.rs` — basic traits
- `exercises/03-advanced-trait-patterns.rs` — associated types, HRTBs, variance in practice
