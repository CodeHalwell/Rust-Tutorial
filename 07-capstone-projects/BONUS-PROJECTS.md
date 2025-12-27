# Bonus Projects: Advanced Challenges

These projects integrate concepts from across the curriculum and provide additional practice with systems programming patterns.

---

## Bonus Project 1: Custom String Type

**Difficulty:** Intermediate
**Duration:** 3-4 days
**Concepts:** Ownership, Deref trait, custom Drop, indexing

### Goal

Create a `MyString` type that:
- Manages its own heap memory
- Implements Deref to act like &str
- Supports indexing via Index trait
- Custom Drop for cleanup
- Implements common string methods

### Learning Objectives

- Understand how String works internally
- Implement smart pointer traits (Deref)
- Implement custom indexing (Index trait)
- Memory layout and pointer arithmetic

### Key Traits to Implement

```rust
impl Deref for MyString {
    type Target = str;
    fn deref(&self) -> &str { ... }
}

impl Index<Range<usize>> for MyString {
    type Output = str;
    fn index(&self, range: Range<usize>) -> &str { ... }
}

impl Drop for MyString {
    fn drop(&mut self) { ... }
}
```

### Bonus Features

- Iterator over characters
- Clone implementation
- PartialEq for comparison
- Display trait for printing
- Unsafe pointer manipulation (advanced)

### Testing Strategy

```rust
#[test]
fn test_create_and_len() { }

#[test]
fn test_deref() { }

#[test]
fn test_indexing() { }

#[test]
fn test_drop_called() { }  // Use counter to verify
```

---

## Bonus Project 2: Custom Hashmap

**Difficulty:** Intermediate-Advanced
**Duration:** 4-5 days
**Concepts:** Hash functions, collision handling, generics, traits

### Goal

Implement a basic HashMap from scratch:
- Fixed-size hash table with linear probing
- Support insert, get, remove
- Generic over key and value types
- Hash and Eq trait bounds
- Load factor and resizing

### Learning Objectives

- Hash functions and collision handling
- Generic data structures
- Trait bounds for custom types
- Memory layout optimization

### Implementation Plan

1. **Phase 1:** Fixed-size table with linear probing
2. **Phase 2:** Generic K, V types with bounds
3. **Phase 3:** Resizing and load factor
4. **Phase 4:** Iterator support
5. **Phase 5:** Custom key types (struct with Hash/Eq)

### Key Methods

```rust
impl<K: Hash + Eq, V> HashMap<K, V> {
    pub fn new(capacity: usize) -> Self { }
    pub fn insert(&mut self, key: K, value: V) -> Option<V> { }
    pub fn get(&self, key: &K) -> Option<&V> { }
    pub fn remove(&mut self, key: &K) -> Option<V> { }
    pub fn iter(&self) -> Iter { }
}
```

### Collision Handling Visualization

```
Linear Probing:
    Insert(key1, val1) → hash(key1) % capacity = 3
    Insert(key2, val2) → hash(key2) % capacity = 3 (collision!)
                      → Try slot 4 (3+1) → insert here

Lookup:
    Get(key2) → hash(key2) % capacity = 3
             → Not there, check 4 → found!

Hash Table:
    [0] empty
    [1] empty
    [2] empty
    [3] (key1, val1)
    [4] (key2, val2)  ← Probed to here
    [5] empty
```

### Testing

- Insert and retrieve values
- Overwrite existing keys
- Delete keys
- Custom key types (struct with Hash/Eq impl)
- Load factor and resizing
- Iterator functionality

---

## Bonus Project 3: Thread-Safe Task Scheduler

**Difficulty:** Advanced
**Duration:** 5-7 days
**Concepts:** Concurrency, Mutex, Arc, channels, work-stealing

### Goal

Build a simple task scheduler:
- Multiple worker threads
- Task queue with thread safety
- FIFO or work-stealing scheduling
- Graceful shutdown
- Performance metrics

### Learning Objectives

- Multi-threaded programming patterns
- Mutex and Arc usage
- Channel-based communication
- Thread pool architecture
- Measuring concurrency

### Architecture

```
Main Thread:
    │
    ├─ Create TaskQueue (Mutex<Vec<Task>>)
    ├─ Spawn N worker threads
    └─ Submit tasks to queue

Worker Threads:
    │
    ├─ Loop:
    │   ├─ Lock queue (block if empty)
    │   ├─ Get task from queue
    │   ├─ Execute task
    │   ├─ Update metrics
    │   └─ Repeat

Shutdown:
    ├─ Broadcast shutdown signal
    ├─ Workers exit loop
    ├─ Wait for all threads to finish
    └─ Print metrics
```

### Implementation Plan

1. **Phase 1:** Simple task queue with mutex
2. **Phase 2:** Worker thread pool
3. **Phase 3:** Task submission and execution
4. **Phase 4:** Graceful shutdown
5. **Phase 5:** Metrics collection (work done, queue depth, etc.)

### Key Types

```rust
pub enum Task {
    Work(Box<dyn FnOnce() + Send>),
    Shutdown,
}

pub struct TaskQueue {
    queue: Mutex<Vec<Task>>,
    workers: Vec<thread::JoinHandle<()>>,
}

impl TaskQueue {
    pub fn new(num_workers: usize) -> Self { }
    pub fn submit<F>(&self, task: F) where F: FnOnce() + Send + 'static { }
    pub fn shutdown(self) { }
}
```

### Testing

- Create scheduler with multiple workers
- Submit tasks and verify execution
- Queue under load (many tasks)
- Shutdown and graceful termination
- Verify all tasks eventually execute

---

## Bonus Project 4: Simple JSON Parser

**Difficulty:** Intermediate
**Duration:** 3-4 days
**Concepts:** Enums, Pattern matching, recursive data structures, error handling

### Goal

Implement a JSON parser:
- Parse JSON into custom AST
- Support: objects, arrays, strings, numbers, booleans, null
- Pretty-print JSON
- Error reporting with line/column

### Learning Objectives

- Recursive data structures
- Pattern matching on enums
- Error handling and reporting
- Parsing techniques

### JSON AST

```rust
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

pub struct Parser {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Parser {
    pub fn parse(&mut self) -> Result<JsonValue, ParseError> { }
    fn parse_value(&mut self) -> Result<JsonValue, ParseError> { }
    fn parse_object(&mut self) -> Result<JsonValue, ParseError> { }
    fn parse_array(&mut self) -> Result<JsonValue, ParseError> { }
    fn parse_string(&mut self) -> Result<String, ParseError> { }
    fn parse_number(&mut self) -> Result<f64, ParseError> { }
}
```

### Example Usage

```rust
let json_str = r#"{"name": "Alice", "age": 30, "skills": ["Rust", "Python"]}"#;
let parser = Parser::new(json_str);
match parser.parse() {
    Ok(json) => println!("{:?}", json),
    Err(e) => println!("Parse error: {} at line {}", e.message, e.line),
}
```

### Testing

- Parse simple values (numbers, strings, booleans)
- Parse arrays and objects
- Nested structures
- Error cases (invalid syntax, unterminated strings)
- Round-trip: parse → pretty-print → parse

---

## Bonus Project 5: LRU Cache Benchmarking

**Difficulty:** Intermediate
**Duration:** 2-3 days
**Concepts:** Performance, generics, traits, criterion

### Goal

Implement multiple LRU cache designs and benchmark them:
1. LinkedList + HashMap (moves on access)
2. Vec + HashMap (removes and re-inserts)
3. Custom linked list with raw pointers (advanced)
4. Doubly-linked list with Rc/RefCell

Compare performance, memory usage, and code complexity.

### Learning Objectives

- Performance measurement with criterion
- Trade-offs in different data structures
- Generics and trait bounds
- Memory efficiency analysis

### LRU Cache Trait

```rust
pub trait LRUCache<K, V> {
    fn new(capacity: usize) -> Self;
    fn get(&mut self, key: &K) -> Option<V>;
    fn put(&mut self, key: K, value: V);
    fn stats(&self) -> CacheStats;
}

pub struct CacheStats {
    hits: usize,
    misses: usize,
    evictions: usize,
}
```

### Benchmark Plan

```rust
fn bench_get_hit_rate(c: &mut Criterion) {
    // 1M gets, 90% hit rate, measure throughput
}

fn bench_memory_usage() {
    // Compare heap allocations across implementations
}

fn bench_workload_zipfian() {
    // Realistic workload: few keys accessed frequently
}
```

### Insights to Explore

- Which design is fastest for reads vs writes?
- How do they scale with cache size?
- Zipfian distribution (realistic) vs uniform (benchmark)
- Memory overhead per entry

---

## Bonus Project 6: Minimal Web Framework

**Difficulty:** Advanced
**Duration:** 7-10 days
**Concepts:** Async, traits, macros, HTTP

### Goal

Build a minimal async web framework:
- Route matching (GET /api/users/:id)
- Middleware pipeline
- JSON serialization/deserialization
- Error handling and status codes
- Request/Response abstractions

### Learning Objectives

- Async/await patterns in production code
- Trait-based middleware design
- Macro DSL for routing
- End-to-end system design

### Architecture

```rust
pub struct App {
    routes: Vec<Route>,
    middleware: Vec<Box<dyn Middleware>>,
}

pub struct Route {
    pattern: String,  // "/api/users/:id"
    method: HttpMethod,
    handler: Box<dyn Fn(Request) -> Response + Send + Sync>,
}

pub trait Middleware {
    async fn process(&self, req: &mut Request, next: Next) -> Response;
}

// Usage:
let mut app = App::new();
app.get("/users/:id", handle_get_user);
app.post("/users", handle_create_user);
app.run("127.0.0.1:8080").await;
```

### Routing Macro DSL

```rust
app.routes!(
    GET "/api/users" => get_users,
    GET "/api/users/:id" => get_user,
    POST "/api/users" => create_user,
    PUT "/api/users/:id" => update_user,
    DELETE "/api/users/:id" => delete_user,
);
```

### Implementation Steps

1. HTTP types (Request, Response)
2. Route matching with parameter extraction
3. Basic middleware pipeline
4. Integrate with tokio HTTP (hyper)
5. JSON middleware (serialize/deserialize)
6. Error handling

---

## Bonus Project 7: Custom Allocator

**Difficulty:** Advanced
**Duration:** 5-7 days
**Concepts:** Unsafe code, memory layout, allocators

### Goal

Implement a custom memory allocator:
- Arena allocator (all allocations freed at once)
- Bump allocator (monotonic allocation)
- Custom Layout calculation

### Learning Objectives

- Unsafe Rust for memory management
- Allocator API
- Layout and alignment
- Memory fragmentation

### Key Trait

```rust
use std::alloc::GlobalAlloc;

pub struct CustomAllocator;

unsafe impl GlobalAlloc for CustomAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 { }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) { }
}

#[global_allocator]
static GLOBAL: CustomAllocator = CustomAllocator;
```

### Types to Implement

```rust
pub struct ArenaAllocator {
    buffer: Vec<u8>,
    offset: usize,
}

pub struct BumpAllocator {
    memory: *mut u8,
    capacity: usize,
    offset: usize,
}
```

---

## Challenge: Multi-File Project

Create a project spread across multiple modules:
- Separate concerns into files
- Use public/private visibility correctly
- Module hierarchies (mod, submod)
- Re-exports

Example structure:
```
src/
├── main.rs
├── lib.rs
├── cache/
│   ├── mod.rs
│   ├── lru.rs
│   └── stats.rs
├── server/
│   ├── mod.rs
│   ├── handler.rs
│   └── middleware.rs
└── utils/
    ├── mod.rs
    └── metrics.rs
```

---

## Completion Criteria

For each bonus project:

1. ✅ Compiles without warnings (`cargo clippy`)
2. ✅ Comprehensive test suite (>80% coverage)
3. ✅ Documentation comments on public API
4. ✅ Performance analyzed or benchmarked
5. ✅ Error handling throughout
6. ✅ Example or demo program
7. ✅ README explaining design

---

## Progression Path

**Recommended order by difficulty:**
1. Custom String Type
2. Simple JSON Parser
3. LRU Cache Benchmarking
4. Custom HashMap
5. Thread-Safe Task Scheduler
6. Custom Allocator
7. Minimal Web Framework

Each builds skills for the next. Start with any that interest you!
