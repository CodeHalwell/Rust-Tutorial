# Unsafe Rust: Breaking Safety Guarantees Responsibly

## The Safety Contract

**Safe Rust guarantees:**
1. No data races
2. No use-after-free
3. No double-free
4. No null pointer dereferences
5. No buffer overflows

**Unsafe Rust says:** "I take responsibility for these guarantees. The compiler won't check."

```rust
unsafe {
    // You're on your own here
}
```

---

## What Unsafe Can Do

Unsafe unblocks these operations:

```rust
// 1. Dereference raw pointers
unsafe {
    let ptr: *const i32 = 0x1000 as *const i32;
    let value = *ptr;  // Could be anything!
}

// 2. Call unsafe functions
unsafe {
    dangerous_function();  // Might violate safety
}

// 3. Mutate static variables
unsafe {
    static mut COUNTER: i32 = 0;
    COUNTER += 1;  // Data race possible!
}

// 4. Implement unsafe traits
struct Node<T> {
    value: T,
    ptr: *mut Node<T>,
}

unsafe impl<T: Sync> Send for Node<T> { }
```

---

## Raw Pointers

Raw pointers are like C pointers—no safety guarantees.

### Creating Raw Pointers

```rust
let value = 42;
let ptr_const: *const i32 = &value as *const i32;
let mut mutable = 43;
let ptr_mut: *mut i32 = &mut mutable as *mut i32;

// These pointers can be:
// - Null
// - Dangling
// - Unaligned
// - Pointing to freed memory
```

### Dereferencing Raw Pointers

```rust
let value = 42;
let ptr = &value as *const i32;

unsafe {
    println!("{}", *ptr);  // Must be unsafe
}

// You promise:
// 1. ptr is valid
// 2. ptr is properly aligned
// 3. ptr points to initialized value
// 4. Not in UB zone
```

### Raw Pointers Can Outlive References

```rust
fn get_dangling_ptr() -> *const i32 {
    let value = 42;
    &value as *const i32
}  // value dropped

let ptr = get_dangling_ptr();
unsafe {
    println!("{}", *ptr);  // ❌ UNDEFINED BEHAVIOR (dangling)
}
```

---

## Unsafe Functions

A function is unsafe if it expects callers to maintain safety invariants.

### Marking Functions Unsafe

```rust
unsafe fn dereference_ptr(ptr: *const i32) -> i32 {
    *ptr  // Caller must ensure ptr is valid
}

fn main() {
    let value = 42;
    let ptr = &value as *const i32;

    unsafe {
        println!("{}", dereference_ptr(ptr));  // ✅ Safe here
    }

    // Creating dangling pointer
    let dangling = {
        let v = 42;
        &v as *const i32
    };

    unsafe {
        println!("{}", dereference_ptr(dangling));  // ❌ UB
    }
}
```

### Safety Contracts

```rust
/// Reads from a raw pointer without bounds checking.
///
/// # Safety
///
/// Caller must ensure:
/// - `ptr` is valid and aligned for type T
/// - `ptr` points to initialized T
/// - `ptr` is not accessed from another thread
unsafe fn read_unchecked<T>(ptr: *const T) -> T {
    std::ptr::read(ptr)
}
```

---

## Safe Abstractions Over Unsafe Code

The goal: use unsafe internally, expose only safe API.

### Example: Custom Vector

```rust
pub struct Vec<T> {
    ptr: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> Vec<T> {
    pub fn new() -> Self {
        Vec {
            ptr: std::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    pub fn push(&mut self, value: T) {
        if self.len == self.capacity {
            self.grow();
        }

        unsafe {
            // Safe: we know ptr is valid and has capacity
            std::ptr::write(self.ptr.add(self.len), value);
        }

        self.len += 1;
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            unsafe {
                // Safe: we checked bounds
                Some(&*self.ptr.add(index))
            }
        } else {
            None
        }
    }

    fn grow(&mut self) {
        // ... realloc logic ...
    }
}

// Users never see unsafe!
let mut v = Vec::new();
v.push(42);
println!("{:?}", v.get(0));  // ✅ Safe API
```

---

## Undefined Behavior (UB) Categories

### 1. Invalid Memory Access

```rust
// Dangling pointer
let ptr = {
    let x = 5;
    &x as *const i32
};

unsafe {
    println!("{}", *ptr);  // ❌ UB
}

// Out of bounds
let arr = [1, 2, 3];
let ptr = &arr[0] as *const i32;
unsafe {
    println!("{}", *ptr.add(10));  // ❌ UB
}
```

### 2. Data Race

```rust
static mut COUNTER: i32 = 0;

std::thread::spawn(|| {
    unsafe { COUNTER += 1; }  // No synchronization!
});

unsafe { COUNTER += 1; }  // ❌ UB (data race)
```

### 3. Integer Overflow (Debug Only)

```rust
let x: u32 = u32::MAX;
let y = x + 1;  // ❌ UB in debug, wraps in release

// Explicit wrapping
let y = x.wrapping_add(1);  // ✅ Defined: 0
```

### 4. Invalid Cast

```rust
let value: u32 = 123;
let ptr = &value as *const u32 as *const u8;

unsafe {
    println!("{}", *ptr);  // ❌ UB (unaligned access on some platforms)
}
```

### 5. Breaking Invariants

```rust
impl Container {
    // Invariant: buffer.len() == count
    buffer: Vec<i32>,
    count: usize,
}

impl Container {
    unsafe fn corrupt(&mut self) {
        self.buffer.clear();
        self.count = 1000;  // ❌ Invariant broken!
    }
}
```

---

## Detecting UB with Miri

**Miri** is an interpreter that detects many forms of UB.

```bash
# Install
cargo +nightly install miri

# Run tests with Miri
cargo +nightly miri test

# Run a program with Miri
cargo +nightly miri run
```

**Miri catches:**
- Dangling pointers
- Out of bounds access
- Data races (sometimes)
- Uninitialized memory reads
- Invalid enum discriminants

---

## When to Use Unsafe

### ✅ Valid Use Cases

1. **Performance critical path**
   ```rust
   unsafe {
       // Avoid bounds checks in hot loop
       *result.get_unchecked_mut(i) = value;
   }
   ```

2. **Interfacing with C**
   ```rust
   unsafe {
       libc::malloc(size)
   }
   ```

3. **Custom data structures**
   ```rust
   // Vec, HashMap, linked list need unsafe internally
   ```

4. **Low-level systems code**
   ```rust
   // OS kernel, firmware, etc.
   ```

### ❌ Avoid

- Ignoring borrow checker ("I know better")
- Random optimizations without benchmarking
- Avoiding bounds checks just because
- Complex logic in unsafe blocks

---

## Unsafe Block Best Practices

### 1. Minimal Unsafe Blocks

```rust
// ❌ Bad: whole function is unsafe
unsafe fn process(data: &[i32]) {
    for i in 0..data.len() {
        let val = *data.as_ptr().add(i);
        println!("{}", val);
    }
}

// ✅ Good: minimal unsafe
fn process(data: &[i32]) {
    for i in 0..data.len() {
        let val = unsafe { *data.as_ptr().add(i) };
        println!("{}", val);
    }
}
```

### 2. Document Safety Invariants

```rust
/// Returns the element without bounds checking.
///
/// # Safety
///
/// Caller must ensure index < self.len()
pub unsafe fn get_unchecked(&self, index: usize) -> &T {
    &self.data[index]
}
```

### 3. Encapsulate in Safe API

```rust
pub struct SafeWrapper {
    ptr: *mut i32,
}

// Users only see safe methods
impl SafeWrapper {
    pub fn new(value: i32) -> Self {
        // ...
    }

    pub fn read(&self) -> i32 {
        unsafe { *self.ptr }  // ✅ Safe: we maintain invariants
    }
}
```

---

## Key Patterns

### Pattern: Initialized-But-Uninitialized Optimization

```rust
// Avoid zeroing memory when creating large vector
let mut vec = Vec::with_capacity(1000);

unsafe {
    vec.set_len(1000);  // Claim items exist without zeroing
}

// Initialize as you go
for i in 0..1000 {
    vec[i] = compute_value(i);
}
```

### Pattern: Interior Mutability With Unsafe

```rust
pub struct UnsafeCell<T> {
    value: T,
}

impl<T> UnsafeCell<T> {
    pub fn get(&self) -> *mut T {
        &self.value as *const T as *mut T
    }
}

// Callers must use responsibly
```

---

## Summary

| Aspect | Safe Rust | Unsafe Rust |
|--------|-----------|------------|
| **Guarantees** | Compiler verified | Your responsibility |
| **Performance** | Some overhead | No overhead |
| **Complexity** | Simple | Can be complex |
| **Use** | 99% of code | Necessary but minimal |
| **Testing** | Logic errors | Memory safety errors |

**Golden Rule:** Use unsafe when necessary, keep it minimal, encapsulate it, and document it thoroughly.
