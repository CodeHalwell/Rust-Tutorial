/// ============================================================================
/// CUSTOM DERIVE EXERCISE
/// ============================================================================
///
/// Part A (runs standalone with rustc): a macro_rules!-based approximation
/// of a derive — you invoke it manually instead of via #[derive(...)]:
///
/// Run: rustc --edition 2021 03-custom-derive.rs -o cd && ./cd
///
/// Part B (needs cargo): the REAL derive from 02-procedural-macro-crate.
/// In a separate crate that depends on `my-macros` (path dependency):
///
/// ```toml
/// # Cargo.toml of the consumer crate
/// [dependencies]
/// my-macros = { path = "../02-procedural-macro-crate" }
/// ```
///
/// ```rust
/// use my_macros::TypeName;
///
/// #[derive(TypeName)]
/// struct Account { id: u64 }
///
/// fn main() {
///     assert_eq!(Account::type_name(), "Account");
/// }
/// ```
///
/// For a full-featured derive (generics, named-field inspection, attributes)
/// use syn + quote — see notes/04-syn-and-quote.md and notes/05-derive-macros.md.

// ============================================================================
// PART A: macro_rules! approximation of #[derive(TypeName)]
// ============================================================================

pub trait TypeName {
    fn type_name() -> &'static str;
}

/// Usage: impl_type_name!(MyStruct);
macro_rules! impl_type_name {
    ($t:ty) => {
        impl TypeName for $t {
            fn type_name() -> &'static str {
                stringify!($t)
            }
        }
    };
}

pub struct Account {
    pub id: u64,
}

pub struct Order {
    pub sku: String,
}

impl_type_name!(Account);
impl_type_name!(Order);

// ============================================================================
// PART B: a slightly fancier derive-like macro — field names via macro_rules
// ============================================================================

/// impl_fields!(Point { x, y }) generates a `field_names()` method.
macro_rules! impl_fields {
    ($t:ident { $( $field:ident ),* $(,)? }) => {
        impl $t {
            pub fn field_names() -> &'static [&'static str] {
                &[ $( stringify!($field) ),* ]
            }
        }
    };
}

pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl_fields!(Point { x, y });

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("=== CUSTOM DERIVE EXERCISE ===\n");
    println!("Account::type_name() = {}", Account::type_name());
    println!("Order::type_name() = {}", Order::type_name());
    println!("Point::field_names() = {:?}", Point::field_names());
    println!("\n=== COMPLETE ===");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_name_macro() {
        assert_eq!(Account::type_name(), "Account");
        assert_eq!(Order::type_name(), "Order");
    }

    #[test]
    fn test_field_names_macro() {
        assert_eq!(Point::field_names(), &["x", "y"]);
    }
}
