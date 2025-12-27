/// ============================================================================
/// ERROR HANDLING DEEP DIVE: Result, Option, and Error Propagation
/// ============================================================================
///
/// Rust's error handling philosophy: Make errors explicit and recoverable.
/// No null pointer exceptions—types force you to handle failure cases.

use std::fs;
use std::io;
use std::num::ParseIntError;

// ============================================================================
// PART 1: OPTION - ABSENCE OF VALUE
// ============================================================================

/// Option<T> represents either Some(value) or None
/// Use when a value might not exist (not an error, just absence)

fn find_first_even(nums: &[i32]) -> Option<i32> {
    for num in nums {
        if num % 2 == 0 {
            return Some(*num);
        }
    }
    None
}

fn demonstrate_option() {
    println!("=== OPTION<T> ===");

    let nums = vec![1, 3, 5, 6, 9];
    match find_first_even(&nums) {
        Some(n) => println!("Found even: {}", n),
        None => println!("No even numbers"),
    }

    // Option provides convenient methods:
    let result = find_first_even(&[1, 3, 5]);

    // map(): transform the value if it exists
    let doubled = result.map(|x| x * 2);
    println!("Doubled: {:?}", doubled);  // None

    // unwrap_or(): provide default
    let value = find_first_even(&[1, 2, 3]).unwrap_or(0);
    println!("Value or default: {}", value);  // 2

    // if let: convenient pattern matching
    if let Some(n) = find_first_even(&[1, 2, 3]) {
        println!("Found: {}", n);
    }
}

// ============================================================================
// PART 2: RESULT - RECOVERABLE ERRORS
// ============================================================================

/// Result<T, E> represents either Ok(value) or Err(error)
/// Use when an operation might fail with an error

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn parse_config(config_str: &str) -> Result<(String, i32), String> {
    let parts: Vec<&str> = config_str.split(':').collect();
    if parts.len() != 2 {
        return Err("Invalid config format".to_string());
    }

    let port: i32 = parts[1]
        .parse()
        .map_err(|_| "Port must be a number".to_string())?;

    Ok((parts[0].to_string(), port))
}

fn demonstrate_result() {
    println!("\n=== RESULT<T, E> ===");

    // Match on Result
    match divide(10, 2) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("Error: {}", e),
    }

    // unwrap (panics on Err)
    let result = divide(10, 2).unwrap();
    println!("Unwrapped: {}", result);

    // unwrap_or (provide default)
    let result = divide(10, 0).unwrap_or(-1);
    println!("With default: {}", result);

    // is_ok / is_err
    let ok_result = divide(10, 2);
    println!("Is ok: {}", ok_result.is_ok());

    // map: transform the value
    let doubled = divide(10, 2).map(|x| x * 2);
    println!("Mapped: {:?}", doubled);

    // and_then: chain operations that return Result
    let chained = divide(10, 2)
        .and_then(|x| divide(x, 2));
    println!("Chained: {:?}", chained);

    // Config parsing
    match parse_config("localhost:8080") {
        Ok((host, port)) => println!("Config: {} on {}", host, port),
        Err(e) => println!("Config error: {}", e),
    }
}

// ============================================================================
// PART 3: THE ? OPERATOR (ERROR PROPAGATION)
// ============================================================================

/// The ? operator is syntactic sugar for error propagation
/// If Result is Err, return that error immediately
/// If Result is Ok, unwrap the value and continue

fn read_file_and_parse(path: &str) -> io::Result<String> {
    let contents = fs::read_to_string(path)?;
    // ? means: if read_to_string returns Err, return that Err
    // If Ok, unwrap the String and continue
    Ok(contents)
}

fn read_and_parse_number(path: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;  // io::Error
    let num: i32 = contents.parse()?;           // ParseIntError
    Ok(num)
}

// Without ?, this is verbose:
fn read_and_parse_number_verbose(path: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let contents = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => return Err(Box::new(e)),
    };

    let num: i32 = match contents.parse() {
        Ok(n) => n,
        Err(e) => return Err(Box::new(e)),
    };

    Ok(num)
}

fn demonstrate_question_operator() {
    println!("\n=== THE ? OPERATOR ===");
    println!("(Skipped file I/O demo, but shows error propagation)");
}

// ============================================================================
// PART 4: CUSTOM ERROR TYPES
// ============================================================================

#[derive(Debug)]
enum ConfigError {
    MissingField(String),
    InvalidPort(ParseIntError),
    IoError(io::Error),
}

/// Implement Display for nice error messages
impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConfigError::MissingField(field) => write!(f, "Missing field: {}", field),
            ConfigError::InvalidPort(e) => write!(f, "Invalid port: {}", e),
            ConfigError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl std::error::Error for ConfigError {}

/// Implement From for automatic conversion
impl From<ParseIntError> for ConfigError {
    fn from(err: ParseIntError) -> Self {
        ConfigError::InvalidPort(err)
    }
}

impl From<io::Error> for ConfigError {
    fn from(err: io::Error) -> Self {
        ConfigError::IoError(err)
    }
}

fn parse_port(s: &str) -> Result<u16, ConfigError> {
    s.parse::<u16>()
        .map_err(ConfigError::InvalidPort)
}

fn demonstrate_custom_errors() {
    println!("\n=== CUSTOM ERROR TYPES ===");

    match parse_port("8080") {
        Ok(port) => println!("Port: {}", port),
        Err(e) => println!("Error: {}", e),
    }

    match parse_port("invalid") {
        Ok(_) => println!("Port: valid"),
        Err(e) => println!("Error: {}", e),
    }
}

// ============================================================================
// PART 5: COMMON PATTERNS
// ============================================================================

/// Pattern 1: Handle most cases, crash on unexpected
fn critical_operation() -> Result<i32, String> {
    Ok(42)
}

fn pattern_unwrap() {
    let value = critical_operation().expect("Critical operation failed!");
    println!("Value: {}", value);
}

/// Pattern 2: Provide default on error
fn pattern_unwrap_or() {
    let value = critical_operation().unwrap_or(-1);
    println!("Value: {}", value);
}

/// Pattern 3: Chain multiple operations
fn pattern_chaining() {
    let result = divide(10, 2)
        .and_then(|x| divide(x, 2))
        .and_then(|x| divide(x, 2))
        .map(|x| x + 1);

    println!("Chained: {:?}", result);
}

/// Pattern 4: Try multiple alternatives
fn try_parse(s: &str) -> Option<i32> {
    // Try parsing as i32, then as float
    s.parse::<i32>()
        .ok()
        .or_else(|| s.parse::<f32>().ok().map(|f| f as i32))
}

fn pattern_alternatives() {
    println!("Parse '42': {:?}", try_parse("42"));
    println!("Parse '3.14': {:?}", try_parse("3.14"));
    println!("Parse 'invalid': {:?}", try_parse("invalid"));
}

// ============================================================================
// PART 6: ITERATOR WITH OPTION/RESULT
// ============================================================================

fn demonstrate_iterator_handling() {
    println!("\n=== ITERATORS WITH OPTION/RESULT ===");

    // Filter: keep only Some values
    let values = vec![Some(1), None, Some(2), None, Some(3)];
    let some_values: Vec<i32> = values
        .iter()
        .filter_map(|v| *v)  // Removes None, extracts Some
        .collect();
    println!("Filtered: {:?}", some_values);

    // Collect into Result (fails if any Err)
    let results = vec![
        Ok(1),
        Ok(2),
        Err("error"),
        Ok(3),
    ];
    let collected: Result<Vec<i32>, &str> = results.into_iter().collect();
    println!("Collected: {:?}", collected);  // Err("error")

    // Successful collection
    let results = vec![Ok(1), Ok(2), Ok(3)];
    let collected: Result<Vec<i32>, String> = results.into_iter().collect();
    println!("Collected: {:?}", collected);  // Ok([1,2,3])
}

// ============================================================================
// PART 7: PANIC VS ERROR HANDLING
// ============================================================================

/// When to panic:
/// - Logic errors (bug in your code)
/// - Impossible conditions (should never happen)
/// - Test code
///
/// When to return Result:
/// - Expected failures (file not found, parse error)
/// - Recoverable errors
/// - Library code

fn maybe_divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn definitely_divide_valid_input(a: i32, b: i32) -> i32 {
    // Caller guarantees b != 0
    assert!(b != 0, "Logic error: b should never be 0");
    a / b
}

fn demonstrate_panic_vs_error() {
    println!("\n=== PANIC VS ERROR HANDLING ===");

    // Recoverable error
    match maybe_divide(10, 0) {
        Ok(r) => println!("Result: {}", r),
        Err(e) => println!("Error: {}", e),
    }

    // Guarantee invariant with assertion
    let result = definitely_divide_valid_input(10, 2);
    println!("Result: {}", result);
}

// ============================================================================
// PART 8: CONVERTING BETWEEN OPTION AND RESULT
// ============================================================================

fn maybe_sqrt(x: f32) -> Option<f32> {
    if x >= 0.0 {
        Some(x.sqrt())
    } else {
        None
    }
}

fn result_sqrt(x: f32) -> Result<f32, String> {
    maybe_sqrt(x).ok_or("Cannot take sqrt of negative".to_string())
}

fn demonstrate_conversions() {
    println!("\n=== OPTION/RESULT CONVERSION ===");

    // Option to Result
    match maybe_sqrt(4.0) {
        Some(r) => println!("Sqrt: {}", r),
        None => println!("No sqrt"),
    }

    match result_sqrt(4.0) {
        Ok(r) => println!("Sqrt: {}", r),
        Err(e) => println!("Error: {}", e),
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("=== ERROR HANDLING DEEP DIVE ===\n");

    demonstrate_option();
    demonstrate_result();
    demonstrate_question_operator();
    demonstrate_custom_errors();

    println!("\n=== COMMON PATTERNS ===");
    pattern_unwrap();
    pattern_unwrap_or();
    pattern_chaining();
    pattern_alternatives();

    demonstrate_iterator_handling();
    demonstrate_panic_vs_error();
    demonstrate_conversions();

    println!("\n=== ERROR HANDLING SUMMARY ===");
    println!("- Option<T>: Value might be missing (not an error)");
    println!("- Result<T,E>: Operation might fail with error");
    println!("- ?: Propagate errors up the call stack");
    println!("- match/if let: Handle error cases explicitly");
    println!("- unwrap/expect: Crash on error (use carefully)");
    println!("- Custom error types: Implement Error + Display");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_even() {
        assert_eq!(find_first_even(&[1, 2, 3]), Some(2));
        assert_eq!(find_first_even(&[1, 3, 5]), None);
    }

    #[test]
    fn test_divide_success() {
        assert_eq!(divide(10, 2), Ok(5));
    }

    #[test]
    fn test_divide_by_zero() {
        assert!(divide(10, 0).is_err());
    }

    #[test]
    fn test_parse_config_valid() {
        assert_eq!(parse_config("localhost:8080"), Ok(("localhost".to_string(), 8080)));
    }

    #[test]
    fn test_parse_config_invalid() {
        assert!(parse_config("localhost").is_err());
    }

    #[test]
    fn test_option_map() {
        let opt = Some(5);
        let mapped = opt.map(|x| x * 2);
        assert_eq!(mapped, Some(10));
    }

    #[test]
    fn test_result_and_then() {
        let result = Ok(10)
            .and_then(|x| Ok(x / 2))
            .and_then(|x| Ok(x + 1));
        assert_eq!(result, Ok(6));
    }

    #[test]
    fn test_try_parse() {
        assert_eq!(try_parse("42"), Some(42));
        assert_eq!(try_parse("3.14"), Some(3)); // Truncates
    }
}
