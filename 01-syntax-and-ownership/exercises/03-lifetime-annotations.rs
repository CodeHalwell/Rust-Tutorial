/// ============================================================================
/// LIFETIME ANNOTATIONS: Write the Lifetimes Yourself
/// ============================================================================
///
/// Lifetimes are the compiler's way of encoding "how long is this reference
/// valid" as part of the type. This file makes elision rules explicit.
///
/// Run: rustc --edition 2021 03-lifetime-annotations.rs -o la && ./la

// ============================================================================
// EXERCISE 1: First Word (elided vs explicit)
// ============================================================================

/// Elided form (what you normally write):
/// ```rust
/// fn first_word(s: &str) -> &str { ... }
/// ```
///
/// Explicit form — the compiler expands the above to:
/// ```rust
/// fn first_word<'a>(s: &'a str) -> &'a str { ... }
/// ```
///
/// Meaning: the returned slice lives exactly as long as the input slice.

pub fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    s
}

// ============================================================================
// EXERCISE 2: Two Inputs, One Output — why elision fails here
// ============================================================================

/// Elision CANNOT infer this — ambiguous: should the output borrow from x
/// or from y? The compiler refuses:
/// ```rust
/// fn longest(x: &str, y: &str) -> &str { ... }  // ❌ missing lifetime
/// ```
///
/// Fix: annotate explicitly. We choose the output to live as long as the
/// SHORTER input (the common intersection):

pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

/// With two DIFFERENT lifetimes we can be more precise:
/// the result may borrow from either input, so it needs the intersection:
/// ```rust
/// fn longest2<'a, 'b>(x: &'a str, y: &'b str) -> &'??? str
/// ```
/// The only sound choice is the shorter one, written as:
/// `fn longest2<'a>(x: &'a str, y: &'a str) -> &'a str` (callers unify),
/// or with two params where output ties to x only:
pub fn longest_of_x<'a, 'b>(x: &'a str, _y: &'b str) -> &'a str {
    x
}

// ============================================================================
// EXERCISE 3: Struct Holding a Reference
// ============================================================================

/// A struct that borrows must declare the lifetime:
/// ```rust
/// struct Excerpt<'a> {
///     part: &'a str,
/// }
/// ```
///
/// The lifetime parameter tells the compiler: an Excerpt can never outlive
/// the string it points into.

pub struct Excerpt<'a> {
    pub part: &'a str,
}

impl<'a> Excerpt<'a> {
    pub fn new(part: &'a str) -> Self {
        Excerpt { part }
    }

    /// Elision applies to methods too: &self gets its own lifetime, and the
    /// output borrows from self:
    /// `fn announce(&self) -> &str` desugars to
    /// `fn announce<'s>(&'s self) -> &'s str`
    pub fn announce(&self) -> &str {
        self.part
    }
}

// ============================================================================
// EXERCISE 4: 'static — the longest possible lifetime
// ============================================================================

/// 'static means "lives for the entire program" — e.g. string literals are
/// baked into the binary:
/// ```rust
/// let s: &'static str = "I live forever";
/// ```
///
/// Common gotcha: a &'static str does NOT mean "immutable" — it means
/// "valid forever". Owned Strings can also produce 'static references via
/// leaking (Box::leak), but that's a memory leak — avoid it.

pub fn static_str() -> &'static str {
    "I live in the binary"
}

// ============================================================================
// EXERCISE 5: Lifetime Bounds on Generics
// ============================================================================

/// `T: 'a` means "T contains no references shorter than 'a":
/// ```rust
/// fn store<'a, T: 'a>(container: &'a mut Vec<T>, item: T) { container.push(item); }
/// ```

pub fn store<'a, T: 'a>(container: &'a mut Vec<T>, item: T) {
    container.push(item);
}

// ============================================================================
// EXERCISE 6: Putting It Together — a parser struct
// ============================================================================

pub struct Parser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser { input, pos: 0 }
    }

    /// Returns the next token as a slice of the ORIGINAL input — the output
    /// lifetime is tied to 'a (the input), not to &self. That's why we can
    /// return data that outlives the parser itself.
    pub fn next_token(&mut self) -> Option<&'a str> {
        let rest = &self.input[self.pos..];
        match rest.find(' ') {
            Some(i) => {
                let tok = &rest[..i];
                self.pos += i + 1;
                Some(tok)
            }
            None => {
                if rest.is_empty() {
                    None
                } else {
                    self.pos = self.input.len();
                    Some(rest)
                }
            }
        }
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("=== LIFETIME ANNOTATIONS ===\n");

    let s = String::from("hello world rust");
    println!("first_word: {}", first_word(&s));
    println!("longest: {}", longest("ab", "abc"));

    let ex = Excerpt::new("a piece of text");
    println!("excerpt: {}", ex.announce());

    println!("static: {}", static_str());

    let mut v: Vec<&str> = Vec::new();
    store(&mut v, "borrowed");
    println!("store: {:?}", v);

    let input = String::from("alpha beta gamma");
    let mut parser = Parser::new(&input);
    while let Some(tok) = parser.next_token() {
        println!("token: {}", tok);
    }

    println!("\n=== ALL LIFETIME EXERCISES COMPLETE ===");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_word() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("solo"), "solo");
    }

    #[test]
    fn test_longest() {
        assert_eq!(longest("ab", "abcd"), "abcd");
        assert_eq!(longest("abcd", "ab"), "abcd");
    }

    #[test]
    fn test_excerpt() {
        let e = Excerpt::new("part");
        assert_eq!(e.announce(), "part");
    }

    #[test]
    fn test_parser_tokens() {
        let input = String::from("a bb ccc");
        let mut p = Parser::new(&input);
        assert_eq!(p.next_token(), Some("a"));
        assert_eq!(p.next_token(), Some("bb"));
        assert_eq!(p.next_token(), Some("ccc"));
        assert_eq!(p.next_token(), None);
    }

    #[test]
    fn test_static_str() {
        assert_eq!(static_str(), "I live in the binary");
    }
}
