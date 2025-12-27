/// ============================================================================
/// ADVANCED PATTERNS: Builder, Typestate, and RAII Patterns
/// ============================================================================
///
/// These patterns represent production-quality Rust code patterns you'll see
/// in professional Rust projects.

// ============================================================================
// PATTERN 1: BUILDER PATTERN
// ============================================================================

/// Builder pattern for ergonomic object construction
#[derive(Debug)]
pub struct HttpRequest {
    method: String,
    url: String,
    headers: std::collections::HashMap<String, String>,
    body: Option<String>,
    timeout_secs: u64,
}

pub struct HttpRequestBuilder {
    method: String,
    url: String,
    headers: std::collections::HashMap<String, String>,
    body: Option<String>,
    timeout_secs: u64,
}

impl HttpRequestBuilder {
    /// Start building a request
    pub fn new(method: &str, url: &str) -> Self {
        HttpRequestBuilder {
            method: method.to_string(),
            url: url.to_string(),
            headers: std::collections::HashMap::new(),
            body: None,
            timeout_secs: 30,
        }
    }

    /// Add a header
    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self  // Return self for chaining
    }

    /// Set body
    pub fn body(mut self, body: &str) -> Self {
        self.body = Some(body.to_string());
        self
    }

    /// Set timeout
    pub fn timeout(mut self, secs: u64) -> Self {
        self.timeout_secs = secs;
        self
    }

    /// Build the request
    pub fn build(self) -> HttpRequest {
        HttpRequest {
            method: self.method,
            url: self.url,
            headers: self.headers,
            body: self.body,
            timeout_secs: self.timeout_secs,
        }
    }
}

pub fn pattern_1_builder() {
    println!("=== PATTERN 1: Builder ===");

    let request = HttpRequestBuilder::new("POST", "https://api.example.com/users")
        .header("Content-Type", "application/json")
        .header("Authorization", "Bearer token123")
        .body(r#"{"name": "Alice"}"#)
        .timeout(60)
        .build();

    println!("{:#?}", request);
}

// ============================================================================
// PATTERN 2: TYPESTATE PATTERN
// ============================================================================

/// Typestate pattern: encode state in the type system
/// This prevents invalid operations at compile-time

pub struct Connection<State> {
    socket: String,  // Pretend this is a socket
    _state: std::marker::PhantomData<State>,
}

pub struct Disconnected;
pub struct Connected;
pub struct Authenticated;

impl Connection<Disconnected> {
    pub fn new() -> Self {
        Connection {
            socket: "socket:8080".to_string(),
            _state: std::marker::PhantomData,
        }
    }

    /// Connect: Disconnected -> Connected
    pub fn connect(self) -> Connection<Connected> {
        println!("Connecting...");
        Connection {
            socket: self.socket,
            _state: std::marker::PhantomData,
        }
    }
}

impl Connection<Connected> {
    /// Authenticate: Connected -> Authenticated
    pub fn authenticate(self, password: &str) -> Connection<Authenticated> {
        println!("Authenticating with password...");
        Connection {
            socket: self.socket,
            _state: std::marker::PhantomData,
        }
    }
}

impl Connection<Authenticated> {
    /// Query: only available when authenticated
    pub fn query(&self, sql: &str) -> String {
        println!("Executing query: {}", sql);
        "Results".to_string()
    }

    /// Disconnect: Authenticated -> Disconnected
    pub fn disconnect(self) -> Connection<Disconnected> {
        println!("Disconnecting...");
        Connection {
            socket: self.socket,
            _state: std::marker::PhantomData,
        }
    }
}

pub fn pattern_2_typestate() {
    println!("\n=== PATTERN 2: Typestate ===");

    let conn = Connection::new();
    println!("Created disconnected connection");

    let conn = conn.connect();
    println!("Connected");

    let conn = conn.authenticate("secret");
    println!("Authenticated");

    let _results = conn.query("SELECT * FROM users");

    // This wouldn't compile - demonstrates type safety:
    // let conn2 = Connection::new();
    // conn2.query("SELECT *");  // ❌ ERROR: can't query on Disconnected

    let _disconnected = conn.disconnect();
    println!("Disconnected");
}

// ============================================================================
// PATTERN 3: RESOURCE POOL / OBJECT POOL
// ============================================================================

/// Object pool for reusing expensive resources
pub struct DatabaseConnectionPool {
    available: std::sync::Mutex<Vec<String>>,
    total_created: std::sync::atomic::AtomicUsize,
}

impl DatabaseConnectionPool {
    pub fn new(initial_size: usize) -> Self {
        let mut available = Vec::with_capacity(initial_size);
        for _ in 0..initial_size {
            available.push("connection".to_string());
        }

        DatabaseConnectionPool {
            available: std::sync::Mutex::new(available),
            total_created: std::sync::atomic::AtomicUsize::new(initial_size),
        }
    }

    pub fn acquire(&self) -> PooledConnection {
        let mut available = self.available.lock().unwrap();

        if let Some(conn) = available.pop() {
            PooledConnection {
                connection: Some(conn),
                pool: self,
            }
        } else {
            // Create new connection if none available
            self.total_created
                .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            PooledConnection {
                connection: Some("new_connection".to_string()),
                pool: self,
            }
        }
    }

    pub fn total_created(&self) -> usize {
        self.total_created.load(std::sync::atomic::Ordering::SeqCst)
    }
}

/// RAII guard: returns connection to pool on drop
pub struct PooledConnection<'a> {
    connection: Option<String>,
    pool: &'a DatabaseConnectionPool,
}

impl<'a> Drop for PooledConnection<'a> {
    fn drop(&mut self) {
        if let Some(conn) = self.connection.take() {
            let mut available = self.pool.available.lock().unwrap();
            available.push(conn);
        }
    }
}

pub fn pattern_3_object_pool() {
    println!("\n=== PATTERN 3: Object Pool ===");

    let pool = DatabaseConnectionPool::new(2);
    println!("Pool created with 2 connections");

    {
        let _conn1 = pool.acquire();
        println!("Acquired connection 1");

        let _conn2 = pool.acquire();
        println!("Acquired connection 2");

        let _conn3 = pool.acquire();
        println!("Acquired connection 3 (new)");

        println!("Total created: {}", pool.total_created());
        // Connections returned to pool when dropped
    }

    println!("Connections returned to pool");
}

// ============================================================================
// PATTERN 4: NEWTYPE PATTERN FOR STRONG TYPING
// ============================================================================

/// Newtype pattern: wrapper around primitive type for type safety
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserId(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PostId(u64);

impl UserId {
    pub fn new(id: u64) -> Self {
        UserId(id)
    }
}

impl PostId {
    pub fn new(id: u64) -> Self {
        PostId(id)
    }
}

pub fn pattern_4_newtype() {
    println!("\n=== PATTERN 4: Newtype ===");

    let user_id = UserId::new(123);
    let post_id = PostId::new(456);

    // These won't compile - type safety!
    // get_user(post_id);  // ❌ ERROR: expected UserId, found PostId

    println!("User: {:?}", user_id);
    println!("Post: {:?}", post_id);
    println!("✓ Type safety prevents mixing user and post IDs");
}

// ============================================================================
// PATTERN 5: EXTENSION TRAIT PATTERN
// ============================================================================

/// Extension trait: add methods to types without owning them
pub trait StringExt {
    fn shout(&self) -> String;
    fn is_question(&self) -> bool;
}

impl StringExt for &str {
    fn shout(&self) -> String {
        format!("{}!", self.to_uppercase())
    }

    fn is_question(&self) -> bool {
        self.ends_with('?')
    }
}

impl StringExt for String {
    fn shout(&self) -> String {
        format!("{}!", self.to_uppercase())
    }

    fn is_question(&self) -> bool {
        self.ends_with('?')
    }
}

pub fn pattern_5_extension_trait() {
    println!("\n=== PATTERN 5: Extension Trait ===");

    let greeting = "hello";
    println!("{}", greeting.shout());  // hello -> HELLO!

    let question = "are you ok?";
    println!("Is question: {}", question.is_question());

    let owned = String::from("goodbye");
    println!("{}", owned.shout());  // Works on owned String too
}

// ============================================================================
// PATTERN 6: INTERIOR MUTABILITY WITH SMART CACHING
// ============================================================================

use std::cell::RefCell;

pub struct CachedValue<T> {
    value: T,
    cache: RefCell<Option<String>>,
}

impl<T: std::fmt::Display> CachedValue<T> {
    pub fn new(value: T) -> Self {
        CachedValue {
            value,
            cache: RefCell::new(None),
        }
    }

    /// Get cached string representation (compute once, reuse)
    pub fn display(&self) -> std::cell::Ref<Option<String>> {
        let mut cache = self.cache.borrow_mut();
        if cache.is_none() {
            *cache = Some(self.value.to_string());
        }
        drop(cache);
        self.cache.borrow()
    }
}

pub fn pattern_6_interior_mutability() {
    println!("\n=== PATTERN 6: Interior Mutability Caching ===");

    let value = CachedValue::new(42);

    // First call computes
    println!("Display: {}", value.display().as_ref().unwrap());

    // Second call uses cache
    println!("Display: {}", value.display().as_ref().unwrap());
}

// ============================================================================
// PATTERN 7: ERROR PROPAGATION WITH CONTEXT
// ============================================================================

use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct AppError {
    message: String,
    context: Vec<String>,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)?;
        for ctx in &self.context {
            write!(f, "\n  -> {}", ctx)?;
        }
        Ok(())
    }
}

impl Error for AppError {}

impl AppError {
    pub fn new(msg: &str) -> Self {
        AppError {
            message: msg.to_string(),
            context: Vec::new(),
        }
    }

    pub fn context(mut self, ctx: &str) -> Self {
        self.context.push(ctx.to_string());
        self
    }
}

pub fn pattern_7_error_context() {
    println!("\n=== PATTERN 7: Error Context ===");

    let error = AppError::new("Database connection failed")
        .context("Tried connecting to 127.0.0.1:5432")
        .context("Called from UserService::get_user()");

    println!("Error:\n{}", error);
}

// ============================================================================
// PATTERN 8: TRAIT OBJECTS FOR POLYMORPHISM
// ============================================================================

pub trait Handler {
    fn handle(&self, message: &str);
    fn name(&self) -> &str;
}

pub struct ConsoleHandler;
pub struct FileHandler;

impl Handler for ConsoleHandler {
    fn handle(&self, message: &str) {
        println!("[CONSOLE] {}", message);
    }

    fn name(&self) -> &str {
        "Console"
    }
}

impl Handler for FileHandler {
    fn handle(&self, message: &str) {
        println!("[FILE] {}", message);
    }

    fn name(&self) -> &str {
        "File"
    }
}

pub fn pattern_8_trait_objects() {
    println!("\n=== PATTERN 8: Trait Objects ===");

    let handlers: Vec<Box<dyn Handler>> = vec![
        Box::new(ConsoleHandler),
        Box::new(FileHandler),
    ];

    for handler in handlers {
        handler.handle("System started");
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    pattern_1_builder();
    pattern_2_typestate();
    pattern_3_object_pool();
    pattern_4_newtype();
    pattern_5_extension_trait();
    pattern_6_interior_mutability();
    pattern_7_error_context();
    pattern_8_trait_objects();

    println!("\n=== ALL PATTERNS DEMONSTRATED ===");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder() {
        let req = HttpRequestBuilder::new("GET", "https://example.com")
            .header("User-Agent", "Test")
            .build();

        assert_eq!(req.method, "GET");
        assert_eq!(req.url, "https://example.com");
        assert_eq!(req.headers.get("User-Agent").unwrap(), "Test");
    }

    #[test]
    fn test_newtype() {
        let user = UserId::new(123);
        let post = PostId::new(456);

        assert_eq!(user, UserId(123));
        assert_eq!(post, PostId(456));
        assert_ne!(UserId(123), UserId(124));
    }

    #[test]
    fn test_string_ext() {
        assert_eq!("hello".shout(), "HELLO!");
        assert!(
"what?".is_question());
        assert!("statement".is_question() == false);
    }
}
