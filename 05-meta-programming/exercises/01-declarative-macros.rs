/// ============================================================================
/// DECLARATIVE MACRO EXERCISES (macro_rules!)
/// ============================================================================
///
/// Build your own versions of vec!, hashmap!, a TT muncher, and more.
/// Run: rustc --edition 2021 01-declarative-macros.rs -o dm && ./dm

// ============================================================================
// EXERCISE 1: my_vec! — like vec![...]
// ============================================================================

macro_rules! my_vec {
    ( $( $elem:expr ),* $(,)? ) => {{
        #[allow(unused_mut)]
        let mut v = Vec::new();
        $( v.push($elem); )*
        v
    }};
}

// ============================================================================
// EXERCISE 2: my_hashmap! — key => value pairs
// ============================================================================

macro_rules! my_hashmap {
    ( $( $key:expr => $value:expr ),* $(,)? ) => {{
        let mut m = std::collections::HashMap::new();
        $( m.insert($key, $value); )*
        m
    }};
}

// ============================================================================
// EXERCISE 3: count_tt! — TT muncher counting token trees
// ============================================================================

macro_rules! count_tt {
    () => { 0usize };
    ($first:tt $($rest:tt)*) => { 1 + count_tt!($($rest)*) };
}

// ============================================================================
// EXERCISE 4: sum! — internal rules with @acc accumulator
// ============================================================================

macro_rules! my_sum {
    ($($x:expr),* $(,)?) => { my_sum!(@acc 0usize, $($x),*) };
    (@acc $acc:expr, $x:expr $(, $rest:expr)* $(,)?) => {
        my_sum!(@acc $acc + $x $(, $rest)*)
    };
    (@acc $acc:expr $(,)?) => { $acc };
}

// ============================================================================
// EXERCISE 5: hygiene demo — macro-local variable must not leak
// ============================================================================

macro_rules! using_temp {
    ($e:expr) => {{
        let temp = 1000; // hygienic: does not touch outer `temp`
        $e + temp
    }};
}

// ============================================================================
// EXERCISE 6: my_matches! — like std matches!
// ============================================================================

macro_rules! my_matches {
    ($expr:expr, $pat:pat $(if $guard:expr)? $(,)?) => {
        match $expr {
            $pat $(if $guard)? => true,
            _ => false,
        }
    };
}

// ============================================================================
// EXERCISE 7: a mini DSL — define simple "routes"
// ============================================================================

macro_rules! routes {
    ( $( $method:ident $path:literal => $handler:expr );* $(;)? ) => {{
        let mut r: Vec<(&str, &str, fn() -> String)> = Vec::new();
        $( r.push((stringify!($method), $path, $handler)); )*
        r
    }};
}

fn hello() -> String { "hello".to_string() }
fn bye() -> String { "bye".to_string() }

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("=== DECLARATIVE MACRO EXERCISES ===\n");

    let v = my_vec![1, 2, 3,];
    println!("my_vec!: {:?}", v);

    let m = my_hashmap! { "a" => 1, "b" => 2 };
    println!("my_hashmap!: {:?}", m);

    println!("count_tt!(a b (c d) [e]) = {}", count_tt!(a b (c d) [e]));
    println!("my_sum!(1, 2, 3, 4) = {}", my_sum!(1, 2, 3, 4));

    let temp = 5;
    println!("using_temp!(1) = {} (outer temp still {})", using_temp!(1), temp);

    println!("my_matches!(Some(3), Some(x) if x > 2) = {}", my_matches!(Some(3), Some(x) if x > 2));
    println!("my_matches!(None::<i32>, Some(_)) = {}", my_matches!(None::<i32>, Some(_)));

    let r = routes! {
        GET "/" => hello;
        GET "/bye" => bye;
    };
    for (method, path, handler) in &r {
        println!("{} {} -> {}", method, path, handler());
    }

    println!("\n=== ALL DECLARATIVE MACRO EXERCISES COMPLETE ===");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_vec() {
        assert_eq!(my_vec![1, 2, 3], vec![1, 2, 3]);
        let empty: Vec<i32> = my_vec![];
        assert!(empty.is_empty());
    }

    #[test]
    fn test_my_hashmap() {
        let m = my_hashmap! { "x" => 10 };
        assert_eq!(m.get("x"), Some(&10));
    }

    #[test]
    fn test_count_tt() {
        assert_eq!(count_tt!(), 0);
        assert_eq!(count_tt!(a b c), 3);
        assert_eq!(count_tt!(a (b c) d), 3);
    }

    #[test]
    fn test_my_sum() {
        assert_eq!(my_sum!(), 0);
        assert_eq!(my_sum!(5), 5);
        assert_eq!(my_sum!(1, 2, 3), 6);
    }

    #[test]
    fn test_hygiene() {
        let temp = 5;
        assert_eq!(using_temp!(1), 1001);
        assert_eq!(temp, 5); // untouched
    }

    #[test]
    fn test_my_matches() {
        assert!(my_matches!(Some(5), Some(_)));
        assert!(!my_matches!(None::<i32>, Some(_)));
        assert!(my_matches!(7, x if x % 2 == 1));
    }

    #[test]
    fn test_routes() {
        let r = routes! { GET "/" => hello; POST "/x" => bye; };
        assert_eq!(r.len(), 2);
        assert_eq!(r[0].0, "GET");
        assert_eq!(r[1].2(), "bye");
    }
}
