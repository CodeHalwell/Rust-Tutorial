# The Waker Mechanism: How Async Wakeup Works

## The Problem: Busy-Waiting Is Wasteful

A naive executor could poll a future in a loop:

```rust
// ❌ Burns CPU, blocks the thread
loop {
    match future.poll(&mut cx) {
        Poll::Ready(v) => break v,
        Poll::Pending => continue,  // spin!
    }
}
```

Polling a future that isn't ready wastes CPU. Instead, the future should tell the executor **when to come back**.

---

## The Solution: Waker

Every `poll` receives a `Context<'a>` which contains a `Waker`:

```rust
pub struct Context<'a> {
    waker: &'a Waker,
    // ...
}

pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

**The contract:**
1. If a future returns `Poll::Pending`, it must arrange for `cx.waker().wake()` (or `wake_by_ref()`) to be called when it can make progress.
2. The executor only re-polls after being woken.

**Key insight:** `poll` returning `Pending` is a *promise*: "I will wake you when I'm ready." Breaking that promise hangs the task forever.

---

## Building a Waker by Hand

A `Waker` is constructed from a `RawWaker` + `RawWakerVTable` — essentially a vtable of function pointers:

```rust
use std::task::{RawWaker, RawWakerVTable, Waker};
use std::sync::Arc;

struct ThreadWaker(std::thread::Thread);

fn make_waker(thread: std::thread::Thread) -> Waker {
    let arc = Arc::new(ThreadWaker(thread));
    let raw = RawWaker::new(Arc::into_raw(arc) as *const (), &VTABLE);
    unsafe { Waker::from_raw(raw) }
}

unsafe fn clone(data: *const ()) -> RawWaker {
    let arc = Arc::from_raw(data as *const ThreadWaker);
    let cloned = arc.clone();
    std::mem::forget(arc); // don't drop the original
    RawWaker::new(Arc::into_raw(cloned) as *const (), &VTABLE)
}

unsafe fn wake(data: *const ()) {
    let arc = Arc::from_raw(data as *const ThreadWaker);
    arc.0.unpark(); // wake the parked thread
}

unsafe fn wake_by_ref(data: *const ()) {
    let arc = Arc::from_raw(data as *const ThreadWaker);
    arc.0.unpark();
    std::mem::forget(arc);
}

unsafe fn drop_waker(data: *const ()) {
    drop(Arc::from_raw(data as *const ThreadWaker));
}

static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop_waker);
```

---

## A Minimal Executor Using Wakers

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};

// The executor receives "wake" signals over a channel.
struct Executor {
    task: Arc<Mutex<Option<Pin<Box<dyn Future<Output = ()>>>>>>,
    signal: Sender<()>,
}

impl Executor {
    fn run(&self, waker: Waker) {
        let mut cx = Context::from_waker(&waker);
        loop {
            // Wait for a wake signal
            self.signal.recv().ok();
            let mut guard = self.task.lock().unwrap();
            if let Some(fut) = guard.as_mut() {
                match fut.as_mut().poll(&mut cx) {
                    Poll::Ready(()) => break,
                    Poll::Pending => {}
                }
            }
        }
    }
}
```

The pattern: **wake → poll → pending → sleep → wake → poll → ... → ready**.

---

## Waker Semantics

| Method | Effect |
|---|---|
| `wake()` | consumes the waker; executor schedules a poll |
| `wake_by_ref()` | keeps the waker; same effect, can be called again |
| `will_wake(&other)` | true if both wake the same task — lets you avoid cloning |

Rules for future implementors:
- Always wake when transitioning to "can make progress".
- Waking spuriously (extra wakes) is allowed but wasteful.
- Never waking after `Pending` is a bug (task stalls forever).

---

## Real Runtimes

- **Tokio**: a `Waker` wakes a specific task in the work-stealing scheduler; the task is pushed onto a run queue.
- **embassy** (embedded): wakers point to the executor's task slot directly — no allocation.
- **smol/futures**: channel/arc-based like the example above.

**Mental model:** The `Waker` is a callback handle. "Pending" means "call this callback when you have news."

---

## Exercises

- `exercises/01-custom-future.rs` — build a `Future` that wakes properly
- `exercises/02-manual-state-machine.rs` — see what `async` desugars to
- `exercises/03-tokio-patterns.rs` — channels, timeouts, cancellation patterns
