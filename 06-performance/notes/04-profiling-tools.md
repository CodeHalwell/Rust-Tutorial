# Profiling Tools

> **"Profiling beats guessing."** Never optimize without data.

## The Profiling Workflow

1. Build with debug info in release mode: `cargo build --release` with `[profile.release] debug = true`.
2. Run under a profiler to find hot spots.
3. Hypothesize, change ONE thing, re-measure.
4. Verify with benchmarks (criterion).

---

## perf (Linux)

```bash
# Record:
perf record -g --call-graph dwarf ./target/release/myapp
# Report:
perf report
# Or script-friendly:
perf script | stackcollapse-perf.pl | flamegraph.pl > flame.svg
```

Useful events: `perf stat -e cycles,instructions,cache-misses,LLC-load-misses ./app`.

---

## cargo-flamegraph

```bash
cargo install flamegraph
cargo flamegraph --bin myapp -- arg1 arg2
# produces flamegraph.svg — open in a browser
```

Great for seeing *which functions* burn CPU and *why* (call stacks).

---

## cargo-show-asm / cargo-asm

See the actual assembly your code compiles to:

```bash
cargo install cargo-show-asm
cargo asm my_module::my_function --rust
```

Check: is the loop vectorized? Was the abstraction inlined? Any unexpected calls?

---

## Valgrind / Callgrind + DHAT

```bash
# CPU profiling:
valgrind --tool=callgrind ./target/release/myapp
callgrind_annotate callgrind.out.*

# Heap profiling (allocations!):
valgrind --tool=dhat ./target/release/myapp
```

DHAT is invaluable for finding allocation hot spots.

---

## heaptrack / heappoint (Linux)

```bash
heaptrack ./target/release/myapp    # tracks all allocations with stacks
heaptrack --analyze
```

---

## Tracing (structured, low-overhead)

For async and distributed systems, use the `tracing` crate:

```rust
use tracing::{info_span, instrument};

#[instrument]
async fn handle_request(id: u64) {
    let span = info_span!("db_query");
    // ... work ...
}
```

View with `tokio-console` (for tokio) or export to Jaeger/Zipkin.

---

## eBPF (advanced, Linux)

`bpftrace` for one-liners:

```bash
# Count syscalls by process:
bpftrace -e 'tracepoint:syscalls:sys_enter_* { @[comm] = count(); }'
# Trace Rust function calls:
bpftrace -e 'uprobe:./target/release/myapp:my_function { printf("hit\n"); }'
```

---

## macOS: Instruments (Xcode)

Time Profiler + Allocations instruments give the same data as perf/heaptrack on macOS.

---

## Rules

1. Profile the **release** build (with `debug = true` for symbols), not debug.
2. Profile **realistic workloads**, not toy inputs.
3. Look at *cumulative* time (inclusive), then drill into *self* time.
4. Cache misses (`perf stat -e LLC-load-misses`) often matter more than instruction counts.

---

## Exercises

- `benchmarks/vector-iteration.rs`, `benchmarks/allocation-patterns.rs` — measure before/after
