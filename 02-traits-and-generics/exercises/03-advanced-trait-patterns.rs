/// ============================================================================
/// ADVANCED TRAIT PATTERNS: Associated Types, HRTBs, Variance in Practice
/// ============================================================================
///
/// Run: rustc --edition 2021 03-advanced-trait-patterns.rs -o at && ./at

// ============================================================================
// EXERCISE 1: Associated Types — a type-safe unit system
// ============================================================================

pub trait Quantity {
    type Unit: std::fmt::Display;
    fn value(&self) -> f64;
    fn unit(&self) -> Self::Unit;
}

pub struct Meters(pub f64);
impl std::fmt::Display for Meters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}m", self.0)
    }
}

pub struct Seconds(pub f64);
impl std::fmt::Display for Seconds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}s", self.0)
    }
}

pub struct Length(pub f64);
impl Quantity for Length {
    type Unit = Meters;
    fn value(&self) -> f64 { self.0 }
    fn unit(&self) -> Meters { Meters(self.0) }
}

pub struct Duration(pub f64);
impl Quantity for Duration {
    type Unit = Seconds;
    fn value(&self) -> f64 { self.0 }
    fn unit(&self) -> Seconds { Seconds(self.0) }
}

/// Only same-unit quantities can be added — enforced at compile time,
/// because add takes two &Length and returns Length.
pub fn add_lengths(a: &Length, b: &Length) -> Length {
    Length(a.0 + b.0)
}

/// Generic over any Quantity — the associated type flows through:
pub fn show_quantity<Q: Quantity>(q: &Q) -> String {
    format!("{} {}", q.value(), q.unit())
}

// ============================================================================
// EXERCISE 2: Higher-Ranked Trait Bounds
// ============================================================================

/// Accepts a closure that works with a &str of ANY lifetime.
pub fn apply_to_str<F>(f: F, inputs: &[&str]) -> Vec<bool>
where
    F: for<'a> Fn(&'a str) -> bool,
{
    inputs.iter().map(|s| f(s)).collect()
}

/// HRTB with a reference returned: identity for any lifetime.
pub fn hrtb_identity<F>(f: F, s: &str) -> &str
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
    f(s)
}

/// Without for<'a>, a closure that must work for multiple lifetimes fails:
/// this compiles BECAUSE of the HRTB.
pub fn check_all<F>(f: F) -> bool
where
    F: for<'a> Fn(&'a str) -> bool,
{
    f("short") && f("a considerably longer input string")
}

// ============================================================================
// EXERCISE 3: Variance — covariance of &T and 'static
// ============================================================================

/// 'static coerces to any shorter lifetime (covariance of lifetimes):
pub fn accepts_any_lifetime<'a>(s: &'a str) -> &'a str {
    s
}

pub fn exercise_covariance() -> &'static str {
    let s: &'static str = "lives forever";
    let _shorter: &str = accepts_any_lifetime(s); // 'static -> shorter OK
    s
}

/// &T is covariant in T only via lifetime subtyping; &mut T is invariant in T.
/// Demonstrate lifetime covariance with &mut:
pub fn shorten_mut<'long, 'short>(r: &'long mut i32) -> &'short mut i32
where
    'long: 'short,
{
    r // ✅ allowed: lifetime may shrink, but the type i32 may NOT change
}

// ============================================================================
// EXERCISE 4: Fully Qualified Syntax
// ============================================================================

pub trait Producer {
    type Item;
    fn produce(&self) -> Self::Item;
}

pub struct IntProducer;
impl Producer for IntProducer {
    type Item = i32;
    fn produce(&self) -> i32 { 42 }
}

pub fn produce_fq<P: Producer>(p: &P) -> <P as Producer>::Item {
    p.produce()
}

// ============================================================================
// EXERCISE 5: Blanket Implementations
// ============================================================================

/// Any T that is Display gets a ToString-like helper for free.
pub trait Shout {
    fn shout(&self) -> String;
}

impl<T: std::fmt::Display> Shout for T {
    fn shout(&self) -> String {
        format!("{}!!!", self)
    }
}

// ============================================================================
// EXERCISE 6: where Clause with Associated Type Bounds
// ============================================================================

use std::fmt::Debug;

pub fn describe_iter<I>(iter: I) -> Vec<String>
where
    I: Iterator,
    I::Item: Debug,
{
    iter.map(|x| format!("{:?}", x)).collect()
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("=== ADVANCED TRAIT PATTERNS ===\n");

    let l1 = Length(1.5);
    let l2 = Length(2.5);
    let total = add_lengths(&l1, &l2);
    println!("lengths: {} + {} = {}", show_quantity(&l1), show_quantity(&l2), show_quantity(&total));
    println!("duration: {}", show_quantity(&Duration(9.5)));

    let results = apply_to_str(|s| s.len() > 3, &["hi", "hello", "hey"]);
    println!("apply_to_str: {:?}", results);

    let s = String::from("borrowed");
    println!("hrtb_identity: {}", hrtb_identity(|x| x, &s));
    println!("check_all: {}", check_all(|s| !s.is_empty()));

    println!("covariance: {}", exercise_covariance());
    let mut n = 7;
    {
        let r = shorten_mut(&mut n);
        *r += 1;
    }
    println!("shorten_mut: {}", n);

    println!("produce_fq: {}", produce_fq(&IntProducer));

    println!("shout: {}", 42.shout());

    let items = describe_iter(vec![1, 2, 3].into_iter());
    println!("describe_iter: {:?}", items);

    println!("\n=== ALL ADVANCED PATTERN EXERCISES COMPLETE ===");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_lengths() {
        assert_eq!(add_lengths(&Length(1.0), &Length(2.0)).0, 3.0);
    }

    #[test]
    fn test_show_quantity() {
        assert_eq!(show_quantity(&Length(2.0)), "2 2m");
        assert_eq!(show_quantity(&Duration(3.0)), "3 3s");
    }

    #[test]
    fn test_hrtb() {
        let out = apply_to_str(|s| s.starts_with('a'), &["abc", "xyz", "axe"]);
        assert_eq!(out, vec![true, false, true]);
        assert!(check_all(|s| s.len() < 100));
    }

    #[test]
    fn test_produce_fq() {
        assert_eq!(produce_fq(&IntProducer), 42);
    }

    #[test]
    fn test_blanket_shout() {
        assert_eq!(7.shout(), "7!!!");
    }

    #[test]
    fn test_describe_iter() {
        assert_eq!(describe_iter(vec![1, 2].into_iter()), vec!["1", "2"]);
    }
}
