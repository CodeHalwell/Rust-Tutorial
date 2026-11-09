# Higher-Ranked Trait Bounds (HRTBs): `for<'a>`

## The Problem

Consider a function that accepts any closure working on a borrowed string:

```rust
fn apply<F>(f: F) -> bool
where
    F: Fn(&str) -> bool,   // what lifetime does this &str have?
{
    f("hello")
}
```

The `&str` hides a lifetime. The compiler actually reads this as:

```rust
fn apply<F>(f: F) -> bool
where
    F: for<'a> Fn(&'a str) -> bool,   // HRTB: works for ALL lifetimes 'a
```

**Key insight:** `for<'a> Fn(&'a str) -> bool` means "this closure can be called with a `&str` of *any* lifetime" — the closure must not assume the reference lives longer than the call. That's exactly what you want for callbacks.

---

## Reading `for<'a>` Bounds

```rust
// "For all lifetimes 'a, F is callable with &'a T and returns bool"
fn call_with_str<F>(f: F) -> bool
where
    F: for<'a> Fn(&'a str) -> bool,
{
    f("short") && f("a much longer string")
}

// Equivalent: closures that borrow nothing work fine
fn main() {
    let ok = call_with_str(|s| !s.is_empty());
    assert!(ok);
}
```

### Contrast with a single fixed lifetime

```rust
// ❌ This does NOT compile — you can't name the caller's lifetime here:
// fn apply_bad<F>(f: F) where F: Fn(&'??? str) -> bool { ... }
//
// ✅ HRTB is the way to say "any lifetime":
fn apply_good<F>(f: F) -> bool
where
    F: for<'a> Fn(&'a str) -> bool,
{
    f("hi")
}
```

---

## Common HRTB Shapes

```rust
// Works with references of any lifetime and any type T: 'a
fn for_each<T, F>(items: &[T], f: F)
where
    F: for<'a> Fn(&'a T),
{
    for item in items {
        f(item);
    }
}

// Fn traits with HRTB are how Iterator-like adapters accept closures:
// Iterator::any is: fn any<F>(&mut self, f: F) -> bool
//                   where F: FnMut(Self::Item) -> bool
// For closures taking references, the bound is typically written:
//   F: for<'a> FnMut(&'a Self::Item) -> bool

// Compare: without for<'a>, the compiler picks ONE specific lifetime,
// and closures that need to work with many different lifetimes fail:
fn needs_hrtb<F>(f: F)
where
    F: for<'a> Fn(&'a u32) -> &'a u32,  // identity, any lifetime
{
    let x = 5;
    let y = f(&x);
    assert_eq!(*y, 5);
}
```

---

## HRTBs and Closures That Capture

A closure that *captures* a reference can only satisfy the HRTB if its captured reference outlives every call:

```rust
fn apply<F>(f: F) -> bool
where
    F: for<'a> Fn(&'a str) -> bool,
{
    f("test")
}

fn main() {
    let captured = String::from("owned");
    // ✅ closure borrows `captured`, but the borrow of captured outlives
    //    the call to apply, so the HRTB is satisfied
    let r = apply(|s| s.len() > captured.len());
    println!("{}", r);
}
```

If the closure tried to *store* the `&'a str` beyond the call, the HRTB would reject it — which is precisely the safety guarantee.

---

## `for<'a>` on Trait Objects

```rust
// A trait object that works with any borrowed input:
type StrProcessor = dyn for<'a> Fn(&'a str) -> usize;

fn run(p: &StrProcessor, s: &str) -> usize {
    p(s)
}
```

---

## Quick Reference

| You write | Compiler reads | Meaning |
|---|---|---|
| `F: Fn(&str) -> bool` | `F: for<'a> Fn(&'a str) -> bool` | any lifetime (elided = HRTB for fn traits in bounds) |
| `F: for<'a> Fn(&'a str) -> &'a str` | same, explicit | identity over any borrow |
| `F: Fn(&'static str) -> bool` | fixed `'static` only | only 'static refs — very restrictive |

**Mental model:** `for<'a>` is a "for all lifetimes" quantifier. Whenever a trait bound mentions a reference inside `Fn(...)` arguments, think HRTB by default.

---

## Exercises

See `exercises/03-advanced-trait-patterns.rs` for hands-on HRTB challenges.
