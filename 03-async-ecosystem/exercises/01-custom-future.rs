/// ============================================================================
/// CUSTOM FUTURE EXERCISES: Build Futures by Hand
/// ============================================================================
///
/// Everything here uses only std — no tokio needed. We build:
/// 1. A thread-unparking Waker (the heart of any executor)
/// 2. A minimal block_on executor
/// 3. A Ready<T> future (immediately complete)
/// 4. A Delay future (completes after a duration, wakes properly)
///
/// Run: rustc --edition 2021 01-custom-future.rs -o cf && ./cf

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use std::thread;
use std::time::{Duration, Instant};

// ============================================================================
// PART 1: A Waker that unparks the current thread
// ============================================================================

struct ThreadWaker(thread::Thread);

fn thread_waker(t: thread::Thread) -> Waker {
    let arc = Arc::new(ThreadWaker(t));
    let raw = RawWaker::new(Arc::into_raw(arc) as *const (), &VTABLE);
    unsafe { Waker::from_raw(raw) }
}

unsafe fn waker_clone(data: *const ()) -> RawWaker {
    let arc = Arc::from_raw(data as *const ThreadWaker);
    let cloned = arc.clone();
    std::mem::forget(arc);
    RawWaker::new(Arc::into_raw(cloned) as *const (), &VTABLE)
}

unsafe fn waker_wake(data: *const ()) {
    let arc = Arc::from_raw(data as *const ThreadWaker);
    arc.0.unpark();
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

// ============================================================================
// PART 2: A minimal block_on executor
// ============================================================================

/// Poll the future until Ready, parking the thread between polls.
pub fn block_on<F: Future>(fut: F) -> F::Output {
    let waker = thread_waker(thread::current());
    let mut cx = Context::from_waker(&waker);
    // Safe: we never move `fut` after pinning it here.
    let mut fut = Box::pin(fut);
    loop {
        match fut.as_mut().poll(&mut cx) {
            Poll::Ready(v) => return v,
            Poll::Pending => thread::park(),
        }
    }
}

// ============================================================================
// PART 3: Ready<T> — a future that is immediately complete
// ============================================================================

pub struct Ready<T> {
    value: Option<T>,
}

impl<T> Ready<T> {
    pub fn new(value: T) -> Self {
        Ready { value: Some(value) }
    }
}

// Ready holds no self-references, so it is always safe to move.
impl<T> Unpin for Ready<T> {}

impl<T> Future for Ready<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<T> {
        Poll::Ready(self.get_mut().value.take().expect("Ready polled after completion"))
    }
}

// ============================================================================
// PART 4: Delay — a future that completes after a duration
// ============================================================================

/// Like tokio::time::sleep. On first poll it spawns a thread that sleeps,
/// then wakes us. (Production timers use a timing wheel instead of a
/// thread per timer — noted here for efficiency.)
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
        // Not ready: arrange a wake when the time elapses.
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
// PART 5: and_then — chaining futures by hand (what .await desugars to)
// ============================================================================

pub struct AndThen<Fut, F> {
    state: AndThenState<Fut, F>,
}

enum AndThenState<Fut, F> {
    // Box-pin the inner future once: it is never moved again afterwards.
    First(Pin<Box<Fut>>, F),
    Done,
}

pub trait FutureExt: Future + Sized {
    fn and_then<F, Fut2>(self, f: F) -> AndThen<Self, F>
    where
        F: FnOnce(Self::Output) -> Fut2,
    {
        AndThen {
            state: AndThenState::First(Box::pin(self), f),
        }
    }
}

impl<F: Future> FutureExt for F {}

// The inner future is boxed (Pin<Box<..>> is Unpin), so AndThen itself never
// moves a pinned value: it is safe to be Unpin.
impl<Fut, F> Unpin for AndThen<Fut, F> {}

impl<Fut, F, Fut2> Future for AndThen<Fut, F>
where
    Fut: Future,
    F: FnOnce(Fut::Output) -> Fut2,
    Fut2: Future,
{
    type Output = Fut2::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        let state = std::mem::replace(&mut this.state, AndThenState::Done);
        match state {
            AndThenState::First(mut fut, f) => match fut.as_mut().poll(cx) {
                Poll::Ready(v) => {
                    let next = f(v);
                    Box::pin(next).as_mut().poll(cx)
                }
                Poll::Pending => {
                    this.state = AndThenState::First(fut, f);
                    Poll::Pending
                }
            },
            AndThenState::Done => panic!("AndThen polled after completion"),
        }
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("=== CUSTOM FUTURE EXERCISES ===\n");

    let v = block_on(Ready::new(42));
    println!("Ready -> {}", v);

    let t0 = Instant::now();
    block_on(Delay::new(Duration::from_millis(50)));
    println!("Delay 50ms elapsed: {:?}", t0.elapsed());

    let result = block_on(
        Ready::new(10).and_then(|x| Ready::new(x * 2)),
    );
    println!("and_then chain -> {}", result);

    let result = block_on(
        Delay::new(Duration::from_millis(10))
            .and_then(|_| Ready::new("after the delay")),
    );
    println!("delay then value -> {}", result);

    println!("\n=== ALL CUSTOM FUTURE EXERCISES COMPLETE ===");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ready() {
        assert_eq!(block_on(Ready::new(7)), 7);
    }

    #[test]
    fn test_delay_completes() {
        let t0 = Instant::now();
        block_on(Delay::new(Duration::from_millis(10)));
        assert!(t0.elapsed() >= Duration::from_millis(10));
    }

    #[test]
    fn test_and_then() {
        assert_eq!(block_on(Ready::new(5).and_then(|x| Ready::new(x + 1))), 6);
    }

    #[test]
    fn test_block_on_returns_output() {
        fn async_like() -> Ready<&'static str> {
            Ready::new("done")
        }
        let out = block_on(async_like());
        assert_eq!(out, "done");
    }
}
