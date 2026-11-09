/// ============================================================================
/// MANUAL STATE MACHINE: What `async fn` Desugars To
/// ============================================================================
///
/// This file hand-writes the enum the compiler generates for:
///
/// ```rust
/// async fn wait_twice() -> u32 {
///     delay(20ms).await;
///     delay(30ms).await;
///     2
/// }
/// ```
///
/// Run: rustc --edition 2021 02-manual-state-machine.rs -o sm && ./sm

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

// ---- minimal waker + executor (same idea as 01-custom-future.rs) ----

struct ThreadWaker(thread::Thread);

fn thread_waker(t: thread::Thread) -> Waker {
    let arc = Arc::new(ThreadWaker(t));
    unsafe { Waker::from_raw(RawWaker::new(Arc::into_raw(arc) as *const (), &VTABLE)) }
}

unsafe fn waker_clone(data: *const ()) -> RawWaker {
    let arc = Arc::from_raw(data as *const ThreadWaker);
    let c = arc.clone();
    std::mem::forget(arc);
    RawWaker::new(Arc::into_raw(c) as *const (), &VTABLE)
}
unsafe fn waker_wake(data: *const ()) {
    Arc::from_raw(data as *const ThreadWaker).0.unpark();
}
unsafe fn waker_wake_by_ref(data: *const ()) {
    let arc = Arc::from_raw(data as *const ThreadWaker);
    arc.0.unpark();
    std::mem::forget(arc);
}
unsafe fn waker_drop(data: *const ()) {
    drop(Arc::from_raw(data as *const ThreadWaker));
}
static VTABLE: RawWakerVTable =
    RawWakerVTable::new(waker_clone, waker_wake, waker_wake_by_ref, waker_drop);

pub fn block_on<F: Future>(fut: F) -> F::Output {
    let waker = thread_waker(thread::current());
    let mut cx = Context::from_waker(&waker);
    let mut fut = Box::pin(fut);
    loop {
        match fut.as_mut().poll(&mut cx) {
            Poll::Ready(v) => return v,
            Poll::Pending => thread::park(),
        }
    }
}

// ---- a Delay future to await on ----

pub struct Delay {
    dur: Duration,
    start: Option<Instant>,
}

impl Delay {
    pub fn new(dur: Duration) -> Self {
        Delay { dur, start: None }
    }
}

impl Future for Delay {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let this = &mut *self;
        let start = this.start.get_or_insert_with(Instant::now);
        if start.elapsed() >= this.dur {
            return Poll::Ready(());
        }
        let waker = cx.waker().clone();
        let remaining = this.dur - start.elapsed();
        thread::spawn(move || {
            thread::sleep(remaining);
            waker.wake();
        });
        Poll::Pending
    }
}

// ============================================================================
// THE HAND-WRITTEN STATE MACHINE
// ============================================================================
// Equivalent of:
//   async fn wait_twice() -> u32 {
//       delay(20ms).await;
//       delay(30ms).await;
//       2
//   }

pub enum WaitTwice {
    Start,               // before first await
    AwaitingFirst(Delay),  // suspended at first .await — holds the inner future
    AwaitingSecond(Delay), // suspended at second .await
    Done,
}

impl WaitTwice {
    pub fn new() -> Self {
        WaitTwice::Start
    }
}

impl Future for WaitTwice {
    type Output = u32;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<u32> {
        // WaitTwice is Unpin (no self-references), so we can get &mut.
        let this = self.get_mut();
        loop {
            // Take the state out so we can poll/move inner futures freely.
            let state = std::mem::replace(this, WaitTwice::Done);
            match state {
                WaitTwice::Start => {
                    *this = WaitTwice::AwaitingFirst(Delay::new(Duration::from_millis(20)));
                }
                WaitTwice::AwaitingFirst(mut d) => match Pin::new(&mut d).poll(cx) {
                    Poll::Ready(()) => {
                        *this = WaitTwice::AwaitingSecond(Delay::new(Duration::from_millis(30)));
                    }
                    Poll::Pending => {
                        *this = WaitTwice::AwaitingFirst(d);
                        return Poll::Pending;
                    }
                },
                WaitTwice::AwaitingSecond(mut d) => match Pin::new(&mut d).poll(cx) {
                    Poll::Ready(()) => {
                        *this = WaitTwice::Done;
                        return Poll::Ready(2); // the value after the last await
                    }
                    Poll::Pending => {
                        *this = WaitTwice::AwaitingSecond(d);
                        return Poll::Pending;
                    }
                },
                WaitTwice::Done => panic!("WaitTwice polled after completion"),
            }
        }
    }
}

// ============================================================================
// A SELF-REFERENTIAL FUTURE (why Pin exists)
// ============================================================================

/// Like the compiler-generated future for:
///   async fn self_ref() { let x = 5; let r = &x; delay().await; use(r); }
/// `r` points into the same struct that owns `x` — moving the struct breaks `r`.
/// Pin promises "this value will not move", making it safe to hold such a
/// self-reference.
pub struct SelfRef {
    x: i32,
    r: Option<*const i32>, // raw self-pointer, pinned by contract
}

impl SelfRef {
    pub fn new() -> Self {
        SelfRef { x: 5, r: None }
    }
}

impl Future for SelfRef {
    type Output = i32;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<i32> {
        // First poll: create the self-reference. Safe because we're pinned.
        if self.r.is_none() {
            let this = unsafe { self.as_mut().get_unchecked_mut() };
            this.r = Some(&this.x as *const i32);
            let waker = cx.waker().clone();
            thread::spawn(move || {
                thread::sleep(Duration::from_millis(10));
                waker.wake();
            });
            return Poll::Pending;
        }
        // Second poll: read through the self-pointer.
        let this = unsafe { self.get_unchecked_mut() };
        let r = this.r.unwrap();
        let val = unsafe { *r };
        Poll::Ready(val)
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("=== MANUAL STATE MACHINE ===\n");

    let t0 = Instant::now();
    let n = block_on(WaitTwice::new());
    println!("wait_twice returned {} after {:?}", n, t0.elapsed());

    let v = block_on(SelfRef::new());
    println!("self-referential future read x = {}", v);

    println!("\n=== STATE MACHINE EXERCISES COMPLETE ===");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wait_twice() {
        let t0 = Instant::now();
        let n = block_on(WaitTwice::new());
        assert_eq!(n, 2);
        assert!(t0.elapsed() >= Duration::from_millis(50));
    }

    #[test]
    fn test_self_ref() {
        assert_eq!(block_on(SelfRef::new()), 5);
    }
}
