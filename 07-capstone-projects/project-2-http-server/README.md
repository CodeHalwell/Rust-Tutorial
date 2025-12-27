# Project 2: Multi-threaded HTTP Server (Intermediate-Advanced)

## Overview

Build a production-quality HTTP/1.0 web server from scratch using only the Rust standard library (no web frameworks). This project teaches networking, concurrency, thread pooling, HTTP protocol understanding, and performance optimization.

## Learning Objectives

- ✅ TCP networking (TcpListener, TcpStream, socket operations)
- ✅ Concurrency without async/await (thread-based parallelism)
- ✅ Thread pool implementation and work queue management
- ✅ HTTP protocol parsing and response generation
- ✅ Error handling in networked applications
- ✅ Performance measurement and benchmarking
- ✅ Clean API design and modularity

## Requirements

### Core Functionality

Your server must:

1. **Listen on localhost:7878**
   ```bash
   $ cargo run
   Server running on http://127.0.0.1:7878
   ```

2. **Serve static files over HTTP/1.0**
   ```bash
   $ curl http://localhost:7878/index.html
   $ curl http://localhost:7878/styles/main.css
   $ curl http://localhost:7878/js/app.js
   ```

3. **Handle GET requests only**
   - Return the file if it exists
   - Return 404 if file not found
   - Return 500 on server error

4. **Thread pool for concurrent requests**
   - Default: 4 worker threads
   - Handle multiple simultaneous connections
   - Graceful shutdown

5. **Basic routing**
   - Serve files from `public/` directory
   - Deny access outside that directory (security)
   - Handle directory requests (serve index.html)

### HTTP Responses

Implement proper HTTP/1.0 responses:

```
GET /index.html HTTP/1.0
Host: localhost:7878

HTTP/1.0 200 OK
Content-Type: text/html
Content-Length: 1234

<file contents>
```

**Response codes:**
- 200 OK: Successful request
- 404 Not Found: File doesn't exist
- 403 Forbidden: Trying to access outside public directory
- 500 Internal Server Error: Server error

## Implementation Guide

### Phase 1: Basic TCP Server

1. Create a TcpListener on 127.0.0.1:7878
2. Accept connections in a loop
3. Read HTTP request from TcpStream
4. Send a simple response
5. Log incoming requests

```rust
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    for stream in listener.incoming() {
        let stream = stream?;
        handle_connection(stream)?;
    }
}

fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer)?;
    println!("Request: {}", String::from_utf8_lossy(&buffer[..bytes_read]));

    let response = "HTTP/1.0 200 OK\r\n\r\nHello, World!";
    stream.write_all(response.as_bytes())?;
    Ok(())
}
```

### Phase 2: HTTP Request Parsing

1. Parse HTTP request line: `GET /path HTTP/1.0`
2. Extract method, path, and version
3. Parse headers (optional for this project)
4. Handle malformed requests gracefully

```rust
pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: PathBuf,
    pub version: String,
}

pub enum HttpMethod {
    Get,
    Head,
    Unsupported,
}

impl HttpRequest {
    pub fn parse(raw: &str) -> Result<Self> {
        // Parse first line: GET /path HTTP/1.0
        // Return error if malformed
    }
}
```

### Phase 3: File Serving

1. Resolve requested path to file on disk
2. Check path doesn't escape `public/` directory
3. Read file contents
4. Generate appropriate HTTP response with Content-Type

```rust
pub fn serve_file(requested_path: &Path) -> Result<(HttpStatus, String, Vec<u8>)> {
    let canonical = fs::canonicalize(&requested_path)?;
    let public_dir = fs::canonicalize("public")?;

    // Verify path is within public/
    if !canonical.starts_with(&public_dir) {
        return Ok((HttpStatus::Forbidden, "text/plain".into(), b"Access Denied".to_vec()));
    }

    // Serve directory index
    if canonical.is_dir() {
        let index = canonical.join("index.html");
        // ...
    }

    // Read and serve file
    let contents = fs::read(&canonical)?;
    let content_type = guess_content_type(&canonical);
    Ok((HttpStatus::Ok, content_type, contents))
}
```

### Phase 4: Thread Pool Implementation

Build a thread pool to handle requests concurrently:

```rust
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Message>,
}

enum Message {
    NewJob(Job),
    Terminate,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> Result<Self> {
        // Create size worker threads
        // Return channel sender for job queue
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job))?;
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // Send Terminate to all workers
        // Wait for workers to finish
    }
}
```

### Phase 5: Integration and Polish

1. Connect request handling to thread pool
2. Add proper error logging
3. Implement graceful shutdown
4. Add configuration (port, thread count, root directory)

## Code Architecture

Suggested structure:

```
src/
├── main.rs                    # Entry point, server setup
├── lib.rs                     # Public API
├── server.rs                  # TcpListener, connection handling
├── thread_pool.rs             # ThreadPool implementation
├── http/
│   ├── mod.rs
│   ├── request.rs             # HttpRequest parsing
│   ├── response.rs            # HttpResponse building
│   └── status.rs              # HTTP status codes
├── file_server.rs             # File serving logic
└── error.rs                   # Error types
```

## Testing Checklist

- [ ] Server starts and listens on port 7878
- [ ] GET request returns 200 with file contents
- [ ] 404 for missing files
- [ ] 403 when accessing outside public/ directory
- [ ] Multiple concurrent requests handled
- [ ] Directory requests serve index.html
- [ ] Correct Content-Type headers
- [ ] Correct Content-Length headers
- [ ] Graceful shutdown (Ctrl+C)
- [ ] Malformed HTTP requests don't crash server

## Performance Optimization

### Benchmarking

Measure request throughput:

```bash
# Using Apache Bench
ab -n 1000 -c 10 http://localhost:7878/test.html

# Using wrk
wrk -t4 -c100 -d30s http://localhost:7878/test.html
```

### Optimization Opportunities

1. **Buffer pooling**: Reuse buffers instead of allocating per request
2. **Zero-copy responses**: mmap files for very large files
3. **Connection keep-alive**: Handle multiple requests per connection
4. **Reduce allocations**: Pre-allocate response buffers
5. **Sendfile**: Use OS sendfile() for efficient file transfer

### Profiling

```bash
# Create a flame graph
cargo install flamegraph
sudo cargo flamegraph --bin server -- [args]
# Opens flamegraph.svg
```

## HTTP/1.0 Specification Details

Key points:

1. **Connection handling**: Close connection after each response (HTTP/1.0 default)
2. **Status line**: `HTTP/1.0 200 OK\r\n`
3. **Headers**: `Header-Name: value\r\n`
4. **CRLF line endings**: All lines end with `\r\n`
5. **Blank line**: `\r\n` separates headers from body
6. **Content-Length**: Required for response body
7. **Content-Type**: Tell client what type of data is being sent

### Minimal Valid Response

```
HTTP/1.0 200 OK\r\n
Content-Type: text/plain\r\n
Content-Length: 13\r\n
\r\n
Hello, World!
```

## Setup

Create test files:

```bash
mkdir -p public/css public/js
echo "<h1>Hello</h1>" > public/index.html
echo "body { color: blue; }" > public/css/style.css
echo "console.log('hi');" > public/js/app.js
```

## Advanced Extensions (Optional)

1. **HTTP/1.1 support**: Keep-alive connections
2. **Range requests**: Partial file downloads (206 Partial Content)
3. **Gzip compression**: Compress responses on-the-fly
4. **Caching headers**: ETag, Last-Modified, Cache-Control
5. **HTTPS/TLS**: Using rustls or OpenSSL
6. **Virtual hosts**: Serve multiple domains
7. **CGI support**: Execute scripts
8. **Directory listing**: HTML listing of directory contents
9. **Rate limiting**: Prevent abuse
10. **Load balancing**: Multiple worker pools with queue management

## Advanced Concepts to Explore

- **Message passing**: How channels work in Rust
- **Lock-free queues**: Alternative to Mutex-based work queue
- **Backpressure**: Handling when queue gets full
- **Connection pooling**: Managing persistent connections
- **Async/await**: Compare to thread-based approach

## Debugging Tips

1. **Print all requests**: Log raw HTTP requests to see format
2. **Network monitoring**: `tcpdump` or Wireshark
3. **curl debugging**: `curl -v http://localhost:7878/`
4. **Check headers**: Use `curl -i` to see response headers
5. **File system operations**: Debug path resolution issues

## Completion Criteria

Your server is complete when:

1. ✅ Listens on 127.0.0.1:7878
2. ✅ Serves static files from public/ directory
3. ✅ Returns correct HTTP status codes
4. ✅ Handles multiple concurrent connections
5. ✅ Uses thread pool with configurable workers
6. ✅ No path traversal vulnerabilities
7. ✅ Proper error handling throughout
8. ✅ Comprehensive test coverage
9. ✅ Can be benchmarked with Apache Bench
10. ✅ Gracefully handles shutdown

## Resources

- [HTTP/1.0 Specification (RFC 1945)](https://tools.ietf.org/html/rfc1945)
- [Rust std::net documentation](https://doc.rust-lang.org/std/net/)
- [Thread Pool Pattern](https://doc.rust-lang.org/book/ch20-02-multithreaded-web-server.html)
- [Apache Bench](https://httpd.apache.org/docs/2.4/programs/ab.html)
- [Building a Multithreaded Web Server](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html)
