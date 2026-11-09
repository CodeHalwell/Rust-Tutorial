//! A minimal procedural-macro crate using ONLY the built-in `proc_macro`
//! API — no syn/quote — so it builds fully offline.
//!
//! Provides:
//! - `banner!("text")`   — function-like macro printing a banner
//! - `#[derive(TypeName)]` — adds `fn type_name() -> &'static str`
//!
//! In production you would use `syn` + `quote` (see notes/04-syn-and-quote.md);
//! they make parsing and generation far more robust. This crate shows the
//! raw mechanics so you understand what syn/quote do for you.

use proc_macro::{TokenStream, TokenTree};

/// `banner!("hello")` expands to `println!("=== hello ===");`
#[proc_macro]
pub fn banner(input: TokenStream) -> TokenStream {
    let text = input.to_string();
    let inner = text.trim().trim_matches('"');
    format!("println!(\"=== {} ===\");", inner)
        .parse()
        .expect("failed to build banner expansion")
}

/// `#[derive(TypeName)]` on a struct or enum generates:
/// `impl TheType { pub fn type_name() -> &'static str { "TheType" } }`
///
/// Limitations of this hand-rolled version (fixed by syn in real code):
/// - no support for generics or where clauses
/// - finds the name by scanning for the `struct`/`enum` keyword
#[proc_macro_derive(TypeName)]
pub fn derive_type_name(input: TokenStream) -> TokenStream {
    let mut keyword_seen = false;
    let mut name: Option<String> = None;

    for tt in input {
        if let TokenTree::Ident(id) = tt {
            let s = id.to_string();
            if keyword_seen {
                name = Some(s);
                break;
            }
            if s == "struct" || s == "enum" {
                keyword_seen = true;
            }
        }
    }

    let name = match name {
        Some(n) => n,
        None => {
            return "compile_error!(\"TypeName: could not find a type name after `struct`/`enum`\");"
                .parse()
                .unwrap()
        }
    };

    format!(
        "impl {name} {{ pub fn type_name() -> &'static str {{ \"{name}\" }} }}",
        name = name
    )
    .parse()
    .expect("failed to build TypeName expansion")
}
