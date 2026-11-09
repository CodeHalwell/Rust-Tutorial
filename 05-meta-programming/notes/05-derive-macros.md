# Derive Macros

## What Derive Does

`#[derive(Debug)]` is itself a proc macro: it reads your struct and generates `impl Debug for YourStruct`. You can write your own:

```rust
#[derive(MyTrait)]
struct Point { x: f64, y: f64 }
```

expands (conceptually) to:

```rust
struct Point { x: f64, y: f64 }
impl MyTrait for Point { /* generated */ }
```

---

## A Minimal Derive (with syn/quote)

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(TypeName)]
pub fn derive_type_name(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            pub fn type_name() -> &'static str {
                stringify!(#name)
            }
        }
    };
    expanded.into()
}
```

Usage:

```rust
#[derive(TypeName)]
struct User { name: String }

fn main() {
    println!("{}", User::type_name());  // "User"
}
```

---

## Deriving Trait Impls: Field Inspection

```rust
#[proc_macro_derive(FieldNames)]
pub fn derive_field_names(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let fields = match &ast.data {
        syn::Data::Struct(s) => match &s.fields {
            syn::Fields::Named(f) => f.named.iter().map(|f| f.ident.as_ref().unwrap()).collect::<Vec<_>>(),
            _ => return syn::Error::new_spanned(&ast, "named fields only").to_compile_error().into(),
        },
        _ => return syn::Error::new_spanned(&ast, "structs only").to_compile_error().into(),
    };
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            pub fn field_names() -> &'static [&'static str] {
                &[#(stringify!(#fields)),*]
            }
        }
    };
    expanded.into()
}
```

---

## Helper Attributes

A derive can declare extra attributes it understands, so users can annotate fields:

```rust
#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive_builder(input: TokenStream) -> TokenStream { ... }
```

```rust
#[derive(Builder)]
struct Config {
    #[builder(default = 8080)]
    port: u16,
}
```

In the macro, inspect `field.attrs` for `#[builder(...)]` via `attr.parse_args::<...>()` or `attr.path().is_ident("builder")`.

---

## Common Derives to Study

| Derive | Crate | Generates |
|---|---|---|
| `Debug`, `Clone`, `Default`... | std | standard trait impls |
| `Serialize` / `Deserialize` | serde | (de)serialization impls |
| `Builder` | derive_builder | builder pattern |
| `Error` | thiserror | `std::error::Error` impls with `#[error("...")]` messages |
| `Display` | derive_more | Display from a format string |
| `From` / `Into` | derive_more | conversions |

---

## Rules and Pitfalls

1. **Never panic in a derive** — return `syn::Error::to_compile_error()` so the user gets a real error, not a compiler ICE.
2. **Respect generics** — always `split_for_impl()`.
3. **Don't conflict with std derives** — users can combine `#[derive(Debug, MyDerive)]`; your macro only sees the item, not other derives.
4. **Attribute macros vs derive** — attribute macros *replace/augment* the item; derive macros only *add* an impl.

---

## Exercises

- `exercises/02-procedural-macro-crate/` — build a derive with the plain `proc_macro` API (no syn)
- `exercises/03-custom-derive.rs` — usage + a macro_rules! approximation you can run standalone
