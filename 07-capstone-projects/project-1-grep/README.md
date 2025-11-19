# Project 1: Grep Clone (Intermediate)

## Overview

Build a command-line text search tool that replicates core functionality of Unix `grep`. This project teaches file I/O, error handling, pattern matching, and command-line argument parsing.

## Learning Objectives

- ✅ Idiomatic error handling (Result, ? operator, custom errors)
- ✅ Working with files and reading lines efficiently
- ✅ Command-line argument parsing
- ✅ String processing and pattern matching
- ✅ Writing testable code with separation of concerns
- ✅ Performance: efficient file reading and searching

## Requirements

### Core Functionality

Your grep clone must support:

1. **Basic pattern matching** (literal strings and regex)
   ```bash
   $ grep "pattern" file.txt
   $ grep "^start" file.txt          # Regex support
   $ grep "[0-9]+" file.txt          # Character classes
   ```

2. **Flags**
   - `-i`: Case-insensitive matching
   - `-n`: Print line numbers
   - `-v`: Invert match (show lines that DON'T match)
   - `-c`: Count matching lines (don't print them)
   - `-l`: List files only (when multiple files)

3. **Multiple files**
   ```bash
   $ grep "pattern" file1.txt file2.txt file3.txt
   # Output format: filename:matching_line
   ```

4. **stdin support**
   ```bash
   $ cat file.txt | grep "pattern"
   $ grep "pattern" < file.txt
   ```

5. **Error handling**
   - File not found → error message, exit code 2
   - Invalid regex → error message, exit code 2
   - No matches → exit code 1 (with no output)
   - Match found → exit code 0

### Example Usage

```bash
# Simple search
$ grep "hello" document.txt
This line has hello in it
Another hello here

# Case-insensitive with line numbers
$ grep -ni "HELLO" document.txt
1:This line has hello in it
3:Another hello here

# Invert match
$ grep -v "hello" document.txt
# Lines that don't contain "hello"

# Count matches
$ grep -c "pattern" file.txt
42

# Multiple files
$ grep "error" app.log system.log
app.log:Error: connection timeout
system.log:Error: disk full

# From stdin
$ ps aux | grep "rust"
```

## Implementation Guide

### Phase 1: Basic Structure

1. Create a `Config` struct to hold parsed arguments
2. Implement `Config::build()` to parse command-line args
3. Create a `run()` function that takes Config and executes the search
4. Separate concerns: CLI parsing, file reading, pattern matching

### Phase 2: Core Search Logic

1. Read files line-by-line efficiently
2. Implement literal string matching first
3. Add regex support using the `regex` crate
4. Handle all flag combinations

### Phase 3: Error Handling

1. Use custom Result type: `type GrepResult<T> = Result<T, GrepError>`
2. Implement proper error types (FileNotFound, InvalidRegex, etc.)
3. Propagate errors with ? operator
4. Print errors to stderr, return appropriate exit codes

### Phase 4: Testing

1. Unit tests for Config parsing
2. Integration tests with temporary files
3. Test flag combinations
4. Test error cases

## Code Architecture

Suggested file structure:

```
src/
├── main.rs           # CLI entry point, exit code handling
├── config.rs         # Config struct and parsing
├── search.rs         # Search logic and pattern matching
├── error.rs          # Custom error types
└── lib.rs            # Public API for library use
```

### Core Traits/Types to Implement

```rust
pub struct Config {
    pattern: String,
    files: Vec<PathBuf>,
    case_insensitive: bool,
    show_line_numbers: bool,
    invert_match: bool,
    count_only: bool,
    list_files_only: bool,
}

impl Config {
    pub fn build(args: impl IntoIterator<Item = String>) -> Result<Self, String>;
}

pub fn run(config: Config) -> Result<(), GrepError>;

pub fn search(pattern: &str, text: &str, case_insensitive: bool) -> Vec<String>;
```

## Testing Checklist

- [ ] Single file, literal match
- [ ] Single file, regex match
- [ ] Case-insensitive flag
- [ ] Line numbers flag
- [ ] Invert match flag
- [ ] Count flag
- [ ] Multiple files (show filename in output)
- [ ] stdin input
- [ ] File not found error
- [ ] Invalid regex error
- [ ] No matches (exit code 1)
- [ ] Matches found (exit code 0)

## Performance Considerations

- Read files in chunks, not all at once
- Use `BufReader` for efficient line reading
- Avoid unnecessary string allocations
- Consider memory usage for very large files

## Advanced Extensions (Optional)

1. **Recursive search**: `-r` flag to search directories recursively
2. **Output context**: `-A 2` (after), `-B 2` (before), `-C 2` (context)
3. **File type filtering**: `--include="*.rs"` to search only Rust files
4. **Color output**: `-H` to highlight matches in output
5. **Byte offset**: `-b` to show byte offset of matches
6. **Word boundary**: `-w` to match whole words only

## Helpful Crates

- `regex`: Pattern matching
- `clap`: Advanced argument parsing (optional, use std::env for basic version)
- `anyhow`: Error handling (alternative to custom errors)

## Debugging Hints

- Use `dbg!()` to inspect Config after parsing
- Print regex after compilation to verify parsing
- Test with small files first, then scale up
- Check exit codes with `echo $?` in bash

## Completion Criteria

Your implementation is complete when:

1. ✅ All core flags work correctly
2. ✅ Works with single file, multiple files, and stdin
3. ✅ Proper exit codes for all scenarios
4. ✅ Clear error messages to stderr
5. ✅ Comprehensive test coverage
6. ✅ Code is well-documented with comments
7. ✅ No compiler warnings (run `cargo clippy`)
8. ✅ Formats correctly (`cargo fmt`)

## Real-World Comparison

Test your implementation against actual `grep`:

```bash
# Set up test file
echo -e "hello\nHELLO\nworld\nhello world" > test.txt

# Test your implementation vs grep
$ cargo run -- "hello" test.txt
$ grep "hello" test.txt

$ cargo run -- -i "hello" test.txt
$ grep -i "hello" test.txt

$ cargo run -- -n "hello" test.txt
$ grep -n "hello" test.txt
```

## Resources

- [Rust Book: Chapter 12 - Building a Command Line Program](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)
- [regex crate documentation](https://docs.rs/regex/)
- [Unix grep manual](https://pubs.opengroup.org/onlinepubs/9699919799/utilities/grep.html)
