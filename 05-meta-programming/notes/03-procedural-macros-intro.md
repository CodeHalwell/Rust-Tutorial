# Procedural Macros: Intro

## What Procedural Macros Are

A **procedural macro** is a Rust function that runs at compile time and transforms a `TokenStream` into another `TokenStream`:

```rust
// In a proc-macro crate:
use proc_macro::TokenStream;

#[proc_macro]
pub fn my_macro(input: TokenStream) -> TokenStream {
    // input: the tokens inside my_macro!( ... )
    // return: the tokens that replace the invocation
    input
}
```

**Key insight:** Unlike `macro_rules!`, proc macros are *functions over syntax* — you can parse, inspect, generate identifiers, and produce arbitrary code.

---

## The Three Kinds

| Kind | Syntax | Input | Typical use |
|---|---|---|---|
| Function-like | `my_macro!(...)` | token stream | DSLs, code generation |
| Attribute | `#[my_attr] fn f() {}` | the annotated item | augment/replace items |
| Derive | `#[derive(MyTrait)] struct S;` | the struct/enum | implement traits automatically |

```rust
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream { ... }

#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream { ... }

#[proc_macro_derive(Builder)]
pub fn derive_builder(item: TokenStream) -> TokenStream { ... }
```

---

## The `proc_macro` Crate (std for proc macros)

Only usable inside a `proc-macro = true` crate. Key types:

```rust
use proc_macro::{TokenStream, TokenTree, Ident, Span, Group, Delimiter, Punct, Literal};

#[proc_macro]
pub fn dump(input: TokenStream) -> TokenStream {
    for tt in input {
        match tt {
            TokenTree::Ident(i) => println!("ident: {}", i),
            TokenTree::Literal(l) => println!("literal: {}", l),
            TokenTree::Punct(p) => println!("punct: {}", p),
            TokenTree::Group(g) => println!("group {:?}: {:?}", g.delimiter(), g.stream()),
        }
    }
    TokenStream::new()
}
```

- `TokenStream` — sequence of `TokenTree`s.
- `TokenTree` — `Ident`, `Punct`, `Literal`, or `Group` (delimited `(...)`, `[...]`, `{...}`).
- `Span` — source location, used for error reporting (`span.error("msg").emit()` via `compile_error!` or the `Span::source_file` APIs).

---

## Crate Setup

```toml
# Cargo.toml
[package]
name = "my-macros"
version = "0.1.0"
edition = "2021"

[lib]
proc-macro = true
```

```rust
// src/lib.rs
use proc_macro::TokenStream;

#[proc_macro]
pub fn hello(input: TokenStream) -> TokenStream {
    format!("println!(\"hello from a proc macro, got {} tokens\");", input.into_iter().count())
        .parse()
        .unwrap()
}
```

Usage from another crate:

```rust
use my_macros::hello;
fn main() {
    hello!(a b c);  // prints: hello from a proc macro, got 3 tokens
}
```

**Key insight:** A proc-macro crate cannot export anything except proc macros — that's why real projects split `my-macros` (proc-macro) from `my-macros-impl` or use it only for macros.

---

## Hand-Rolling vs syn/quote

Parsing `TokenStream` by hand is tedious. The standard stack:

- **`syn`** — full Rust syntax parser: `syn::parse::<syn::DeriveInput>(input)`.
- **`quote`** — build token streams with `quote! { ... }` and `#var` interpolation.
- **`proc-macro2`** — a wrapper around `proc_macro` that works outside proc-macro crates (testable!).

See `04-syn-and-quote.md`. The exercise crate in `exercises/02-procedural-macro-crate/` uses the *dependency-free* `proc_macro` API so it builds offline; production code should use syn/quote.

---

## Debugging Proc Macros

- `cargo expand` — shows expanded output.
- `eprintln!` inside the macro — prints at compile time (shows with `cargo build -vv` or when compilation fails).
- `proc_macro::Span` + `compile_error!` for good error messages.
- Test proc macros with `trybuild` (UI tests) or plain integration tests.

---

## Exercises

- `exercises/02-procedural-macro-crate/` — a working dependency-free proc-macro crate
- `exercises/03-custom-derive.rs`
