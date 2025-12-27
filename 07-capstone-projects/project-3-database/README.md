# Project 3: In-Memory Database Engine (Advanced)

## Overview

Build a production-quality key-value database engine with persistence, caching, and concurrent access. This is the capstone project integrating all previous modules: ownership, traits, async (optional), unsafe code, performance, and systems design.

## Learning Objectives

- ✅ Complex data structures (B-trees, hash tables, LRU caches)
- ✅ Serialization and deserialization formats
- ✅ Concurrent access patterns (Mutex, RwLock, Arc)
- ✅ Performance optimization and benchmarking
- ✅ Persistence and recovery
- ✅ Transaction semantics
- ✅ Production-quality API design
- ✅ Unsafe code for custom memory management (advanced)

## Requirements

### Core Engine

#### 1. Key-Value Store

Implement a key-value storage engine supporting:

```rust
pub trait KVStore: Send + Sync {
    /// Get a value by key
    fn get(&self, key: &str) -> Result<Option<Vec<u8>>>;

    /// Set a key-value pair
    fn put(&self, key: &str, value: Vec<u8>) -> Result<()>;

    /// Delete a key
    fn delete(&self, key: &str) -> Result<()>;

    /// Check if key exists
    fn contains(&self, key: &str) -> Result<bool>;

    /// Get all keys matching prefix
    fn scan(&self, prefix: &str) -> Result<Vec<(String, Vec<u8>)>>;

    /// Get database statistics
    fn stats(&self) -> DatabaseStats;
}

pub struct DatabaseStats {
    pub total_keys: usize,
    pub total_size_bytes: usize,
    pub cache_hits: usize,
    pub cache_misses: usize,
}
```

#### 2. Storage Engine (Hash-based or B-tree)

Choose one approach:

**Hash-based (simpler)**:
- HashMap<String, Vec<u8>> for in-memory storage
- O(1) average lookup
- Not suitable for range queries
- Simplest implementation

**B-tree (more advanced)**:
- Sorted key storage
- O(log n) operations
- Supports range queries efficiently
- More complex but production-ready

#### 3. LRU Cache Layer

Implement a Least-Recently-Used cache to avoid repeated disk lookups:

```rust
pub struct LRUCache {
    capacity: usize,
    cache: Mutex<LinkedHashMap<String, Vec<u8>>>,
}

impl LRUCache {
    pub fn new(capacity: usize) -> Self;
    pub fn get(&self, key: &str) -> Option<Vec<u8>>;
    pub fn put(&self, key: String, value: Vec<u8>);
    pub fn evict_oldest(&self) -> Option<(String, Vec<u8>)>;
}
```

#### 4. Persistence

Implement an append-only log for crash recovery:

```
[Header: Version | Timestamp]
[Entry 0: Op | KeyLen | Key | ValueLen | Value | Checksum]
[Entry 1: Op | KeyLen | Key | ValueLen | Value | Checksum]
...

Op codes:
  0x01 = PUT
  0x02 = DELETE
  0x03 = COMPACT
```

**Features**:
- Write operations to log before applying to in-memory store
- Recover from crash by replaying log
- Periodic compaction to remove deleted entries

#### 5. Transactions (ACID Lite)

Implement basic transaction support:

```rust
pub struct Transaction {
    id: u64,
    writes: Vec<(String, Option<Vec<u8>>)>,  // Key -> Value or None (delete)
    committed: AtomicBool,
}

impl Transaction {
    pub fn begin() -> Self;
    pub fn put(&mut self, key: String, value: Vec<u8>);
    pub fn delete(&mut self, key: &str);
    pub fn commit(self) -> Result<()>;
    pub fn rollback(self);
}
```

**Guarantees**:
- Atomicity: All writes in transaction succeed or all fail
- Isolation: Transactions don't see uncommitted writes
- Durability: Committed transactions survive crashes
- (Consistency is application-dependent)

#### 6. Thread Safety

All operations must be thread-safe:

```rust
pub struct Database {
    storage: Arc<RwLock<HashMap<String, Vec<u8>>>>,
    cache: Arc<LRUCache>,
    log: Arc<Mutex<PersistenceLog>>,
}

impl Database {
    pub fn new(path: PathBuf) -> Result<Self>;
    pub fn get(&self, key: &str) -> Result<Option<Vec<u8>>>;
    pub fn put(&self, key: String, value: Vec<u8>) -> Result<()>;
}
```

Use `RwLock` for mostly-read workloads, `Mutex` for balanced workloads.

### Optional Advanced Features

1. **Compression**
   - Compress values with gzip or zstd
   - Reduce disk footprint
   - Trade CPU for space

2. **Snapshots**
   - Create point-in-time snapshots
   - Incremental backup
   - Time-travel queries

3. **Indexing**
   - Create secondary indexes on prefixes
   - Speed up range queries
   - Maintain at commit time

4. **Memory-mapped Files**
   - Use `mmap` for large values
   - Avoid loading entire file into memory
   - Dangerous (requires careful unsafe code)

5. **Async API**
   - tokio-based async/await interface
   - `async fn get()` instead of blocking
   - More complex but better for high-concurrency scenarios

## Implementation Guide

### Phase 1: Basic In-Memory Store

1. Implement simple HashMap-based storage
2. Add get/put/delete operations
3. Write tests for basic functionality
4. Measure performance with criterion

```rust
pub struct SimpleStore {
    data: Mutex<HashMap<String, Vec<u8>>>,
}
```

### Phase 2: LRU Cache

1. Implement a LinkedHashMap-based LRU cache
2. Evict least-recently-used entry on overflow
3. Track cache hits/misses for stats
4. Integration test with SimpleStore

### Phase 3: Persistence

1. Design serialization format
2. Implement log writer that appends entries
3. Implement log reader for recovery
4. Test crash recovery by:
   - Write some entries
   - Kill process
   - Restart and verify entries are there

### Phase 4: Transactions

1. Implement transaction builder pattern
2. Queue writes without applying them
3. Commit atomically (all or nothing)
4. Rollback on error

### Phase 5: Advanced Features

Pick one or more:
- Compression
- Snapshots
- Indexing
- Memory-mapped storage
- Async API

## Code Architecture

```
src/
├── main.rs                    # CLI/REPL for database
├── lib.rs                     # Public API
├── db/
│   ├── mod.rs
│   ├── storage.rs             # Core KVStore trait and impls
│   ├── cache.rs               # LRU cache implementation
│   ├── transaction.rs         # Transaction support
│   └── stats.rs               # Statistics and metrics
├── persistence/
│   ├── mod.rs
│   ├── log.rs                 # Append-only log
│   ├── serialization.rs        # Format definition
│   └── recovery.rs            # Crash recovery
├── api/
│   ├── mod.rs
│   ├── sync_api.rs            # Synchronous API
│   └── async_api.rs           # Optional async API
├── error.rs                   # Error types
└── config.rs                  # Configuration
```

## Data Format (Example)

Design a clean binary format for persistence:

```
[Magic: 4 bytes "KVDB"]
[Version: 1 byte, 0x01]
[Created: 8 bytes timestamp]

For each entry:
[Opcode: 1 byte]
  0x01 = PUT
  0x02 = DELETE
  0x03 = COMPACT
[Key length: 4 bytes, big-endian]
[Key: variable bytes]
[Value length: 4 bytes, big-endian] (0 for DELETE)
[Value: variable bytes]
[CRC32: 4 bytes checksum]

Example (PUT "user:123" = {"name": "Alice"}):
[01][00 00 00 08][75 73 65 72 3a 31 32 33][00 00 00 1f][...json...][crc32]
```

## Testing Strategy

### Unit Tests

```rust
#[test]
fn test_put_get() {
    let db = SimpleStore::new();
    db.put("key".into(), b"value".to_vec()).unwrap();
    assert_eq!(db.get("key").unwrap(), Some(b"value".to_vec()));
}

#[test]
fn test_lru_eviction() {
    let cache = LRUCache::new(2);
    cache.put("a".into(), vec![1]);
    cache.put("b".into(), vec![2]);
    cache.put("c".into(), vec![3]);  // Evicts "a"
    assert!(cache.get("a").is_none());
}

#[test]
fn test_transaction_atomicity() {
    let db = Database::new(...).unwrap();
    let mut tx = Transaction::begin();
    tx.put("a".into(), vec![1]);
    tx.put("b".into(), vec![2]);
    tx.commit().unwrap();

    assert_eq!(db.get("a").unwrap(), Some(vec![1]));
    assert_eq!(db.get("b").unwrap(), Some(vec![2]));
}

#[test]
fn test_crash_recovery() {
    // Write entries to database
    let db = Database::new(path.clone()).unwrap();
    db.put("key".into(), vec![1, 2, 3]).unwrap();
    drop(db);  // Simulate crash

    // Reopen and verify
    let db = Database::new(path).unwrap();
    assert_eq!(db.get("key").unwrap(), Some(vec![1, 2, 3]));
}
```

### Integration Tests

```bash
# Concurrent readers
# Concurrent writers
# Mixed read/write workloads
# Large datasets (100MB+)
# Recovery after simulated crash
```

### Benchmarks

Measure with criterion:

```rust
criterion_group!(benches, bench_put, bench_get, bench_concurrent);

fn bench_put(c: &mut Criterion) {
    c.bench_function("put_10k", |b| {
        b.iter(|| {
            for i in 0..10000 {
                db.put(format!("key{}", i), vec![i as u8]).unwrap();
            }
        })
    });
}

fn bench_concurrent(c: &mut Criterion) {
    c.bench_function("concurrent_reads_1M", |b| {
        b.iter(|| {
            let db = Arc::new(database);
            let handles: Vec<_> = (0..4).map(|_| {
                let db = Arc::clone(&db);
                thread::spawn(move || {
                    for i in 0..250000 {
                        let _ = db.get(&format!("key{}", i));
                    }
                })
            }).collect();
            for h in handles { h.join().unwrap(); }
        })
    });
}
```

## Performance Targets

Aim for:

- **Single-threaded GET**: < 1 microsecond
- **Single-threaded PUT**: < 10 microseconds
- **4-thread concurrent GETs**: > 4M ops/sec
- **Mixed read/write**: > 1M ops/sec
- **Log compaction**: < 100ms for 1GB log
- **Cache hit rate**: > 90% under typical workload

## Persistence File Format Decisions

Trade-offs:

| Format | Pros | Cons |
|--------|------|------|
| **Binary** | Fast, compact | Hard to debug |
| **JSON** | Human-readable | Slow, verbose |
| **MessagePack** | Fast, compact, debuggable | Extra dependency |
| **Protobuf** | Schema versioning | Complex |

**Recommendation**: Start with simple binary format, add JSON marshaling for debugging.

## Safety Considerations

### Safe Rust Areas
- Serialization
- Transaction management
- API layer

### Unsafe Code Opportunities (Advanced)
- Custom allocator for key-value pairs
- Memory-mapped file access
- Lock-free data structures
- Pointer arithmetic for B-tree nodes

**Rule**: Only use unsafe if you benchmark and prove it's necessary.

## Configuration

Support configurable parameters:

```rust
pub struct Config {
    pub data_dir: PathBuf,
    pub max_cache_size: usize,
    pub max_log_size: usize,
    pub compression_enabled: bool,
    pub num_worker_threads: usize,
}

// Default values
cache_size: 64MB
log_size: 256MB before compaction
compression: disabled (use snappy)
```

## CLI Interface (Optional)

Implement a simple REPL:

```bash
$ cargo run
> SET user:123 {"name": "Alice"}
OK

> GET user:123
{"name": "Alice"}

> KEYS user:*
user:123
user:456

> STATS
Total keys: 2
Cache hits: 145
Cache misses: 23
Size: 2.4 MB

> BACKUP /path/to/backup
Backup complete

> SHUTDOWN
Goodbye!
```

## Advanced Concepts to Explore

1. **B-tree Implementation**: Balanced tree for range queries
2. **Bloom Filters**: Fast negative lookups
3. **Consistent Hashing**: For distributed scenarios
4. **Write-Ahead Logging**: Durability guarantees
5. **Copy-on-Write**: Efficient snapshots
6. **Multiversion Concurrency Control (MVCC)**: Concurrent transactions
7. **Compaction Strategies**: LSM (Log-Structured Merge) trees

## Completion Criteria

Your database is complete when:

1. ✅ Get/put/delete operations work correctly
2. ✅ LRU cache reduces disk accesses
3. ✅ Persistence survives crashes
4. ✅ Transactions are atomic
5. ✅ Thread-safe for concurrent access
6. ✅ Comprehensive test coverage (>80%)
7. ✅ Benchmarks show good performance
8. ✅ Clean API design
9. ✅ Documentation for usage
10. ✅ Handles edge cases (empty values, large keys, etc.)

## Resources

- [Designing Data-Intensive Applications](https://dataintensive.net/) - Kleppmann
- [RocksDB](https://github.com/facebook/rocksdb) - Reference implementation
- [SQLite Architecture](https://www.sqlite.org/arch.html) - Study-worthy code
- [LMDB](http://www.lmdb.tech/doc/) - Lightning Memory-Mapped Database
- [Criterion.rs](https://bheisler.github.io/criterion.rs/book/) - Benchmarking guide
- [Serialization in Rust](https://serde.rs/) - serde crate documentation

## Timeline

- **Phase 1** (Simple store): 2-3 days
- **Phase 2** (LRU cache): 2-3 days
- **Phase 3** (Persistence): 3-4 days
- **Phase 4** (Transactions): 2-3 days
- **Phase 5** (Advanced): 4-7 days

**Total**: 3-4 weeks for core, longer with advanced features

## Bonus Challenges

Once complete, try:

1. Build a SQL layer on top (query parsing, execution)
2. Implement a distributed version (consensus, replication)
3. Add ACID transactions with conflict detection
4. Create a JavaScript/WebAssembly binding
5. Implement a full LSM tree for better write performance
