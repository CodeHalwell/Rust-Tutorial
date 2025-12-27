# Futures: The Foundation of Async Rust

## What is a Future?

A **Future** is an abstraction for a value that might not be available yet. Instead of blocking until it arrives, Futures allow your program to do other work while waiting.

```rust
pub trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output>;
}

pub enum Poll<T> {
    Ready(T),
    Pending,
}
```

**Key insight:** Futures are lazy—nothing happens until you poll them.

---

## How Futures Work

### The Poll Interface

A Future has one method: `poll()`. The runtime calls it repeatedly until it returns `Ready`.

```rust
let future = some_async_operation();

// First poll
match future.poll(&mut cx) {
    Poll::Ready(value) => println!("Done! {}", value),
    Poll::Pending => println!("Not ready yet"),
}

// Later, when more work is available
match future.poll(&mut cx) {
    Poll::Ready(value) => println!("Done! {}", value),
    Poll::Pending => println!("Still waiting"),
}
```

### Simple Future Example

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct CountToThree {
    count: u32,
}

impl Future for CountToThree {
    type Output = String;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context) -> Poll<String> {
        if self.count < 3 {
            self.count += 1;
            Poll::Pending  // Not done yet
        } else {
            Poll::Ready("Reached three!".to_string())  // Done!
        }
    }
}

// Using the future
let future = CountToThree { count: 0 };
// Would need executor to poll and run to completion
```

---

## Async/Await Desugaring

`async fn` and `.await` are syntactic sugar that the compiler transforms into Futures.

### What You Write

```rust
async fn fetch_user(id: u32) -> String {
    let user = get_from_api(id).await;    // ← suspend point
    let profile = get_profile(&user).await;  // ← suspend point
    format!("{:?}", profile)
}
```

### What the Compiler Generates (Simplified)

```rust
fn fetch_user(id: u32) -> impl Future<Output = String> {
    enum FetchUserFuture {
        // Start - initial state
        GetUser { id: u32 },
        // Waiting for get_from_api
        WaitingForUser { future: GetFromApiFuture },
        // Waiting for get_profile
        WaitingForProfile { user: String, future: GetProfileFuture },
        // Done
        Done { profile: String },
    }

    impl Future for FetchUserFuture {
        type Output = String;

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<String> {
            loop {
                match self {
                    GetUser { id } => {
                        let future = get_from_api(*id);
                        *self = WaitingForUser { future };
                        // Continue to next match
                    }
                    WaitingForUser { future } => {
                        match Pin::new(future).poll(cx) {
                            Poll::Ready(user) => {
                                let profile_future = get_profile(&user);
                                *self = WaitingForProfile {
                                    user,
                                    future: profile_future,
                                };
                                // Continue
                            }
                            Poll::Pending => return Poll::Pending,
                        }
                    }
                    WaitingForProfile { future, .. } => {
                        match Pin::new(future).poll(cx) {
                            Poll::Ready(profile) => {
                                *self = Done { profile };
                                return Poll::Ready(format!("{:?}", profile));
                            }
                            Poll::Pending => return Poll::Pending,
                        }
                    }
                    Done { .. } => panic!("Polled after completion"),
                }
            }
        }
    }

    FetchUserFuture::GetUser { id }
}
```

**Key insight:** Each `.await` point becomes a state in an enum!

---

## The Runtime (Executor)

The **runtime** (like Tokio) repeatedly polls Futures until they're all done.

```
Runtime Loop (Simplified):

1. Start with list of Futures
2. While futures exist:
   a. Poll each Future
   b. If Poll::Ready -> remove from list, handle result
   c. If Poll::Pending -> leave in list
3. When no more futures, exit
```

### Simple Executor Example

```rust
use std::collections::VecDeque;
use std::future::Future;
use std::task::Context;

struct SimpleExecutor {
    futures: VecDeque<Box<dyn Future<Output = ()>>>,
}

impl SimpleExecutor {
    fn run(&mut self) {
        while let Some(mut future) = self.futures.pop_front() {
            // Would need to create context
            let mut cx = /* ... */;

            match Pin::new(&mut future).poll(&mut cx) {
                Poll::Ready(_) => { /* Future done */ }
                Poll::Pending => {
                    self.futures.push_back(future);
                }
            }
        }
    }
}
```

---

## Waker: Efficient Notification

Instead of the runtime constantly polling, **Wakers** notify when a Future is ready to make progress.

### How Waker Works

```
1. Future is polled, returns Poll::Pending
2. Before returning, Future registers a Waker with the resource
3. When resource is ready (file loaded, network response, etc.):
   - OS/runtime calls waker.wake()
4. Runtime puts Future back on runnable list
5. Next iteration, runtime polls it again (should return Poll::Ready)
```

### Example: File Read

```
Timeline:

T=0:   Runtime polls Future
       Future calls read_file("/data.txt")
       Returns Poll::Pending with Waker registered

T=0:   OS starts disk I/O

T=5ms: Disk I/O completes
       OS notifies Waker: "Your file is ready!"
       Waker puts Future on runnable queue

T=5ms: Runtime polls Future again
       File data is now available
       Returns Poll::Ready(contents)
```

**Without Waker (naive implementation):**
- Runtime would poll every microsecond
- 100% CPU usage even while waiting
- Terrible battery life

**With Waker:**
- Runtime only polls when notified
- Thread can sleep while waiting
- Efficient resource usage

---

## Composing Futures

### Sequential: Then

```rust
async fn get_user_posts(user_id: u32) -> Vec<Post> {
    let user = get_user(user_id).await;     // First
    let posts = get_posts(&user).await;     // Then
    posts
}
```

### Concurrent: Join

```rust
async fn get_user_and_posts(user_id: u32) -> (User, Vec<Post>) {
    let (user, posts) = tokio::join!(
        get_user(user_id),
        get_posts(user_id)
    ).await;
    (user, posts)
}
```

### Multiple: Select

```rust
async fn first_response() -> String {
    tokio::select! {
        result1 = async { get_data("/api1") } => {
            format!("Got: {}", result1)
        }
        result2 = async { get_data("/api2") } => {
            format!("Got: {}", result2)
        }
    }
}
```

---

## Error Handling

Futures can produce errors just like synchronous code.

```rust
async fn fetch_with_retry(url: &str) -> Result<String, HttpError> {
    for attempt in 0..3 {
        match http_get(url).await {
            Ok(data) => return Ok(data),
            Err(e) if attempt < 2 => continue,  // Retry
            Err(e) => return Err(e),
        }
    }
    unreachable!()
}
```

---

## Cancellation

Futures can be cancelled by dropping them.

```rust
let future = long_running_operation();

tokio::select! {
    result = &mut future => {
        println!("Done: {}", result);
    }
    _ = tokio::time::sleep(Duration::from_secs(5)) => {
        println!("Timeout!");
        drop(future);  // Cancel the future
    }
}
```

---

## Key Takeaways

1. **Futures are lazy:** Nothing happens until polled
2. **Poll-based:** Returns either Ready or Pending
3. **Waker-based scheduling:** Efficient notification system
4. **Async/await is sugar:** Compiles to state machines
5. **Composable:** Can combine multiple Futures
6. **Cancellable:** Drop to cancel
7. **Error handling:** Works with Result, Option, etc.

---

## Common Patterns

### Timeout

```rust
tokio::select! {
    result = some_future => result,
    _ = tokio::time::sleep(Duration::from_secs(10)) => {
        Err("Timeout".into())
    }
}
```

### Retry with Backoff

```rust
let mut interval = Duration::from_millis(100);
loop {
    match operation().await {
        Ok(result) => return Ok(result),
        Err(_) => {
            tokio::time::sleep(interval).await;
            interval *= 2;  // Exponential backoff
        }
    }
}
```

### Spawning Tasks

```rust
tokio::spawn(async {
    // Runs concurrently
    long_running_operation().await
});
```

---

## Performance Characteristics

| Operation | Cost |
|-----------|------|
| Creating a Future | Very cheap (just data) |
| Polling a ready Future | Microseconds |
| Polling a pending Future | Returns immediately |
| Task spawn | Microseconds, allocates ~50-100 bytes |
| Context switch (blocking) | Milliseconds |

**Key:** Async is efficient when Futures spend time in Pending state.
