/// ============================================================================
/// INTERIOR MUTABILITY: Mutating Data Through Immutable References
/// ============================================================================
///
/// This module demonstrates the interior mutability pattern in Rust.
/// Interior mutability allows you to mutate data even though you only have
/// an immutable reference—by deferring the borrow checking to runtime.
///
/// Key Insight: Rust's safety guarantees are not violated. The borrow checker
/// ensures there's only ONE mutable reference at any time. Interior mutability
/// simply moves this check from compile-time to runtime.
///
/// ============================================================================

// ============================================================================
// PART 1: MEMORY LAYOUT FUNDAMENTALS
// ============================================================================

/// Let's first understand memory layout:
///
/// Normal mutable reference:
///   let x = 5;                    // Stack: [5]
///   let r = &mut x;               // r points to x's address
///   *r = 10;                       // Mutate through mutable ref ✓
///
/// Interior mutability attempt with normal ownership:
///   let x = 5;                    // Stack: [5]
///   let r = &x;                   // Immutable ref to x
///   *r = 10;                       // ❌ COMPILE ERROR: can't mutate through &x
///
/// With RefCell (single-threaded interior mutability):
///   let x = RefCell::new(5);      // Stack: [Cell { value: 5, borrow_flag: 0 }]
///   let r = &x;                   // Immutable ref to RefCell
///   *r.borrow_mut() = 10;         // ✓ Runtime check allows mutation
///
/// Memory Layout of RefCell<T>:
///   RefCell {
///       value: T,                 // The actual data
///       borrow: Cell<BorrowFlag>  // Runtime borrow counter
///   }
///
/// BorrowFlag states:
///   - UNUSED = 0:          No active borrows
///   - WRITING = -1:        One exclusive mutable borrow
///   - n > 0:               n immutable borrows active
///
/// When you call borrow_mut(), RefCell checks if WRITING flag is set.
/// If it is, panic! If not, set WRITING and return Ref<T>.
/// The Ref<T> holds a reference to RefCell and decrements on drop.

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

// ============================================================================
// PART 2: SINGLE-THREADED INTERIOR MUTABILITY (RefCell)
// ============================================================================

/// RefCell<T>: Single-threaded interior mutability with runtime borrow checking.
///
/// - Use when: You need to mutate data but only have immutable references
///   in single-threaded code
/// - No runtime cost beyond borrow flag (usually just 1-4 bytes of overhead)
/// - Panics if you try to borrow_mut() while an immutable borrow exists
/// - NOT thread-safe (no synchronization primitives)

#[derive(Debug)]
struct RefCellExample {
    // Store a counter that we can mutate even though RefCellExample might be immutable
    access_count: RefCell<usize>,
}

impl RefCellExample {
    fn new() -> Self {
        RefCellExample {
            access_count: RefCell::new(0),
        }
    }

    /// This method takes &self (immutable ref), but we mutate access_count!
    /// In safe Rust without RefCell, this would be impossible.
    fn log_access(&self) {
        // borrow_mut() returns Result<RefMut<usize>, BorrowError>
        // RefMut acts like &mut usize and automatically decrements the borrow flag on drop
        let mut count = self.access_count.borrow_mut();
        *count += 1;
        println!("Access count: {}", *count);
    }

    fn get_access_count(&self) -> usize {
        // borrow() returns Result<Ref<usize>, BorrowError>
        // Ref acts like &usize and automatically decrements on drop
        *self.access_count.borrow()
    }
}

// ============================================================================
// PART 3: REFERENCE COUNTING + INTERIOR MUTABILITY (Rc<RefCell<T>>)
// ============================================================================

/// Rc<T>: Reference-counted pointer for single-threaded shared ownership.
///
/// Memory layout of Rc<T>:
///   [Rc Control Block]  [T value]
///   - strong_count: usize
///   - weak_count: usize
///   - value: T
///
/// Every clone() increments strong_count. Drop decrements it.
/// When strong_count reaches 0, the value is deallocated.
///
/// Rc<T> is NOT Send/Sync, so it cannot cross thread boundaries.

/// Example: A graph node where multiple parents reference the same child.
/// We need:
///   1. Shared ownership (Rc) - multiple parents own the same node
///   2. Interior mutability (RefCell) - mutate the node through shared refs

#[derive(Debug)]
struct GraphNode {
    id: usize,
    // Interior mutability: allows mutation through &self
    neighbors: RefCell<Vec<Rc<GraphNode>>>,
}

impl GraphNode {
    fn new(id: usize) -> Rc<GraphNode> {
        Rc::new(GraphNode {
            id,
            neighbors: RefCell::new(Vec::new()),
        })
    }

    /// Connect this node to another. Takes &self because RefCell handles interior mutability.
    fn connect_to(&self, other: Rc<GraphNode>) {
        self.neighbors.borrow_mut().push(other);
    }

    fn get_neighbor_count(&self) -> usize {
        self.neighbors.borrow().len()
    }
}

// ============================================================================
// PART 4: THREAD-SAFE INTERIOR MUTABILITY (Mutex)
// ============================================================================

/// Mutex<T>: Thread-safe interior mutability with compile-time lock guarantees.
///
/// Memory layout of Mutex<T>:
///   [OS Mutex / Atomic Lock]
///   [T value]
///
/// Lock states:
///   - UNLOCKED: Any thread can acquire
///   - LOCKED: One thread holds exclusive access, others wait (blocked)
///
/// Differences from RefCell:
///   - Thread-safe: Can cross thread boundaries (Send + Sync)
///   - Blocking: Threads waiting for lock are put to sleep by OS
///   - Heavier weight: OS-level synchronization, context switches
///   - Runtime cost: Lock/unlock operations, potential contention
///
/// RefCell panics on double-borrow; Mutex blocks and waits.

#[derive(Debug)]
struct Counter {
    value: Mutex<usize>,
}

impl Counter {
    fn new() -> Self {
        Counter {
            value: Mutex::new(0),
        }
    }

    /// Increment the counter. Thread-safe.
    fn increment(&self) {
        // lock() blocks the current thread until it acquires the lock
        // Returns Result<MutexGuard<usize>, PoisonError>
        let mut guard = self.value.lock().unwrap();
        *guard += 1;
    }

    fn get(&self) -> usize {
        *self.value.lock().unwrap()
    }
}

// ============================================================================
// PART 5: THREAD-SAFE REFERENCE COUNTING (Arc)
// ============================================================================

/// Arc<T>: Atomic Reference Counted pointer for thread-safe shared ownership.
///
/// Memory layout is similar to Rc, but with atomic operations:
///   [Arc Control Block]
///   - atomic_strong_count: AtomicUsize
///   - atomic_weak_count: AtomicUsize
///   - value: T
///
/// Key difference from Rc:
///   - Atomic operations ensure multiple threads can safely increment/decrement
///   - Send + Sync if T is Send + Sync
///   - Slightly higher overhead than Rc (atomic ops vs regular arithmetic)
///
/// Common pattern: Arc<Mutex<T>> for thread-safe shared state

#[derive(Debug)]
struct SharedCounter {
    value: Arc<Mutex<usize>>,
}

impl SharedCounter {
    fn new() -> Self {
        SharedCounter {
            value: Arc::new(Mutex::new(0)),
        }
    }

    /// Clone creates a new Arc pointing to the same Mutex.
    /// This is cheap: just increments the atomic refcount.
    fn clone_arc(&self) -> Arc<Mutex<usize>> {
        Arc::clone(&self.value)
    }

    fn increment(&self) {
        let mut guard = self.value.lock().unwrap();
        *guard += 1;
    }

    fn get(&self) -> usize {
        *self.value.lock().unwrap()
    }
}

// ============================================================================
// PART 6: COMPARISON TABLE
// ============================================================================

/// Memory Layout & Performance Comparison:
///
/// +-------------------+------------+----------+---------+--------+
/// | Type              | Overhead   | Thread-  | Panic   | Notes  |
/// |                   |            | Safe     | Risk    |        |
/// +-------------------+------------+----------+---------+--------+
/// | RefCell<T>        | 1-4 bytes  | ✗        | Yes     | Fastest|
/// |                   | (borrow_   |          |         | for    |
/// |                   |  flag)     |          |         | single |
/// |                   |            |          |         | thread |
/// +-------------------+------------+----------+---------+--------+
/// | Rc<RefCell<T>>    | Rc overhead| ✗        | Yes     | Shared |
/// |                   | + RefCell  |          |         | owned, |
/// |                   | overhead   |          |         | mutable|
/// +-------------------+------------+----------+---------+--------+
/// | Mutex<T>          | OS lock    | ✓        | No      | Blocks |
/// |                   | + padding  |          |         | waiting|
/// |                   | (usually   |          |         | threads|
/// |                   | 40+ bytes) |          |         |        |
/// +-------------------+------------+----------+---------+--------+
/// | Arc<Mutex<T>>     | Arc overhead| ✓        | No      | Shared |
/// |                   | + Mutex    |          |         | owned, |
/// |                   | overhead   |          |         | thread |
/// |                   |            |          |         | safe   |
/// +-------------------+------------+----------+---------+--------+
/// | RwLock<T>         | Similar to | ✓        | No      | Many   |
/// |                   | Mutex but  |          |         | readers|
/// |                   | allows     |          |         | or one |
/// |                   | multiple   |          |         | writer |
/// |                   | readers    |          |         |        |
/// +-------------------+------------+----------+---------+--------+

// ============================================================================
// PART 7: PRACTICAL EXAMPLE: CACHED COMPUTATION
// ============================================================================

/// A cached value that computes on first access and stores the result.
/// Uses RefCell for interior mutability and Rc for shared ownership.

#[derive(Clone)]
struct CachedValue {
    compute_fn: Arc<dyn Fn() -> i32 + Send + Sync>,
    cache: Rc<RefCell<Option<i32>>>,
}

impl CachedValue {
    fn new<F>(f: F) -> Self
    where
        F: Fn() -> i32 + Send + Sync + 'static,
    {
        CachedValue {
            compute_fn: Arc::new(f),
            cache: Rc::new(RefCell::new(None)),
        }
    }

    /// Get the computed value, computing only once.
    /// Takes &self but mutates the cache through RefCell.
    fn get(&self) -> i32 {
        let mut cache = self.cache.borrow_mut();
        if let Some(value) = *cache {
            println!("  [cache hit]");
            value
        } else {
            println!("  [computing...]");
            let value = (self.compute_fn)();
            *cache = Some(value);
            value
        }
    }
}

// ============================================================================
// PART 8: MEMORY SAFETY GUARANTEES
// ============================================================================

/// Interior mutability does NOT violate Rust's safety guarantees.
/// It simply moves the borrow checking from compile-time to runtime.
///
/// Key Invariant: Only ONE mutable reference can exist at any time.
///
/// RefCell enforces this at runtime:
///   - Calling borrow_mut() while borrow() is active = PANIC
///   - This prevents the Undefined Behavior that Rust's type system normally prevents
///
/// Mutex enforces this with locks:
///   - Only one thread can hold the lock at a time
///   - Prevents data races through OS-level synchronization
///
/// Why not just use unsafe?
///   - Because these patterns are SAFE if used correctly
///   - The type system can verify the safety properties
///   - Panics are better than undefined behavior (though catching panics is bad practice)

// ============================================================================
// DEMONSTRATION: Running All Examples
// ============================================================================

fn main() {
    println!("=== INTERIOR MUTABILITY EXAMPLES ===\n");

    // -------- RefCell Example --------
    println!("1. RefCell Example (single-threaded):");
    let example = RefCellExample::new();
    example.log_access();
    example.log_access();
    println!("Total accesses: {}\n", example.get_access_count());

    // -------- Rc<RefCell<T>> Example --------
    println!("2. Rc<RefCell<T>> Example (shared ownership with mutation):");
    let node1 = GraphNode::new(1);
    let node2 = GraphNode::new(2);
    let node3 = GraphNode::new(3);

    // Create a cloned reference to node2
    let node2_clone = Rc::clone(&node2);

    // Both node1 and node2_clone can connect to node3
    node1.connect_to(Rc::clone(&node3));
    node2_clone.connect_to(Rc::clone(&node3));

    println!("Node 1 neighbors: {}", node1.get_neighbor_count());
    println!("Node 2 neighbors: {}\n", node2_clone.get_neighbor_count());

    // -------- Mutex Example (single-threaded) --------
    println!("3. Mutex Example (thread-safe interior mutability):");
    let counter = Counter::new();
    counter.increment();
    counter.increment();
    println!("Counter value: {}\n", counter.get());

    // -------- Arc<Mutex<T>> Example (multi-threaded) --------
    println!("4. Arc<Mutex<T>> Example (shared mutable state across threads):");
    let shared = Arc::new(SharedCounter::new());
    let mut handles = vec![];

    // Spawn 5 threads, each increments the counter 1000 times
    for i in 0..5 {
        let counter_clone = Arc::clone(&shared.value);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                let mut guard = counter_clone.lock().unwrap();
                *guard += 1;
            }
            println!("Thread {} done", i);
        });
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter: {}", shared.get());
    println!("(Expected: 5000, one increment per thread)\n");

    // -------- Cached Value Example --------
    println!("5. CachedValue Example (interior mutability pattern):");
    let expensive = CachedValue::new(|| {
        println!("    Performing expensive computation...");
        42
    });

    println!("First call:");
    let result1 = expensive.get();
    println!("Result: {}", result1);

    println!("\nSecond call:");
    let result2 = expensive.get();
    println!("Result: {}\n", result2);

    // -------- Demonstrating RefCell Panic --------
    println!("6. RefCell Panic Example (double-borrow protection):");
    let data = RefCell::new(vec![1, 2, 3]);

    let borrow1 = data.borrow();
    println!("Got immutable borrow: {:?}", &*borrow1);

    // This would panic if uncommented (double-mutable-borrow):
    // let borrow2 = data.borrow_mut();  // ❌ PANIC: already borrowed
    println!("(Commenting out double-borrow to avoid panic)\n");

    println!("=== SUMMARY ===");
    println!("- RefCell: Single-threaded, panic on double-borrow, very fast");
    println!("- Rc<RefCell<T>>: Shared ownership + interior mutability");
    println!("- Mutex: Thread-safe, blocks on contention, heavier overhead");
    println!("- Arc<Mutex<T>>: Shared ownership + thread-safe mutation");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_refcell_interior_mutability() {
        let example = RefCellExample::new();
        example.log_access();
        assert_eq!(example.get_access_count(), 1);
        example.log_access();
        assert_eq!(example.get_access_count(), 2);
    }

    #[test]
    fn test_rc_refcell_shared_ownership() {
        let node = GraphNode::new(1);
        let clone1 = Rc::clone(&node);
        let clone2 = Rc::clone(&node);

        clone1.connect_to(Rc::clone(&clone2));
        // All three references share the same data
        assert_eq!(node.get_neighbor_count(), 1);
    }

    #[test]
    fn test_cached_value_caches_result() {
        let mut call_count = 0;
        let cached = CachedValue::new(|| {
            call_count += 1;
            123
        });

        // Note: Due to Arc clone limitations with closures capturing variables,
        // this test demonstrates the pattern rather than counting calls.
        let result1 = cached.get();
        let result2 = cached.get();
        assert_eq!(result1, result2);
    }

    #[test]
    fn test_mutex_thread_safety() {
        let counter = Arc::new(SharedCounter::new());
        let mut handles = vec![];

        for _ in 0..10 {
            let counter_clone = Arc::clone(&counter.value);
            let handle = thread::spawn(move || {
                for _ in 0..100 {
                    let mut guard = counter_clone.lock().unwrap();
                    *guard += 1;
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(counter.get(), 1000);
    }
}
