# syn and quote: The Standard Proc-Macro Stack

## Why syn/quote

Hand-parsing `TokenStream` is painful. `syn` parses Rust syntax into AST types; `quote` builds output tokens ergonomically. `proc-macro2` provides a testable token type usable outside proc-macro crates.

```toml
[dependencies]
syn = { version = "2", features = ["full", "derive"] }
quote = "1"
proc-macro2 = "1"
```

---

## syn: Parsing

```rust
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Data, Fields};

#[proc_macro_derive(Describe)]
pub fn derive_describe(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let expanded = quote::quote! {
        impl #name {
            pub fn type_name() -> &'static str {
                stringify!(#name)
            }
        }
    };
    expanded.into()
}
```

Key `syn` types:

| Type | Meaning |
|---|---|
| `DeriveInput` | a struct/enum/union being derived on (has `.ident`, `.data`, `.generics`) |
| `Data::Struct(DataStruct { fields, .. })` | struct with `Fields::Named` / `Fields::Unnamed` / `Fields::Unit` |
| `ItemFn` | a function item |
| `ItemStruct` | a struct item |
| `Expr`, `Type`, `Path`, `Lit` | expressions, types, paths, literals |

`parse_macro_input!(input as T)` handles parse errors and returns a compiler error automatically.

---

## quote: Generating Tokens

```rust
use quote::quote;

let name = Ident::new("MyStruct", Span::call_site());
let fields = vec![Ident::new("x", Span::call_site()), Ident::new("y", Span::call_site())];

let output = quote! {
    impl #name {
        pub fn field_names() -> &'static [&'static str] {
            &[#(stringify!(#fields)),*]
        }
    }
};
```

- `#var` — interpolate a variable implementing `ToTokens`.
- `#(...)*` — repetition, like macro_rules.
- `#var` inside repetition iterates if `var` is an iterator.

---

## Handling Generics Correctly

When generating an impl for a generic type, you must split generics:

```rust
use syn::{parse_macro_input, DeriveInput};
use quote::quote;

#[proc_macro_derive(Clone2)]
pub fn derive_clone2(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics Clone for #name #ty_generics #where_clause {
            fn clone(&self) -> Self {
                Self { /* ... */ }
            }
        }
    };
    expanded.into()
}
```

**Key insight:** `split_for_impl()` gives you the three pieces you need: `impl<T> Trait for Name<T> where ...`. Forgetting this is the #1 proc-macro bug with generics.

---

## Errors

Return compile errors instead of panicking:

```rust
use syn::{parse_macro_input, DeriveInput, Data};
use quote::quote;

#[proc_macro_derive(Builder)]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let fields = match &ast.data {
        Data::Struct(data) => match &data.fields {
            syn::Fields::Named(f) => &f.named,
            _ => return syn::Error::new_spanned(&ast, "only structs with named fields")
                .to_compile_error()
                .into(),
        },
        _ => return syn::Error::new_spanned(&ast, "only structs").to_compile_error().into(),
    };
    // ... build with quote! ...
    let _ = fields;
    quote! {}.into()
}
```

`syn::Error::new_spanned(tokens, msg).to_compile_error()` produces a nice `compile_error!` at the right span.

---

## proc-macro2: Testable Tokens

```rust
#[cfg(test)]
mod tests {
    use quote::quote;

    #[test]
    fn test_output() {
        let ts = quote! { fn hello() {} };
        assert_eq!(ts.to_string(), "fn hello() { }");
    }
}
```

Because `proc-macro2::TokenStream` works outside the compiler, you can unit-test your macro logic.

---

## Workflow Summary

1. `syn::parse` (or `parse_macro_input!`) the input into an AST.
2. Inspect/transform with normal Rust code.
3. `quote!` the output.
4. Convert to `proc_macro::TokenStream` and return.
5. Test with integration tests / `trybuild`.

---

## Exercises

- `exercises/02-procedural-macro-crate/` — dependency-free version of this workflow
- `exercises/03-custom-derive.rs`
