# Tokio Internals: The Runtime Architecture

## What Tokio Is

Tokio is an **asynchronous runtime**: it owns the executor loop that polls futures, plus drivers for I/O (epoll/kqueue/IOCP), timers, and a thread pool.

```rust
#[tokio::main]              // builds a multi-threaded runtime
async fn main() {
    tokio::spawn(async { /* task */ }).await;
}

#[tokio::main(flavor = "current_thread")]  // single-threaded runtime
async fn main() { ... }
```

---

## Core Components

```
┌─────────────────────────────────────────────┐
│                 Your async code              │
│  tokio::spawn / join! / select! / channels  │
├─────────────────────────────────────────────┤
│  Scheduler: work-stealing queues per worker  │
│  Tasks: Arc<Task> with a Header + state     │
├─────────────────────────────────────────────┤
│  Drivers: I/O (mio), Timer, Signal, Blocking │
│  park/unpark each worker thread on events   │
├─────────────────────────────────────────────┤
│  Worker threads (default = # of CPU cores)  │
└─────────────────────────────────────────────┘
```

### 1. Tasks

`tokio::spawn(fut)` boxes the future into a `Task` and schedules it. The task's `Waker` points back at its slot in the scheduler, so `wake()` re-queues just that task.

### 2. Work-Stealing Scheduler

- Each worker thread has a **local run queue** (LIFO for cache locality).
- When a worker runs out of work, it **steals** from other workers' queues (FIFO from the victim's perspective).
- This keeps tasks hot on the thread that spawned them while balancing load.

### 3. I/O Driver

- Built on `mio` (epoll on Linux, kqueue on macOS, IOCP on Windows).
- `TcpStream`, `File`, etc. register their fd with the reactor; when the OS says "readable", the reactor wakes the associated task's waker.
- Non-blocking: all I/O ops return `WouldBlock` → the future returns `Pending` and registers interest.

### 4. Timer Driver

- A hierarchical timing wheel. `tokio::time::sleep` registers a deadline; the timer driver wakes the task when it expires.

### 5. Blocking Pool

`tokio::task::spawn_blocking` runs closures on a separate thread pool for blocking work (file I/O, crypto), so worker threads are never blocked.

---

## The Runtime Lifecycle

1. `#[tokio::main]` creates a `Runtime` (builds drivers + spawns workers).
2. `block_on(main_future)` runs your top-level future on the *calling* thread.
3. `spawn`ed tasks go to the scheduler; workers poll them as they're woken.
4. On shutdown, drivers are dropped and workers are joined.

---

## Key Types

| Type | Role |
|---|---|
| `tokio::runtime::Runtime` | the executor + drivers |
| `JoinHandle<T>` | handle to a spawned task; `await` it for the result |
| `tokio::sync::mpsc/oneshot/watch` | channels for task communication |
| `tokio::sync::Mutex/RwLock/Semaphore` | **async-aware** sync primitives — safe across `.await` |
| `tokio::select!` | race multiple futures; losers are dropped (cancelled) |
| `tokio::time::{sleep, timeout, interval}` | timer-based combinators |
| `tokio::io::{AsyncRead, AsyncWrite}` | async I/O traits |

---

## Critical Rules

1. **Never block a worker thread.** Use `spawn_blocking` for blocking calls. A blocked worker stalls all tasks on that thread.
2. **Don't hold `std::sync::Mutex` across `.await`** — use `tokio::sync::Mutex` (its guard is `Send` and it's fair/async-aware). Holding a std Mutex across await can deadlock the runtime.
3. **`select!` cancels losers** — dropping a future mid-await is normal and safe.
4. **`spawn` requires `Send + 'static`** — the task may move to another thread.

```rust
// ✅ async-aware mutex
let lock = tokio::sync::Mutex::new(0);
{
    let mut g = lock.lock().await;
    *g += 1;
} // guard dropped before any await

// ❌ std mutex across await — can deadlock
let lock = std::sync::Mutex::new(0);
let g = lock.lock().unwrap();
foo().await;   // task may migrate threads while holding a non-Send guard
```

---

## current_thread vs multi_thread

- **`current_thread`**: all tasks on one thread; requires everything to be `!Send`-friendly; cheapest, good for embedded-ish servers pinned to a core.
- **`multi_thread`** (default): work-stealing across N workers; tasks must be `Send`.

```rust
// Build manually for control:
let rt = tokio::runtime::Builder::new_multi_thread()
    .worker_threads(4)
    .enable_all()          // I/O + time drivers
    .build()
    .unwrap();
rt.block_on(async { ... });
```

---

## Mental Model

Tokio is: **a thread pool + event loop (reactor) + timer wheel + task scheduler**, hidden behind `async`/`await`. `spawn` = schedule; `await` = yield until woken; `select!` = race with cancellation.

---

## Exercises

- `exercises/03-tokio-patterns.rs` — the patterns (spawn/join, channels, timeout, select) demonstrated with std equivalents you can run anywhere
