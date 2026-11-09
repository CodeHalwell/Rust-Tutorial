# macro_rules! Fundamentals

## What Declarative Macros Are

`macro_rules!` macros match on **token trees** and expand to new tokens — pattern matching on syntax, not on values:

```rust
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

fn main() {
    say_hello!();   // expands to println!("Hello!");
}
```

**Key insight:** A macro is invoked with `!` and expands *before* type checking. The macro must produce valid tokens; the compiler then checks them.

---

## Match Arms and Metavariables

Each `(pattern) => { expansion }` is an arm. Metavariables capture fragments:

| Fragment specifier | Matches | Example |
|---|---|---|
| `$x:expr` | an expression | `1 + 2`, `foo()` |
| `$x:stmt` | a statement | `let a = 1;` |
| `$x:ident` | an identifier | `hello` |
| `$x:ty` | a type | `Vec<u8>` |
| `$x:pat` | a pattern | `Some(x)` |
| `$x:path` | a path | `std::vec::Vec` |
| `$x:literal` | a literal | `42`, `"hi"` |
| `$x:tt` | a single token tree | `(a + b)` |
| `$x:block` | a block | `{ ... }` |
| `$x:item` | an item | `fn f() {}` |
| `$x:meta` | a meta item (attribute contents) | `derive(Debug)` |

```rust
macro_rules! create_fn {
    ($name:ident) => {
        fn $name() {
            println!("function {:?}() created", stringify!($name));
        }
    };
}

create_fn!(foo);
fn main() {
    foo();  // prints: function "foo"() created
}
```

`stringify!($name)` turns the captured identifier back into a string literal.

---

## Multiple Arms: Try in Order

The macro tries arms top-to-bottom, first match wins:

```rust
macro_rules! describe {
    ($x:expr) => { println!("expr: {}", stringify!($x)) };
    ($x:ident) => { println!("ident: {}", stringify!($x)) };
    () => { println!("nothing") };
}
```

---

## Repetition: `$( ... )*`, `+`, `?`

```rust
macro_rules! vec_of {
    // $( $elem:expr ),*  — comma-separated list of exprs, zero or more
    ( $( $elem:expr ),* $(,)? ) => {
        {
            let mut v = Vec::new();
            $( v.push($elem); )*
            v
        }
    };
}

let v = vec_of![1, 2, 3,];  // trailing comma allowed via $(,)?
```

- `$(...)*` — zero or more
- `$(...)+` — one or more
- `$(...)?` — zero or one

Separator goes inside: `$( $x:expr ),*` means "exprs separated by commas". Nested repetition must use the same variable at each depth.

---

## A Real Example: A HashMap Macro

```rust
macro_rules! hashmap {
    ( $( $key:expr => $value:expr ),* $(,)? ) => {{
        let mut m = std::collections::HashMap::new();
        $( m.insert($key, $value); )*
        m
    }};
}

let m = hashmap! { "a" => 1, "b" => 2 };
```

---

## Debugging Macros

- `cargo expand` — see the full expansion.
- `rustc -Zunpretty=expanded` (nightly) — same without cargo.
- `trace_macros!(true);` (nightly) — logs macro expansion at compile time.

**Key insight:** If a macro doesn't expand as expected, the error often appears at the *call site* with confusing spans. Use `cargo expand` to see the truth.

---

## Exercises

- `exercises/01-declarative-macros.rs`
