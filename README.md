# Rust Zero-to-Hero: Systems Architecture Mastery

> *From fighting the borrow checker to writing systems that power databases, operating systems, and high-frequency trading platforms.*

A comprehensive, tier-based Rust curriculum designed to take developers from foundational understanding to **top 1% Rustacean** capable of building production systems. This is not a beginner's tutorial—this is deep systems thinking with Rust.

---

## Table of Contents

1. [Philosophy](#philosophy)
2. [Repository Structure](#repository-structure)
3. [The Curriculum Tiers](#the-curriculum-tiers)
4. [Mental Models & Deep Dives](#mental-models--deep-dives)
5. [Syllabus Overview](#syllabus-overview)
6. [Capstone Projects](#capstone-projects)
7. [Key Topics by Module](#key-topics-by-module)
8. [Performance & Benchmarking](#performance--benchmarking)
9. [How to Use This Repository](#how-to-use-this-repository)

---

## Philosophy

Rust's complexity isn't accidental—it's a feature. The borrow checker, ownership system, and type system are not barriers; they're guardrails that force you to think deeply about:

- **Memory safety**: Every allocation, every reference, every lifetime
- **Concurrency**: Race conditions become compile-time errors
- **Performance**: Zero-cost abstractions with no hidden allocations
- **Systems thinking**: How does this code run on actual hardware?

This curriculum teaches **why** before **how**, building mental models that let you reason about Rust code at the systems level.

---

## Repository Structure

```
rust-zero-to-hero-systems/
├── README.md                                # You are here
├── 01-syntax-and-ownership/                # Foundation: Ownership, Borrowing, Lifetimes
│   ├── notes/
│   │   ├── 01-ownership-model.md           # The core ownership semantics
│   │   ├── 02-borrowing-deep-dive.md       # Mutable vs. immutable borrows
│   │   ├── 03-lifetimes-explained.md       # Lifetime elision & variance
│   │   └── 04-move-semantics.md            # Move vs Copy semantics
│   └── exercises/
│       ├── 01-ownership-puzzle.rs
│       ├── 02-borrow-checker-challenges.rs
│       └── 03-lifetime-annotations.rs
│
├── 02-traits-and-generics/                 # Abstraction: Trait Bounds, Generics, Polymorphism
│   ├── notes/
│   │   ├── 01-trait-fundamentals.md
│   │   ├── 02-generics-and-monomorphization.md
│   │   ├── 03-associated-types.md
│   │   ├── 04-higher-ranked-trait-bounds.md
│   │   └── 05-variance-and-type-safety.md
│   └── exercises/
│       ├── 01-trait-implementations.rs
│       ├── 02-generic-constraints.rs
│       └── 03-advanced-trait-patterns.rs
│
├── 03-async-ecosystem/                     # Concurrency: Futures, Async/Await, Tokio
│   ├── notes/
│   │   ├── 01-future-trait.md              # What is a Future?
│   │   ├── 02-pin-and-unpin.md             # Why Pin exists
│   │   ├── 03-waker-mechanism.md           # How async wakeup works
│   │   ├── 04-state-machines.md            # Async/await desugaring
│   │   └── 05-tokio-internals.md           # Tokio runtime architecture
│   └── exercises/
│       ├── 01-custom-future.rs
│       ├── 02-manual-state-machine.rs
│       └── 03-tokio-patterns.rs
│
├── 04-unsafe-and-ffi/                      # Low-level: Unsafe, Raw Pointers, FFI
│   ├── notes/
│   │   ├── 01-unsafe-guarantees.md         # What safe Rust doesn't check
│   │   ├── 02-raw-pointers.md              # *const T and *mut T
│   │   ├── 03-undefined-behavior.md        # UB categories & gotchas
│   │   ├── 04-ffi-basics.md                # Calling C/C++ from Rust
│   │   └── 05-memory-layout.md             # repr(C), padding, alignment
│   └── exercises/
│       ├── 01-unsafe-operations.rs
│       ├── 02-raw-pointer-manipulation.rs
│       └── 03-ffi-bindings.rs
│
├── 05-meta-programming/                    # Macros: Declarative & Procedural
│   ├── notes/
│   │   ├── 01-macro-rules-fundamentals.md
│   │   ├── 02-declarative-macro-patterns.md
│   │   ├── 03-procedural-macros-intro.md
│   │   ├── 04-syn-and-quote.md
│   │   └── 05-derive-macros.md
│   └── exercises/
│       ├── 01-declarative-macros.rs
│       ├── 02-procedural-macro-crate/
│       └── 03-custom-derive.rs
│
├── 06-performance/                         # Systems: Zero-cost, SIMD, Memory, Profiling
│   ├── notes/
│   │   ├── 01-zero-cost-abstractions.md
│   │   ├── 02-memory-layout-optimization.md
│   │   ├── 03-simd-and-vectorization.md
│   │   ├── 04-profiling-tools.md
│   │   └── 05-benchmarking-with-criterion.md
│   ├── exercises/
│   │   └── 01-memory-layout-analysis.rs
│   └── benchmarks/
│       ├── vector-iteration.rs
│       └── allocation-patterns.rs
│
├── 07-capstone-projects/                   # Capstone: Build Real Systems
│   ├── project-1-grep/
│   │   ├── README.md                       # Grep clone specifications
│   │   ├── src/
│   │   └── tests/
│   ├── project-2-http-server/
│   │   ├── README.md                       # HTTP server specifications
│   │   ├── src/
│   │   └── tests/
│   └── project-3-database/
│       ├── README.md                       # In-memory database specs
│       ├── src/
│       └── tests/
│
├── examples/                               # Intermediate Examples
│   ├── 01-interior-mutability.rs           # RefCell, Rc, Mutex, Arc patterns
│   ├── 02-drop-implementation.rs
│   ├── 03-custom-iterators.rs
│   └── 04-error-handling-deep-dive.rs
│
└── docs/                                   # Additional Resources
    ├── mental-models.md
    ├── glossary.md
    └── resources.md
```

---

## The Curriculum Tiers

### **Tier 1: Foundation (01-syntax-and-ownership)**

**Goal**: Build unshakeable understanding of Rust's core value proposition.

**Duration**: 2-3 weeks (40-50 hours)

**Key Outcomes**:
- Deep understanding of ownership semantics (Move vs Copy)
- Mastery of borrowing rules (mutable & immutable)
- Lifetime annotations and elision rules
- Stack vs heap allocation patterns

**Why This Matters**:
The borrow checker isn't a limitation—it's a proof system. Every compile error is the Rust compiler telling you about a real bug. Once you understand *why* the rules exist, you stop fighting them.

**Mental Model**:
- Every value has exactly one owner
- References are temporary loans with compile-time guarantees
- Lifetimes encode *relationships* between references
- The compiler's job is proving your code is memory-safe

---

### **Tier 2: Abstraction (02-traits-and-generics)**

**Goal**: Master Rust's type system for building composable, generic code.

**Duration**: 2 weeks (30-40 hours)

**Key Outcomes**:
- Trait-based design and composition
- Generic type parameters with constraints
- Associated types and where clauses
- Higher-ranked trait bounds (HRTBs)
- Understanding monomorphization

**Why This Matters**:
Traits are how Rust achieves abstraction without runtime overhead. Generic code is compiled per concrete type (monomorphization), meaning you get polymorphism with zero runtime cost.

**Mental Model**:
- Traits define capability contracts
- Generics are compile-time code specialization
- The type system is your runtime optimizer
- Compile-time errors prevent runtime surprises

---

### **Tier 3: Concurrency (03-async-ecosystem)**

**Goal**: Understand async Rust deeply—not just `await`, but the machinery underneath.

**Duration**: 3 weeks (40-50 hours)

**Key Outcomes**:
- What a `Future` is and how it works
- `Pin` and `Unpin` and why they matter
- `Waker` mechanism for async wakeup
- How `async`/`await` desugars to state machines
- Tokio runtime architecture
- Async patterns and pitfalls

**Why This Matters**:
Async Rust seems magical until you understand Futures are just enums. `Pin` exists to prevent memory unsafety in self-referential structs. Once you can read desugared async code, you understand concurrency at a systems level.

**Mental Model**:
- Futures are lazy state machines
- Async/await is syntactic sugar for state transitions
- Waker allows the runtime to efficiently schedule work
- Zero-copy concurrency with no garbage collector

---

### **Tier 4: Low-Level Systems (04-unsafe-and-ffi)**

**Goal**: Understand when and how to use unsafe Rust, and interact with C/C++.

**Duration**: 2-3 weeks (30-40 hours)

**Key Outcomes**:
- When unsafe is necessary and justified
- Raw pointers (*const T, *mut T)
- Undefined Behavior categories and detection
- FFI with C/C++ libraries
- Memory layout (repr(C), padding, alignment)
- Writing safe abstractions over unsafe code

**Why This Matters**:
The best Rust code minimizes unsafe, but systems programming sometimes requires it. Understanding what's safe vs unsafe lets you write correct low-level code. FFI is how Rust integrates with existing ecosystems.

**Mental Model**:
- Safe code has compile-time guarantees; unsafe code is your responsibility
- Raw pointers are powerful but dangerous
- UB isn't a crash—it's "anything can happen"
- Safe abstractions should be encapsulated—users never see unsafe

---

### **Tier 5: Metaprogramming (05-meta-programming)**

**Goal**: Build code that writes code. Understand declarative and procedural macros.

**Duration**: 2 weeks (25-35 hours)

**Key Outcomes**:
- Declarative macros (macro_rules!) and pattern matching
- Procedural macros (derive, attribute, function-like)
- The `syn` and `quote` crates for macro development
- Common macro pitfalls and hygiene
- Using macros to implement compile-time optimizations

**Why This Matters**:
Macros are Rust's escape hatch for things the type system can't express. They're also used to generate boilerplate, implement type-level programming, and create DSLs. Understanding macro expansion helps you debug complex generated code.

**Mental Model**:
- Macros operate on token streams
- Declarative macros are pattern matching on syntax
- Procedural macros are functions that transform code
- Macro hygiene prevents accidental identifier capture

---

### **Tier 6: Performance & Systems (06-performance)**

**Goal**: Write high-performance Rust with deep understanding of hardware interactions.

**Duration**: 3 weeks (40-50 hours)

**Key Outcomes**:
- Zero-cost abstractions and why they work
- Memory layout optimization (padding, alignment, cache locality)
- SIMD vectorization and packed_simd
- Profiling tools (perf, flamegraph, cargo-flamegraph)
- Benchmarking methodology with criterion
- Identifying and eliminating allocations

**Why This Matters**:
Performance isn't an afterthought—it's a first-class citizen in systems programming. Rust's abstractions compile to optimal machine code. Understanding memory layout and cache behavior is what separates good systems code from great code.

**Mental Model**:
- Every abstraction should have zero runtime cost
- Allocations are visible in your code—know where they happen
- Cache locality matters more than algorithmic complexity (sometimes)
- Profiling beats guessing

---

### **Tier 7: Capstone Projects (07-capstone-projects)**

**Goal**: Build production-quality systems that integrate all previous learning.

**Duration**: 4-6 weeks (60-80 hours)

**Key Outcomes**:
- Complete implementations of complex systems
- Testing, error handling, and code organization
- Performance optimization in real code
- Documentation and API design

---

## Mental Models & Deep Dives

### **1. Ownership: The Core Value Proposition**

**The Question**: Why does Rust require explicit ownership?

**The Answer**: Memory safety without garbage collection. Every byte of memory is accounted for at compile-time.

**Key Insights**:
- Ownership is not optional; it's mandatory
- Move semantics are the default for non-Copy types
- Copy types (u32, f64, bool) are exceptions: they're bitwise-copyable and cheap
- Drop trait executes automatically when ownership ends

**Deep Model**:
```rust
// MOVE SEMANTICS
let s1 = String::from("hello");  // s1 owns the String
let s2 = s1;                      // Ownership MOVES from s1 to s2
// println!("{}", s1);            // ❌ COMPILE ERROR: s1 no longer owns data

// COPY SEMANTICS
let x = 5;                        // x owns an i32
let y = x;                        // x is COPIED to y, x still valid
println!("{}", x);                // ✅ WORKS: i32 implements Copy
```

### **2. Borrowing: Temporary Loans with Guarantees**

**The Question**: How can multiple parts of code read the same data safely?

**The Answer**: Borrowing rules enforced at compile time.

**Key Rules**:
1. Either one mutable reference **or** many immutable references
2. References cannot outlive what they reference
3. You cannot mutate through an immutable reference

**Why It Matters**:
- No data races (checked at compile time, not runtime)
- No iterator invalidation bugs
- No use-after-free vulnerabilities

---

### **3. Lifetimes: Encoding Relationships**

**The Question**: How does the compiler know a reference is valid?

**The Answer**: Lifetimes explicitly encode how long references live.

**Mental Model**:
```rust
// This function says: the returned string slice lives as long as
// the input reference lives. If either input dies before we use the result,
// the compiler rejects it.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// The compiler INFERS:
// fn first_word<'a>(s: &'a str) -> &'a str
```

---

### **4. Variance: Type Safety in Generic Code**

**The Question**: If Cat is a subtype of Animal, is Vec\<Cat\> a subtype of Vec\<Animal\>?

**The Answer**: It depends—on variance.

**Key Concept**:
- **Covariance**: Subtype relationships are preserved (e.g., reading)
- **Contravariance**: Subtype relationships are reversed (e.g., functions)
- **Invariance**: No subtype relationships (e.g., mutable references)

This is why you can't pass `&mut Vec<Cat>` to a function expecting `&mut Vec<Animal>`—it's not type-safe.

---

### **5. Interior Mutability: Mutating the Immutable**

**The Question**: Sometimes you need to mutate data even though you only have an immutable reference. How?

**The Answer**: Interior mutability patterns (RefCell, Cell, Mutex, RwLock).

**Key Pattern**:
```rust
// Single-threaded: RefCell
let data = RefCell::new(vec![1, 2, 3]);
data.borrow_mut().push(4);  // Mutate through immutable ref!

// Thread-safe: Mutex
let data = Mutex::new(vec![1, 2, 3]);
data.lock().unwrap().push(4);  // Mutate from multiple threads safely
```

See `examples/01-interior-mutability.rs` for detailed analysis.

---

### **6. Async/Await: Lazy State Machines**

**The Question**: How does async/await work without threads or garbage collection?

**The Answer**: The compiler transforms async code into state machines, Futures are enums, and Waker enables efficient scheduling.

**Key Insight**:
```rust
// This async function:
async fn fetch_data() -> String {
    let response = client.get("/api").await;
    parse_response(response).await
}

// Desugars to something like:
enum FetchDataFuture {
    GetRequest,
    Parsing,
    Done(String),
}
```

The runtime polls Futures, moving them through states. Waker tells the runtime *when* to poll again, avoiding busy-waiting.

---

## Syllabus Overview

### **Module 1: Syntax and Ownership (Weeks 1-3)**

| Week | Focus | Topics |
|------|-------|--------|
| 1 | Fundamentals | Variables, types, functions, control flow |
| 2 | Ownership | Move semantics, Copy vs Move, Drop trait |
| 2-3 | Borrowing | Immutable refs, mutable refs, aliasing rules |
| 3 | Lifetimes | Lifetime syntax, elision, lifetime bounds |

**Key Exercises**:
- Implement a generic Stack with explicit lifetime management
- Fix compiler errors to understand borrow checker
- Design a doubly-linked list (why is this hard?)

---

### **Module 2: Traits and Generics (Week 4-5)**

| Week | Focus | Topics |
|------|-------|--------|
| 4 | Traits | Trait definitions, impl blocks, trait objects |
| 4-5 | Generics | Type parameters, monomorphization, where clauses |
| 5 | Advanced | Associated types, HRTBs, variance |

**Key Exercises**:
- Implement Iterator trait for custom types
- Create a generic binary search function with trait bounds
- Build a type-safe units system using associated types

---

### **Module 3: Async Rust (Weeks 6-8)**

| Week | Focus | Topics |
|------|-------|--------|
| 6 | Futures | Future trait, executors, async/await syntax |
| 7 | Pin & Waker | Why Pin is necessary, Waker mechanism |
| 7-8 | Tokio | Tokio runtime, multi-task scheduling, Mutex for async |

**Key Exercises**:
- Implement a custom Future
- Write async code without Tokio
- Debug async code and understand cancellation

---

### **Module 4: Unsafe & FFI (Weeks 9-11)**

| Week | Focus | Topics |
|------|-------|--------|
| 9 | Unsafe | What safe Rust checks, unsafe blocks, contracts |
| 10 | Pointers | Raw pointers, dereferencing, pointer arithmetic |
| 10-11 | FFI & UB | C interop, UB categories, detecting UB with Miri |

**Key Exercises**:
- Call C functions from Rust
- Implement a thread-safe linked list with raw pointers
- Identify UB in unsafe code samples

---

### **Module 5: Metaprogramming (Weeks 12-13)**

| Week | Focus | Topics |
|------|-------|--------|
| 12 | Declarative | macro_rules!, repetition, metavariables |
| 12-13 | Procedural | Derive, attribute, function-like macros |
| 13 | Advanced | syn, quote, macro debugging |

**Key Exercises**:
- Implement a custom vec! macro
- Write a procedural derive macro
- Build a DSL using macros

---

### **Module 6: Performance & Systems (Weeks 14-16)**

| Week | Focus | Topics |
|------|-------|--------|
| 14 | Design | Zero-cost abstractions, benchmark methodology |
| 15 | Memory | Layout, padding, cache locality, allocator patterns |
| 15-16 | Tools | Profiling, SIMD, optimization techniques |

**Key Exercises**:
- Analyze memory layout of structs
- Optimize hot paths with profiling data
- Vectorize algorithms with SIMD

---

### **Module 7: Capstone Projects (Weeks 17-22)**

Build three production systems, each integrating previous modules.

---

## Capstone Projects

### **Project 1: Grep Clone (Intermediate)**

**Learning Goals**:
- File I/O and error handling
- String processing and regex
- Command-line argument parsing
- Testing and documentation

**Specification**:
- Match patterns in files (literal and regex)
- Support flags: `-i` (case-insensitive), `-n` (line numbers), `-v` (invert match)
- Handle multiple files and stdin
- Report results in standard format

**Why This Project**:
- Real-world tool with clear spec
- Forces you to design good error handling
- Teaches file I/O and performance considerations

---

### **Project 2: Multi-threaded HTTP Server (Intermediate-Advanced)**

**Learning Goals**:
- Network programming (TcpStream, TcpListener)
- Concurrency without frameworks
- HTTP protocol understanding
- Thread pooling and work queues

**Specification**:
- Serve static files over HTTP/1.0
- Handle GET requests with proper response codes
- Implement a thread pool for concurrent requests
- Support basic routing

**Why This Project**:
- Core networking and concurrency
- Understand how frameworks abstract low-level code
- Performance matters: measure response times

**Architecture Hint**:
```
TcpListener accepts connections
    ↓
ThreadPool distributes work
    ↓
Workers process HTTP requests
    ↓
Send responses, close connections
```

---

### **Project 3: In-Memory Database Engine (Advanced)**

**Learning Goals**:
- Complex data structures and algorithms
- Serialization and deserialization
- Performance optimization
- Testing at scale
- Unsafe code for custom memory management

**Specification**:
- B-tree or Hash-based key-value store
- Custom serialization format
- LRU cache layer
- Persistent append-only log
- Concurrent read/write with Mutex/RwLock
- Transactions with rollback

**Advanced Features**:
- Range queries and indexed lookups
- Compression (gzip or snappy)
- Incremental backup/restore
- Memory-mapped files

**Why This Project**:
- Integrates Traits, Generics, Async, Unsafe, and Performance
- Real-world systems architecture
- Teaches you how databases actually work
- Benchmark and optimize critical paths

---

## Key Topics by Module

### **Module 1: Syntax and Ownership**

**Core Concepts**:
- Variables and mutability
- Ownership model (move semantics)
- Borrowing and references
- Lifetimes and lifetime parameters
- Stack vs heap allocation
- Drop trait and RAII

**Real Understanding**:
- Why Rust needs explicit lifetime syntax when other languages don't
- How move semantics prevent data races
- Why you can't have multiple mutable references (it's not a limitation, it's a guarantee)

---

### **Module 2: Traits and Generics**

**Core Concepts**:
- Trait definitions and implementations
- Generic type parameters
- Trait bounds and where clauses
- Associated types
- Higher-ranked trait bounds (HRTB)
- Monomorphization and code bloat

**Real Understanding**:
- Traits are how you express capabilities, not inheritance
- Generics are a zero-cost abstraction (code is specialized per type)
- Associated types encode relationships between types
- The difference between `fn foo<F: Fn(i32) -> i32>` and `fn foo<F: for<'a> Fn(&'a str) -> bool>`

---

### **Module 3: Async Ecosystem**

**Core Concepts**:
- Future trait and Poll
- async/await syntax
- Pin and Unpin
- Waker and task scheduling
- Tokio runtime architecture
- Async patterns (channels, select, timeout)

**Real Understanding**:
- Futures are just enums that implement a poll interface
- async/await is syntactic sugar for state machines
- Pin prevents self-referential structs from moving
- Waker is how the executor knows when to poll a Future again
- Tokio's work-stealing scheduler balances load across threads

---

### **Module 4: Unsafe & FFI**

**Core Concepts**:
- Unsafe keyword and unsafe blocks
- Raw pointers (*const T, *mut T)
- Undefined behavior categories
- FFI with C and C++
- Memory layout (repr, padding, alignment)
- Common UB pitfalls

**Real Understanding**:
- Unsafe code has no compile-time guarantees; *you* guarantee correctness
- UB is not a segfault—it's "the compiler can assume this never happens"
- FFI requires careful type mapping and safety boundaries
- Safe abstractions hide unsafe code: users should never interact with it directly
- Miri can catch many UB bugs by interpreting code in a controlled environment

---

### **Module 5: Metaprogramming**

**Core Concepts**:
- Declarative macros (macro_rules!)
- Macro repetition and metavariables
- Procedural macros (derive, attribute, function-like)
- The syn and quote crates
- Macro hygiene and scoping
- Common patterns and pitfalls

**Real Understanding**:
- Macros operate on token streams, not abstract syntax trees
- Declarative macros are pattern matching + expansion
- Procedural macros are Rust functions that transform code
- Macro debugging requires understanding token expansion
- Procedural macros enable compile-time optimizations

---

### **Module 6: Performance & Systems**

**Core Concepts**:
- Zero-cost abstractions
- Memory layout optimization
- Struct padding and alignment
- SIMD and vectorization
- Profiling tools (perf, flamegraph)
- Criterion benchmarking
- Allocation patterns

**Real Understanding**:
- Cache locality is often more important than algorithmic complexity
- Memory layout directly impacts performance
- Inlining and optimization happen at the LLVM level
- Benchmarks must be carefully designed to avoid optimization
- Profiling reveals unexpected bottlenecks

---

## Performance & Benchmarking

### Benchmarking Methodology

1. **Establish Baseline**: Measure before optimizing
2. **Profile First**: Don't optimize blind
3. **Measure Atomically**: One change per measurement
4. **Use Criterion**: Statistical rigor in benchmarks
5. **Check Assembly**: Verify your code compiles optimally

### Common Performance Pitfalls

- **Allocations in hot paths**: Every allocation is visible
- **Cloning instead of borrowing**: Can kill performance
- **Monomorphization bloat**: Generic code can increase binary size
- **False sharing**: Multiple threads writing to same cache line
- **Contention on Mutex**: Use RwLock for mostly-reads, or lock-free structures

---

## How to Use This Repository

### **Recommended Path for Beginners**

1. Start with **Module 1** (Syntax and Ownership)
   - Read the notes
   - Complete exercises
   - Build confidence with the borrow checker

2. Continue with **Module 2** (Traits and Generics)
   - Practice trait design
   - Understand generic specialization

3. Move to **Project 1** (Grep Clone)
   - Apply modules 1-2 to a real system
   - Focus on clean error handling

### **Recommended Path for Intermediate Developers**

1. Skim **Module 1-2** (or self-assess)
2. Deep-dive **Module 3** (Async)
3. Study **Module 4** (Unsafe & FFI)
4. Complete **Project 2** (HTTP Server)

### **Recommended Path for Advanced Developers**

1. Self-assess with project challenges
2. Focus on weak areas (likely Async or Unsafe)
3. Tackle **Project 3** (Database Engine)
4. Build your own project: apply everything in a novel system

### **Learning Tips**

- **Read the notes first**: They explain *why*, not just *what*
- **Compile and run code**: Reading is passive; coding is active
- **Fix compiler errors manually**: Don't skip to solutions
- **Benchmark your code**: Intuition about performance is often wrong
- **Teach others**: Explain concepts to peers to find gaps in understanding
- **Build projects**: Theory without practice is incomplete

### **Time Estimates**

- Full curriculum: 22 weeks (20 hours/week = ~440 hours)
- Core modules (1-6): 16 weeks (~320 hours)
- With capstone projects: 22 weeks

Adjust based on prior Rust experience.

---

## Resources

### Official Documentation
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rustlings](https://github.com/rust-lang/rustlings)
- [The Rustonomicon (Unsafe & Advanced)](https://doc.rust-lang.org/nomicon/)

### Deep Dives
- [The Async Rust Book](https://rust-lang.github.io/async-book/)
- [Proc Macro Workshop](https://github.com/dtolnay/proc-macro-workshop)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)

### Tools
- [Miri](https://github.com/rust-lang/miri): UB detector
- [Clippy](https://github.com/rust-lang/rust-clippy): Lints
- [Cargo Flamegraph](https://github.com/flamegraph-rs/flamegraph): Profiling

---

## Contributing

This is an educational repository. Found an error? Want to improve explanations? Open a pull request or issue.

---

## License

This repository is provided as educational material. Code examples are MIT licensed.

---

**Start at Module 1. Build systems thinking layer by layer. Become a top 1% Rustacean.**