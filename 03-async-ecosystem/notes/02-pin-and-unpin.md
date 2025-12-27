# Pin and Unpin: Guaranteeing Values Won't Move

## The Problem: Self-Referential Structs

Some structs need to reference themselves, which is dangerous if they move in memory.

```rust
struct Node {
    data: i32,
    next: Option<&Node>,  // ❌ Self-referential!
}

// If Node moves in memory, the pointer becomes dangling:
let mut node = Node { data: 1, next: None };
let ptr = &node;  // Points to memory location 0x1000
let node = Box::new(node);  // Moves to heap (0x2000)
// ptr is now dangling!
```

**Problem intensified with async:** Async functions store local variables in state machines that must not move.

---

## What is Pin?

`Pin<P<T>>` is a wrapper that **guarantees T will not move after being pinned**.

```rust
pub struct Pin<P: Deref> {
    pointer: P,
}

// Common usage:
Pin<&mut T>     // Pinned mutable reference
Pin<Box<T>>     // Pinned boxed value
```

**Guarantee:** If you have `Pin<&mut T>`, T will not move for the lifetime of the pin.

---

## Pin API

### Creating Pinned Values

```rust
let data = 5;
let pinned_ref = Pin::new(&data);
// Now you can't move data while pinned_ref exists

let boxed = Box::new(String::from("hello"));
let pinned_box = Box::pin(boxed);
// Box never moves to heap again
```

### Unpinning (Moving)

```rust
let pinned = Pin::new(&mut value);

// UNSAFE! Only safe if T implements Unpin
unsafe {
    let unpinned: &mut T = Pin::into_inner(pinned);
}
```

---

## Unpin Trait

Most types automatically implement `Unpin`, meaning they can be safely moved even after being pinned.

```rust
// These are Unpin (safe to move):
impl Unpin for i32 { }
impl Unpin for String { }
impl Unpin for Vec<T> { }

// Meaning:
let mut value = 5;
let pinned = Pin::new(&mut value);
// Can safely move value even though pinned
// (because Pin::into_inner would be safe)
```

**Unpin = "I don't have self-referential pointers"**

### Opting Out of Unpin

```rust
use std::marker::PhantomPinned;

struct Node {
    data: i32,
    ptr: Option<*const Node>,  // Self-referential pointer
    _pin: PhantomPinned,        // "I'm not Unpin!"
}

// Now Node does NOT implement Unpin
// If pinned, it CANNOT be moved
```

---

## Why Pin Matters for Async

Async functions compile to state machines. When you `.await`, the local variables must be stored in the enum.

### The Danger Without Pin

```rust
async fn dangerous() {
    let data = String::from("hello");
    let ptr: *const String = &data;  // Pointer to data

    some_async_op().await;  // ← state machine moves here!
                             // data is moved within the enum
    // ptr is now dangling!

    unsafe {
        println!("{}", *ptr);  // Use-after-free!
    }
}
```

### Protected by Pin

```rust
// If some_async_op() takes Pin<&mut Self>,
// the state machine is guaranteed not to move
// So the pointer remains valid!
```

---

## Pattern: Pinning in Async Code

### Correct Pattern

```rust
async fn safe_async() {
    let data = String::from("hello");
    // data is not self-referential, so it's OK to move

    some_async().await;  // data moved into state machine

    println!("{}", data);  // ✅ Safe
}
```

### Dangerous Pattern (Won't Compile)

```rust
async fn dangerous() {
    let data = String::from("hello");
    let ptr = &data as *const String;

    some_async().await;  // ❌ Compiler error
                         // data might move, invalidating ptr
    unsafe {
        println!("{}", *ptr);
    }
}
```

---

## Self-Referential Structs Without Async

You can create self-referential structs with `Pin`:

```rust
struct Node {
    data: String,
    pointer: Option<*const String>,
    _pin: PhantomPinned,
}

impl Node {
    fn new(data: String) -> Pin<Box<Self>> {
        let node = Node {
            data,
            pointer: None,
            _pin: PhantomPinned,
        };

        let mut boxed = Box::pin(node);

        // Now that it's pinned and won't move, create the self-pointer
        unsafe {
            let self_ptr: *const String = &boxed.data as *const String;
            Pin::as_mut(&mut boxed).pointer = Some(self_ptr);
        }

        boxed
    }

    fn get_data(&self) -> Option<&String> {
        self.pointer.map(|ptr| {
            unsafe {
                &*ptr  // Safe because pinned
            }
        })
    }
}
```

---

## Implementing Pin-Based Types

### Custom Future Type

```rust
pub struct MyFuture {
    state: FutureState,
    _pin: PhantomPinned,
}

enum FutureState {
    Start,
    Waiting { /* ... */ },
    Done,
}

impl Future for MyFuture {
    type Output = String;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<String> {
        // Self is pinned - value won't move
        // Safe to use pointers to self fields
        Poll::Ready("done".to_string())
    }
}

// Can only create via Box::pin or similar
fn create_future() -> Pin<Box<MyFuture>> {
    Box::pin(MyFuture {
        state: FutureState::Start,
        _pin: PhantomPinned,
    })
}
```

---

## Common Pin Patterns

### Pattern 1: Pin Projection

```rust
struct Both<T, U> {
    first: T,
    second: U,
}

impl<T, U> Both<T, U> {
    fn pin_first(self: Pin<&mut Self>) -> Pin<&mut T> {
        unsafe { Pin::map_unchecked_mut(self, |s| &mut s.first) }
    }

    fn pin_second(self: Pin<&mut Self>) -> Pin<&mut U> {
        unsafe { Pin::map_unchecked_mut(self, |s| &mut s.second) }
    }
}

// Safe if first/second don't depend on each other
```

### Pattern 2: Conditional Unpin

```rust
impl<T: Unpin> Unpin for Container<T> { }

// Container<String> is Unpin (String is Unpin)
// Container<Node> is NOT Unpin (Node is not Unpin)
```

---

## Debugging Pin Issues

### Error: "struct is not unpin"

```rust
// Causes error:
let x = Node { ... };
let y = x;  // ❌ Cannot move Node

// Solution: Keep it pinned
let x = Box::pin(Node { ... });
// Can't move anymore
```

### Error: "Cannot destructure pinned value"

```rust
let pinned = Pin::new(&mut node);
let Node { data, ptr } = pinned;  // ❌ Error

// Solution: Use Pin methods
let data = &pinned.data;  // ✅ Works
```

---

## Key Takeaways

| Concept | Meaning |
|---------|---------|
| **Pin<P<T>>** | T is pinned and won't move |
| **Unpin** | Safe to move even if pinned |
| **PhantomPinned** | "I'm not Unpin" marker |
| **Box::pin()** | Create a pinned box |
| **Pin::as_ref()** | Get pinned reference |
| **Pin::as_mut()** | Get pinned mutable reference |

**In practice:** Most types are Unpin. You only need to think about Pin when:
1. Using Futures (compiler handles it)
2. Creating self-referential structures
3. Implementing custom Futures

For 99% of code, you don't interact with Pin directly—the compiler and libraries handle it for you.
