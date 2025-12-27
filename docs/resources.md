# Comprehensive Rust Learning Resources

## Getting Started

### Official Rust Documentation
- **The Book:** https://doc.rust-lang.org/book/
  - Best place to start if new to Rust
  - Covers fundamentals thoroughly
  - 15-20 hours to read

- **Rust by Example:** https://doc.rust-lang.org/rust-by-example/
  - Executable examples for every concept
  - Good for reference
  - 5-10 hours

- **The Rustonomicon:** https://doc.rust-lang.org/nomicon/
  - Advanced unsafe Rust
  - Memory layout
  - For systems programmers

### Interactive Learning
- **Rustlings:** https://github.com/rust-lang/rustlings
  - Small exercises to practice
  - 2-3 hours

- **Rust Playground:** https://play.rust-lang.org/
  - No installation needed
  - Instant feedback

- **Interactive Rust Tutorial:** https://www.rust-lang.org/what/wasm/
  - WebAssembly in browser

---

## This Curriculum

### Module 1: Syntax and Ownership
**Time:** 2-3 weeks | **Difficulty:** Beginner

**Files:**
- `01-syntax-and-ownership/notes/01-ownership-model.md` - Core concept
- `01-syntax-and-ownership/notes/02-borrowing-deep-dive.md` - References
- `01-syntax-and-ownership/notes/03-lifetimes-explained.md` - Lifetimes
- `01-syntax-and-ownership/exercises/01-ownership-exercises.rs` - Hands-on

**Key Concepts:**
- Move semantics
- Borrowing rules
- Lifetime annotations
- Drop trait

**Learning Path:**
1. Read ownership model (2 hours)
2. Do ownership exercises (3 hours)
3. Read borrowing (2 hours)
4. Study lifetimes (2 hours)
5. Review and practice (2 hours)

---

### Module 2: Traits and Generics
**Time:** 2 weeks | **Difficulty:** Intermediate

**Files:**
- `02-traits-and-generics/notes/01-trait-fundamentals.md` - Trait basics
- `02-traits-and-generics/notes/02-generics-and-monomorphization.md` - Generic specialization
- `02-traits-and-generics/exercises/01-trait-implementations.rs` - 11 practical exercises

**Key Concepts:**
- Trait definitions and implementations
- Generic type parameters
- Associated types
- Trait objects
- Monomorphization

**Learning Path:**
1. Read trait fundamentals (2 hours)
2. Study generics and monomorphization (2 hours)
3. Complete exercises 1-5 (3 hours)
4. Complete exercises 6-11 (3 hours)
5. Challenge: Build custom collection type (4 hours)

---

### Module 3: Async Ecosystem
**Time:** 3 weeks | **Difficulty:** Advanced

**Files:**
- `03-async-ecosystem/notes/01-future-trait.md` - Futures explained
- `03-async-ecosystem/notes/02-pin-and-unpin.md` - Memory safety for async
- `examples/04-error-handling-deep-dive.rs` - Error patterns (relevant for async)

**Key Concepts:**
- Future trait
- Async/await desugaring
- Pin and Unpin
- Waker mechanism
- Tokio runtime

**Learning Path:**
1. Read Future trait (3 hours)
2. Read Pin and Unpin (2 hours)
3. Study mental model (1 hour)
4. Build simple executor (4 hours)
5. Tokio tutorials (6 hours)

**Tokio Resources:**
- Official tutorial: https://tokio.rs/tokio/tutorial
- Async book: https://rust-lang.github.io/async-book/
- Tokio docs: https://docs.rs/tokio/

---

### Module 4: Unsafe and FFI
**Time:** 2-3 weeks | **Difficulty:** Advanced

**Files:**
- `04-unsafe-and-ffi/notes/01-unsafe-guarantees.md` - Unsafe Rust patterns
- `docs/glossary.md` - UB, FFI terms defined

**Key Concepts:**
- Unsafe blocks
- Raw pointers
- Undefined behavior categories
- FFI with C/C++
- Memory layout

**Learning Path:**
1. Read unsafe guarantees (3 hours)
2. Study memory layout (2 hours)
3. FFI basics (2 hours)
4. Implement safe wrapper (4 hours)
5. Advanced: custom allocator (8 hours)

**FFI Resources:**
- Rustonomicon FFI section: https://doc.rust-lang.org/nomicon/ffi.html
- Binding Generator: https://github.com/rust-lang/rust-bindgen

---

### Module 5: Metaprogramming
**Time:** 2 weeks | **Difficulty:** Advanced

**Key Concepts:**
- Declarative macros (macro_rules!)
- Procedural macros (derive, attribute, function-like)
- Syn and Quote crates
- Code generation

**Resources:**
- Macro rules: https://veykril.github.io/tlborm/
- Proc macro workshop: https://github.com/dtolnay/proc-macro-workshop
- Syn docs: https://docs.rs/syn/
- Quote docs: https://docs.rs/quote/

**Learning Path:**
1. Declarative macro basics (2 hours)
2. Advanced macro patterns (3 hours)
3. Procedural macro intro (3 hours)
4. Syn and quote (4 hours)
5. Build custom derive (6 hours)

---

### Module 6: Performance and Systems
**Time:** 3 weeks | **Difficulty:** Intermediate-Advanced

**Files:**
- `06-performance/notes/01-zero-cost-abstractions.md` - Performance fundamentals
- `docs/testing-and-benchmarking.md` - Measurement tools

**Key Concepts:**
- Zero-cost abstractions
- Memory layout optimization
- Profiling and benchmarking
- SIMD and vectorization
- Allocation patterns

**Learning Path:**
1. Zero-cost abstractions (2 hours)
2. Memory layout (2 hours)
3. Learn criterion benchmarking (2 hours)
4. Profiling tools (2 hours)
5. Optimize real code (8 hours)

**Performance Tools:**
- Criterion: https://bheisler.github.io/criterion.rs/book/
- Perf: https://perf.wiki.kernel.org/
- Flamegraph: https://github.com/flamegraph-rs/flamegraph

---

## Capstone Projects

### Project 1: Grep Clone
**Difficulty:** Intermediate | **Time:** 3-4 days
- File system basics
- Command-line parsing
- Regex integration
- Error handling

**File:** `07-capstone-projects/project-1-grep/README.md`

### Project 2: HTTP Server
**Difficulty:** Intermediate-Advanced | **Time:** 4-5 days
- Network programming
- Threading and concurrency
- HTTP protocol
- State management

**File:** `07-capstone-projects/project-2-http-server/README.md`

### Project 3: Database Engine
**Difficulty:** Advanced | **Time:** 5-7 days
- Data structures (B-trees, hash tables)
- Serialization
- Persistence
- Transactions
- Concurrency

**File:** `07-capstone-projects/project-3-database/README.md`

---

## Bonus Projects

### 7 Advanced Projects
- Custom String Type
- Custom HashMap
- Thread-Safe Task Scheduler
- JSON Parser
- LRU Cache Benchmarking
- Minimal Web Framework
- Custom Allocator

**File:** `07-capstone-projects/BONUS-PROJECTS.md`

---

## Reference Materials

### Glossary
**File:** `docs/glossary.md`
- 50+ terms defined
- Useful when you encounter unfamiliar concepts

### Mental Models
**File:** `docs/mental-models.md`
- Visual explanations
- Analogies and metaphors
- Deep understanding

### Testing and Benchmarking
**File:** `docs/testing-and-benchmarking.md`
- Unit tests, integration tests
- Property testing
- Benchmarking with Criterion
- Performance profiling

---

## Community and Support

### Official Resources
- **Rust Forum:** https://users.rust-lang.org/
- **Reddit:** https://www.reddit.com/r/rust/
- **Discord:** https://discord.gg/rust-lang
- **Stack Overflow:** Tag: [rust]

### Code Search
- **Docs.rs:** https://docs.rs/ (Crate documentation)
- **Lib.rs:** https://lib.rs/ (Crate discovery)
- **GitHub:** Search for Rust projects

### Blogs and Articles
- **Rust Blog:** https://blog.rust-lang.org/
- **This Week in Rust:** https://this-week-in-rust.org/
- **Rust By Example:** https://doc.rust-lang.org/rust-by-example/

---

## Tools You'll Need

### Essential
- **Rustup:** https://rustup.rs/ (Rust installer)
- **Cargo:** Built-in with Rust
- **VS Code:** https://code.visualstudio.com/ (with rust-analyzer)

### Recommended
- **Clippy:** `rustup component add clippy` (Linter)
- **Rustfmt:** `rustup component add rustfmt` (Formatter)
- **Miri:** `cargo +nightly install miri` (UB detector)
- **Cargo-watch:** `cargo install cargo-watch` (Auto-test on save)

### Advanced
- **Cargo-flamegraph:** Performance profiling
- **Cargo-tarpaulin:** Code coverage
- **Cargo-criterion:** Benchmarking results
- **Cargo-expand:** See macro expansions

---

## Learning Schedule (22 Weeks Total)

### Weeks 1-3: Module 1 (Ownership)
- 50 hours reading, exercises, practice
- Build confidence with borrow checker

### Weeks 4-5: Module 2 (Traits & Generics)
- 30-40 hours
- Practice with exercises
- Build custom collection type

### Weeks 6-8: Module 3 (Async)
- 40-50 hours
- Deep understanding of Futures
- Build mini-executor

### Weeks 9-11: Module 4 (Unsafe & FFI)
- 30-40 hours
- Study unsafe patterns
- FFI integration

### Weeks 12-13: Module 5 (Metaprogramming)
- 25-35 hours
- Macro patterns
- Procedural macro development

### Weeks 14-16: Module 6 (Performance)
- 40-50 hours
- Benchmarking discipline
- Profiling tools

### Weeks 17-22: Capstone Projects
- Project 1: 3-4 days
- Project 2: 4-5 days
- Project 3: 5-7 days

---

## Self-Assessment Checkpoints

### After Module 1
- [ ] Understand move vs copy semantics
- [ ] Can explain borrow checker rules
- [ ] Know what lifetimes do
- [ ] Pass all ownership exercises

### After Module 2
- [ ] Can implement custom traits
- [ ] Understand monomorphization
- [ ] Use generic constraints effectively
- [ ] Know when to use trait objects

### After Module 3
- [ ] Can explain how async/await works
- [ ] Understand Pin and Unpin
- [ ] Know what Waker does
- [ ] Can write simple async code

### After Module 4
- [ ] Know when unsafe is necessary
- [ ] Can safely use raw pointers
- [ ] Understand FFI basics
- [ ] Know major UB categories

### After Module 5
- [ ] Can write declarative macros
- [ ] Can implement derive macros
- [ ] Understand syn and quote
- [ ] Can build custom DSL

### After Module 6
- [ ] Know zero-cost abstractions
- [ ] Can profile and benchmark code
- [ ] Understand memory layout
- [ ] Can identify optimizations

### After Capstone Projects
- [ ] Completed all 3 projects
- [ ] Confident building systems code
- [ ] Can solve real problems with Rust

---

## Tips for Success

### 1. Code Along
Don't just read—write code. Type out examples.

### 2. Experiment
Change examples, see what breaks, understand why.

### 3. Read Others' Code
Study well-written Rust projects:
- Standard library
- Popular crates (serde, tokio, anyhow)
- GitHub projects

### 4. Measure, Don't Guess
Always benchmark before optimizing.

### 5. Build Something Real
Apply learning to real projects.

### 6. Read Error Messages
Rust's error messages are remarkably helpful.

### 7. Join Communities
Ask questions, discuss ideas, learn from others.

---

## Recommended Reading Order

1. This README (overview)
2. Module 1 notes (2 weeks)
3. Module 1 exercises (1 week)
4. Module 2 notes (1 week)
5. Module 2 exercises (1 week)
6. Mental models (refresh)
7. Glossary (as needed)
8. Other modules (follow similar pattern)
9. Capstone projects (test everything)

---

## Common Questions

### How long does this take?
- Full curriculum: 22 weeks (20 hours/week)
- With prior experience: 16 weeks
- Accelerated: 12 weeks (intensive)

### Can I skip modules?
Yes, but not recommended:
- Skip Module 1: You'll struggle with ownership
- Skip Module 2: Traits are fundamental
- Skip Module 3: Async is increasingly important
- Skip Modules 4-6: Advanced but essential for systems code

### How do I practice?
1. Complete exercises provided
2. Do bonus projects
3. Build your own projects
4. Contribute to open source

### What if I get stuck?
1. Re-read the section
2. Try a simpler example
3. Ask in Rust communities
4. Use Godbolt to see assembly
5. Use Miri to detect UB

---

## Staying Current

Rust evolves. Stay updated:

- **Rust Blog:** https://blog.rust-lang.org/
- **This Week in Rust:** https://this-week-in-rust.org/
- **Release Notes:** New version every 6 weeks

Key changes to watch:
- New language features
- Stability of unstable features
- Ecosystem evolution (Tokio, serde, etc.)

---

## Next Steps After This Curriculum

### Deepen Knowledge
- Read more crate source code
- Study the Rust compiler
- Learn WASM in Rust
- Explore embedded Rust

### Build Projects
- Command-line tools
- Web applications (axum, rocket)
- Game development (bevy)
- Systems programming (embedded, OS)

### Contribute
- Rust standard library
- Popular crates
- Your own crate
- Documentation

### Teach Others
- Blog about what you learn
- Create tutorials
- Answer questions
- Mentor others

---

This curriculum is comprehensive but just the beginning. Rust is a deep language with endless learning opportunities. Good luck on your journey!
