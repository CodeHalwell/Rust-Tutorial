# Async/Await Desugaring: State Machines by Hand

## What `async fn` Actually Compiles To

```rust
async fn fetch_and_parse() -> String {
    let data = fetch().await;
    parse(data).await
}
```

desugars roughly to:

```rust
enum FetchAndParseFuture {
    Start,
    WaitingOnFetch(fetch::FetchFuture, String),  // holds the inner future
    WaitingOnParse(parse::ParseFuture),
    Done,
}

impl Future for FetchAndParseFuture {
    type Output = String;
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<String> {
        // match on the state, poll inner futures, transition states
    }
}
```

**Key insight:** Each `.await` point is a *state* in an enum. The future stores whatever locals are live across the await. That's why the generated type can be self-referential — and why `Pin` exists.

---

## A Hand-Written State Machine

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

// async fn countdown(n: u32) -> u32 { while n > 0 { yield_once().await; n -= 1; } n }
enum Countdown {
    Running { n: u32, yielded: bool },
    Done,
}

impl Future for Countdown {
    type Output = u32;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<u32> {
        match &mut *self {
            Countdown::Running { n, yielded } => {
                if *yielded {
                    *n -= 1;
                    *yielded = false;
                    if *n == 0 {
                        self.set(Countdown::Done);
                        Poll::Ready(0)
                    } else {
                        cx.waker().wake_by_ref(); // "poll me again immediately"
                        Poll::Pending
                    }
                } else {
                    *yielded = true;
                    cx.waker().wake_by_ref();
                    Poll::Pending
                }
            }
            Countdown::Done => panic!("polled after completion"),
        }
    }
}
```

---

## Why the Generated Type Is Self-Referential

```rust
async fn example() {
    let x = 5;
    let r = &x;          // r borrows x
    foo().await;         // across the await, BOTH x and r must be stored
    println!("{}", r);   // r is used after the await — it points into the future!
}
```

The future struct contains `x: i32` and `r: &i32` where `r` points at `x` *inside the same struct*. Moving the future would move `x` and invalidate `r`. **That's exactly the self-referential struct problem `Pin` solves.**

---

## Reading Desugared Code

You can see the real state machine with:

```bash
cargo rustc -- -Zunpretty=expanded        # nightly: macro expansion
cargo expand                             # cargo-expand crate
RUSTFLAGS="-Zdump-mir" cargo build       # nightly: MIR dump
```

Or reason about it yourself: every `await` splits the function body into states; locals live across awaits become struct fields.

---

## Cancellation: Dropping a Future

A future is **cancelled** simply by dropping it. There is no cancellation API — `Drop` of the future runs, dropping any in-flight inner futures and locals.

```rust
let fut = fetch_something();
tokio::select! {
    _ = fut => {},        // if this branch loses, `fut` is dropped mid-poll-state
    _ = timeout() => {},
}
```

**Consequence:** Code after an `.await` may never run. Don't hold locks across `.await` unless the lock is async-aware (`tokio::sync::Mutex`), and don't assume cleanup happens after await points.

---

## State Machine Size

The generated future's size = sum of all locals live across awaits, at their largest. You can inspect sizes:

```rust
async fn big() {
    let buf = [0u8; 1024];
    something().await;
    println!("{}", buf[0]);
}

fn main() {
    println!("future size: {} bytes", std::mem::size_of_val(&big()));
}
```

**Key insight:** If a huge buffer is live across an `.await`, it bloats the future — `Box` it if needed.

---

## Summary

| Concept | Meaning |
|---|---|
| `async fn` | sugar for a struct implementing `Future` |
| Each `.await` | a state variant in the enum |
| Locals across `.await` | fields of the future struct |
| Self-referential futures | why `Pin` is required |
| Drop of the future | cancellation |

---

## Exercises

- `exercises/02-manual-state-machine.rs` — write the enum yourself
