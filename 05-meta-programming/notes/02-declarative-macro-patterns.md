# Declarative Macro Patterns

## Pattern 1: Internal Rules (`@` markers)

Pass state through recursion using `@`-prefixed markers that are not valid user input:

```rust
macro_rules! sum {
    // entry point
    ($($x:expr),* $(,)?) => { sum!(@acc 0, $($x),*) };
    // accumulate one item
    (@acc $acc:expr, $x:expr $(, $rest:expr)* $(,)?) => {
        sum!(@acc $acc + $x $(, $rest)*)
    };
    // base case
    (@acc $acc:expr $(,)?) => { $acc };
}

assert_eq!(sum!(1, 2, 3, 4), 10);
```

**Key insight:** `@acc` can never be produced by `$x:expr` matching, so there's no ambiguity between arms.

---

## Pattern 2: TT Muncher

Process one token tree at a time, recursing on the rest:

```rust
macro_rules! count_tt {
    () => { 0 };
    ($tt:tt $($rest:tt)*) => { 1 + count_tt!($($rest)*) };
}

assert_eq!(count_tt!(a b (c d) e), 4);
```

---

## Pattern 3: Incremental Rules (list building)

Build up output incrementally while consuming input:

```rust
macro_rules! reverse {
    () => { Vec::new() };
    ($($rest:expr),* ; $($acc:expr),*) => { reverse!($($rest),* ; $($acc),*) };
}
// (simplified illustration — real implementations munch one item at a time)
```

---

## Hygiene

`macro_rules!` is **hygienic**: identifiers introduced *inside* the macro don't collide with identifiers at the call site:

```rust
macro_rules! using_temp {
    ($e:expr) => {{
        let temp = 42;      // this `temp` is hygienic — invisible outside
        $e + temp
    }};
}

let temp = 0;
let r = using_temp!(1);  // 43 — the outer `temp` is untouched
assert_eq!(temp, 0);
```

**Key insight:** Hygiene prevents accidental capture. But it also means the macro *cannot* refer to call-site variables unless they're passed in as `$x:ident` and used in the expansion (macro_rules is partially hygienic: locals are hygienic, but idents passed as arguments keep their call-site context).

---

## `macro_rules!` Limitations

- Cannot inspect types (no type-level computation beyond matching).
- Cannot generate identifiers (no `paste!` without a proc macro — use the `paste` crate).
- Recursion depth limits (`recursion_limit`, default 128).
- Error messages are often cryptic.

When you hit these limits, it's time for **procedural macros** (see `03-procedural-macros-intro.md`).

---

## Common Real-World Macros to Study

- `vec!` — `$( $x:expr ),*` repetition
- `format!` / `println!` — format string parsing at compile time (actually a built-in, but similar)
- `matches!` — expression → bool pattern match
- `assert_eq!` — comparison with pretty-printing of both sides
- `lazy_static!` / `once_cell` — initialization patterns (largely superseded by `std::sync::LazyLock`)

---

## Exercises

- `exercises/01-declarative-macros.rs` — TT muncher, internal rules, hygiene tests
