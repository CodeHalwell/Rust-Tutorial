/// ============================================================================
/// GENERIC CONSTRAINTS EXERCISES: Trait Bounds, where Clauses, and Friends
/// ============================================================================
///
/// Practice writing generic functions with trait bounds.
/// Run: rustc --edition 2021 02-generic-constraints.rs -o gc && ./gc

// ============================================================================
// EXERCISE 1: Bound Syntax — inline vs where clause
// ============================================================================

/// Inline bound: T: Display
pub fn print_it<T: std::fmt::Display>(x: T) {
    println!("{}", x);
}

/// Same thing, where clause (better for many bounds):
pub fn print_pair<T, U>(x: T, y: U)
where
    T: std::fmt::Display,
    U: std::fmt::Display,
{
    println!("{} {}", x, y);
}

// ============================================================================
// EXERCISE 2: Multiple Bounds with +
// ============================================================================

/// T must be both Display AND Clone
pub fn show_and_clone<T>(x: T) -> T
where
    T: std::fmt::Display + Clone,
{
    println!("value: {}", x);
    x.clone()
}

// ============================================================================
// EXERCISE 3: Generic Struct with Bounds on the impl
// ============================================================================

pub struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

impl<T: std::fmt::Display> Wrapper<T> {
    pub fn show(&self) {
        println!("wrapped: {}", self.value);
    }
}

impl<T: Clone> Wrapper<T> {
    pub fn duplicate(&self) -> T {
        self.value.clone()
    }
}

// ============================================================================
// EXERCISE 4: Functions Returning impl Trait
// ============================================================================

/// The concrete return type is hidden; callers only know it implements Iterator.
pub fn evens_up_to(n: u32) -> impl Iterator<Item = u32> {
    (0..=n).filter(|x| x % 2 == 0)
}

/// Caveat: impl Trait in return position picks ONE concrete type — you cannot
/// return different types from different branches:
/// ```rust
/// fn bad(cond: bool) -> impl std::fmt::Display {
///     if cond { 1 } else { "two" }  // ❌ mismatched types
/// }
/// ```

// ============================================================================
// EXERCISE 5: Trait Objects as Parameters (dyn)
// ============================================================================

pub trait Describable {
    fn describe(&self) -> String;
}

pub struct Book {
    pub title: String,
}

impl Describable for Book {
    fn describe(&self) -> String {
        format!("Book: {}", self.title)
    }
}

/// Accept any Describable — dynamic dispatch via &dyn
pub fn describe_any(d: &dyn Describable) -> String {
    d.describe()
}

/// Or a slice of trait objects:
pub fn describe_all(items: &[&dyn Describable]) -> Vec<String> {
    items.iter().map(|d| d.describe()).collect()
}

// ============================================================================
// EXERCISE 6: Generic Function with Associated-Type Output
// ============================================================================

/// Sum any iterator of numbers — using the associated type Item.
pub fn sum_iter<I>(iter: I) -> i64
where
    I: Iterator,
    I::Item: Into<i64>,
{
    iter.map(Into::into).sum()
}

// ============================================================================
// EXERCISE 7: Conditional Methods via where Self: Sized
// ============================================================================

pub trait Pipeline: Sized {
    fn step(self) -> Self;

    /// Only available on sized types (not behind dyn) because it returns Self.
    fn twice(self) -> Self
    where
        Self: Sized,
    {
        self.step().step()
    }
}

pub struct Doubler(i64);
impl Pipeline for Doubler {
    fn step(self) -> Self {
        Doubler(self.0 * 2)
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("=== GENERIC CONSTRAINTS EXERCISES ===\n");

    print_it(42);
    print_it("hello");
    print_pair(1, "one");

    let s = show_and_clone(String::from("clone me"));
    println!("cloned: {}", s);

    let w = Wrapper::new(99);
    w.show();
    println!("duplicate: {}", w.duplicate());

    let evens: Vec<u32> = evens_up_to(10).collect();
    println!("evens up to 10: {:?}", evens);

    let book = Book { title: "Rust in Action".into() };
    println!("{}", describe_any(&book));
    println!("{:?}", describe_all(&[&book]));

    println!("sum: {}", sum_iter(vec![1i32, 2, 3, 4].into_iter()));

    let d = Doubler(5).twice();
    println!("doubled twice: {}", d.0);

    println!("\n=== ALL GENERIC EXERCISES COMPLETE ===");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_show_and_clone() {
        assert_eq!(show_and_clone(7), 7);
    }

    #[test]
    fn test_wrapper() {
        let w = Wrapper::new(String::from("x"));
        assert_eq!(w.duplicate(), "x");
    }

    #[test]
    fn test_evens_up_to() {
        assert_eq!(evens_up_to(6).collect::<Vec<_>>(), vec![0, 2, 4, 6]);
    }

    #[test]
    fn test_describe_all() {
        let b = Book { title: "T".into() };
        let out = describe_all(&[&b]);
        assert_eq!(out, vec!["Book: T".to_string()]);
    }

    #[test]
    fn test_sum_iter() {
        assert_eq!(sum_iter([10i64, 20, 12].into_iter()), 42);
    }

    #[test]
    fn test_pipeline_twice() {
        assert_eq!(Doubler(3).twice().0, 12);
    }
}
