//! Integration tests for the proc-macro crate.
//! Run from this directory with: cargo test

use my_macros::{banner, TypeName};

#[derive(TypeName)]
struct User {
    name: String,
}

#[derive(TypeName)]
enum Status {
    Active,
    Inactive,
}

#[test]
fn test_derive_type_name_struct() {
    assert_eq!(User::type_name(), "User");
}

#[test]
fn test_derive_type_name_enum() {
    assert_eq!(Status::type_name(), "Status");
}

#[test]
fn test_banner_macro_runs() {
    banner!("integration test");
}
