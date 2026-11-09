# Variance: Subtyping in Generic Types

## What Is Variance?

**Variance** describes how subtyping of a generic type (`Vec<T>`) relates to subtyping of its parameter (`T`). If `Cat` is a subtype of `Animal` (conceptually, via lifetimes: `&'a Cat` where `'a` is longer), then:

- **Covariant** in `T`: `Vec<Cat>` is a subtype of `Vec<Animal>` — relationship preserved
- **Contravariant** in `T`: reversed — `Vec<Animal>` is a subtype of `Vec<Cat>`
- **Invariant** in `T`: no relationship at all — `Vec<Cat>` and `Vec<Animal>` are unrelated

In Rust, the relevant "subtyping" is mostly about **lifetimes**: `'long: 'short` (a longer lifetime is a subtype of a shorter one).

---

## The Three Variances in Rust

| Type | Variance in `T` | Why |
|---|---|---|
| `&'a T` | covariant in both `'a` and `T` | reading only — safe to treat `&'long Cat` as `&'short Animal` |
| `&'a mut T` | covariant in `'a`, **invariant** in `T` | writing — `&mut Cat` is NOT `&mut Animal` |
| `fn(T)` | **contravariant** in `T` | function arguments flow in |
| `fn() -> T` | covariant in `T` | return values flow out |
| `Vec<T>`, `Box<T>` | covariant in `T` | they only read `T` out |
| `Cell<T>`, `RefCell<T>` | **invariant** in `T` | interior mutability can write |

---

## Why `&mut T` Is Invariant

This is the classic footgun:

```rust
// Conceptually: if Cat <: Animal, covariance would let us do:
fn feed_animal(a: &mut Animal) { *a = Dog; }  // replace with a Dog

fn main() {
    let mut cat = Cat;
    let r: &mut Cat = &mut cat;
    feed_animal(r);        // if allowed: now `cat` is a Dog!
    cat.meow();            // 💥 type confusion — UB if it were allowed
}
```

Rust prevents this by making `&mut T` **invariant** in `T`: `&mut Cat` cannot be coerced to `&mut Animal`. You *can* shorten the lifetime (`&'long mut T` → `&'short mut T`, covariant in `'a`), but never change `T`.

```rust
fn takes_animal(_a: &mut Animal) {}

fn main() {
    let mut cat = Cat;
    // takes_animal(&mut cat);  // ❌ invariant: &mut Cat is not &mut Animal
}
```

---

## Covariance in Action

Immutable references ARE covariant, which is safe because you can only read:

```rust
fn print_animal(a: &Animal) { /* read only */ }

fn main() {
    let cat = Cat;
    let r: &Cat = &cat;
    print_animal(r);   // ✅ &Cat coerces to &Animal (covariance)
}
```

And lifetimes: a `&'static str` can be used anywhere a `&'a str` is expected (`'static: 'a` for all `'a`):

```rust
fn needs_short<'a>(s: &'a str) -> &'a str { s }
let s: &'static str = "forever";
let r = needs_short(s);   // ✅ 'static coerces to any shorter lifetime
```

---

## Contravariance with Function Pointers

```rust
// fn(Animal) is a subtype of fn(Cat): a function accepting ANY animal
// can certainly accept a cat.
fn feed(a: Animal) {}
let f: fn(Cat) = feed;   // ✅ contravariance allows this
```

---

## Practical Consequences

1. **`&mut` blocks type changes** — invariance protects you from aliasing+mutation bugs.
2. **`Cell`/`RefCell` are invariant** — because they allow mutation through a shared reference.
3. **Lifetime shortening always works** — `&'a mut T` → `&'b mut T` when `'a: 'b`.
4. **Function arguments contravariant** — a more general function can be used where a specific one is needed.

```rust
// Demonstration: Vec<T> is covariant in T
fn longer_to_shorter<'a>(x: &'a Vec<i32>) -> &'a Vec<i32> { x }

fn main() {
    let v = vec![1, 2, 3];
    let r: &'static Vec<i32> = Box::leak(Box::new(v)); // pretend 'static
    let short: &Vec<i32> = longer_to_shorter(r);        // coerce 'static -> shorter
    println!("{:?}", short);
}
```

---

## Mental Model

- **Covariant** = "read-only pipeline": subtypes flow along with the container.
- **Contravariant** = "input position": subtypes flow against the container.
- **Invariant** = "read-write position": no subtyping allowed, period.

Whenever the compiler says "expected `&'a mut T`, found `&'b mut U`" and refuses, you're looking at invariance doing its job.

---

## Exercises

See `exercises/03-advanced-trait-patterns.rs`.
