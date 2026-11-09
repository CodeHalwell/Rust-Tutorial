/// ============================================================================
/// ASYNC PATTERNS (a.k.a. "tokio patterns", std edition)
/// ============================================================================
///
/// These are the core tokio patterns, implemented with std so the file runs
/// anywhere with plain rustc. Each section shows the tokio equivalent in
/// comments — the concepts map 1:1:
///
///   tokio::spawn      <-> thread::spawn + JoinHandle
///   tokio::sync::mpsc <-> std::sync::mpsc
///   tokio::time::timeout <-> Receiver::recv_timeout
///   tokio::select!    <-> racing two receivers with try_recv
///   drop(future)      <-> dropping a JoinHandle / receiver = cancellation
///
/// Run: rustc --edition 2021 03-tokio-patterns.rs -o tp && ./tp

use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;

// ============================================================================
// PATTERN 1: spawn + join
// ============================================================================

/// tokio:   let h = tokio::spawn(async { 40 + 2 }); let v = h.await?;
pub fn pattern_spawn_and_join() -> i32 {
    let handle = thread::spawn(|| 40 + 2); // tokio::spawn(async { ... })
    handle.join().expect("task panicked")   // h.await.unwrap()
}

// ============================================================================
// PATTERN 2: mpsc channel (multi-producer, single-consumer)
// ============================================================================

/// tokio:   let (tx, mut rx) = tokio::sync::mpsc::channel(32);
///          tx.send(v).await;  let v = rx.recv().await;
pub fn pattern_mpsc_channel() -> Vec<i32> {
    let (tx, rx): (Sender<i32>, Receiver<i32>) = channel();
    for i in 0..3 {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(i * 10).unwrap();
        });
    }
    drop(tx); // close the channel so rx.recv() returns None at the end
    let mut out: Vec<i32> = rx.iter().collect();
    out.sort();
    out
}

// ============================================================================
// PATTERN 3: oneshot channel (single value, e.g. for task results)
// ============================================================================

/// tokio:   let (tx, rx) = tokio::sync::oneshot::channel();
///          tx.send(result);  let r = rx.await;
pub fn pattern_oneshot() -> String {
    let (tx, rx) = channel::<String>();
    thread::spawn(move || {
        tx.send(String::from("the answer")).unwrap();
    });
    rx.recv().unwrap()
}

// ============================================================================
// PATTERN 4: timeout
// ============================================================================

/// tokio:   tokio::time::timeout(Duration::from_millis(50), rx.recv()).await
pub fn pattern_timeout() -> Result<String, &'static str> {
    let (tx, rx) = channel::<String>();
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(200));
        let _ = tx.send(String::from("too late"));
    });
    match rx.recv_timeout(Duration::from_millis(50)) {
        Ok(v) => Ok(v),
        Err(_) => Err("timed out"),
    }
}

// ============================================================================
// PATTERN 5: select — race two operations, cancel the loser
// ============================================================================

/// tokio:
///   tokio::select! {
///       v = rx1.recv() => ...,
///       v = rx2.recv() => ...,
///   }
/// The losing branch's future is DROPPED (cancellation).
pub fn pattern_select() -> &'static str {
    let (tx1, rx1) = channel::<&'static str>();
    let (tx2, rx2) = channel::<&'static str>();

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(10));
        let _ = tx1.send("fast");
    });
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(500));
        let _ = tx2.send("slow");
    });

    loop {
        if let Ok(v) = rx1.try_recv() {
            return v; // rx2's sender gets dropped when the thread ends => cancelled
        }
        if let Ok(v) = rx2.try_recv() {
            return v;
        }
        thread::sleep(Duration::from_millis(1));
    }
}

// ============================================================================
// PATTERN 6: cancellation by dropping
// ============================================================================

/// In async Rust, dropping a future cancels it. Dropping a JoinHandle
/// detaches (the task keeps running); aborting cancels. Here we simply
/// drop the receiver — the sender's send() then fails, which is how a
/// cancelled task observes cancellation.
pub fn pattern_cancellation() -> bool {
    let (tx, rx) = channel::<()>();
    drop(rx); // "task cancelled"
    tx.send(()).is_err() // send fails: nobody is listening anymore
}

// ============================================================================
// PATTERN 7: fan-out / fan-in (map-reduce over workers)
// ============================================================================

/// tokio: spawn N tasks, collect results via JoinSet or a channel.
pub fn pattern_fan_out(input: Vec<i32>) -> i32 {
    let (tx, rx) = channel::<i32>();
    for chunk in input.chunks(2) {
        let tx = tx.clone();
        let chunk = chunk.to_vec();
        thread::spawn(move || {
            tx.send(chunk.iter().sum::<i32>()).unwrap();
        });
    }
    drop(tx);
    rx.iter().sum()
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("=== ASYNC PATTERNS (std edition) ===\n");

    println!("spawn+join: {}", pattern_spawn_and_join());
    println!("mpsc: {:?}", pattern_mpsc_channel());
    println!("oneshot: {}", pattern_oneshot());
    println!("timeout: {:?}", pattern_timeout());
    println!("select winner: {}", pattern_select());
    println!("cancellation observed: {}", pattern_cancellation());
    println!("fan-out sum: {}", pattern_fan_out(vec![1, 2, 3, 4, 5]));

    println!("\n=== ALL ASYNC PATTERN EXERCISES COMPLETE ===");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spawn_and_join() {
        assert_eq!(pattern_spawn_and_join(), 42);
    }

    #[test]
    fn test_mpsc_channel() {
        assert_eq!(pattern_mpsc_channel(), vec![0, 10, 20]);
    }

    #[test]
    fn test_oneshot() {
        assert_eq!(pattern_oneshot(), "the answer");
    }

    #[test]
    fn test_timeout() {
        assert_eq!(pattern_timeout(), Err("timed out"));
    }

    #[test]
    fn test_select_picks_fast() {
        assert_eq!(pattern_select(), "fast");
    }

    #[test]
    fn test_cancellation() {
        assert!(pattern_cancellation());
    }

    #[test]
    fn test_fan_out() {
        assert_eq!(pattern_fan_out(vec![1, 2, 3, 4, 5]), 15);
    }
}
