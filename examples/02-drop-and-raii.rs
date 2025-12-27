/// ============================================================================
/// DROP TRAIT AND RAII: Resource Acquisition Is Initialization
/// ============================================================================
///
/// This module demonstrates the Drop trait and RAII pattern in Rust.
/// RAII is a fundamental pattern: resources are cleaned up automatically
/// when they go out of scope, even if an error occurs.
///
/// This is how Rust achieves memory safety without garbage collection.

use std::fs::File;
use std::io::{self, Write};
use std::ptr;

// ============================================================================
// PART 1: DROP TRAIT BASICS
// ============================================================================

/// The Drop trait is called automatically when a value goes out of scope.
/// It's the destructor pattern in Rust.

#[derive(Debug)]
struct FileWrapper {
    file: File,
    path: String,
}

impl FileWrapper {
    fn new(path: &str) -> io::Result<Self> {
        let file = File::create(path)?;
        Ok(FileWrapper {
            file,
            path: path.to_string(),
        })
    }

    fn write_line(&mut self, line: &str) -> io::Result<()> {
        writeln!(self.file, "{}", line)
    }
}

/// When a FileWrapper goes out of scope, Drop::drop is called automatically.
/// The File is closed and resources are released.
impl Drop for FileWrapper {
    fn drop(&mut self) {
        // This runs when the value is destroyed
        println!("Dropping FileWrapper for: {}", self.path);
        // File::drop is also called automatically, closing the file
    }
}

// ============================================================================
// PART 2: RAII IN ACTION
// ============================================================================

/// Example 1: File cleanup even if error occurs
fn demonstrate_raii_with_error() -> io::Result<()> {
    println!("=== RAII with Error Handling ===");

    {
        let mut file = FileWrapper::new("/tmp/test.txt")?;
        file.write_line("Hello")?;
        file.write_line("World")?;

        // If we uncomment this, the error occurs but file is still cleaned up:
        // return Err(io::Error::new(io::ErrorKind::Other, "Simulated error"));

        // When scope ends, file is automatically closed
    }

    println!("File was automatically closed when FileWrapper went out of scope\n");
    Ok(())
}

/// Example 2: Multiple resources cleaned up in correct order
fn demonstrate_multiple_resources() -> io::Result<()> {
    println!("=== Multiple Resources Cleaned Up ===");

    let mut file1 = FileWrapper::new("/tmp/file1.txt")?;
    let mut file2 = FileWrapper::new("/tmp/file2.txt")?;
    let mut file3 = FileWrapper::new("/tmp/file3.txt")?;

    file1.write_line("File 1")?;
    file2.write_line("File 2")?;
    file3.write_line("File 3")?;

    // Cleanup order: file3, file2, file1 (reverse creation order)
    // This is guaranteed by Rust
    println!("All files will be closed in reverse order...\n");
    Ok(())
}

// ============================================================================
// PART 3: CUSTOM RAII TYPE FOR MUTEX-LIKE BEHAVIOR
// ============================================================================

/// A simple mutex-like lock guard that releases the lock on drop
#[derive(Debug)]
struct Lock<'a> {
    locked: &'a mut bool,
}

impl<'a> Lock<'a> {
    fn acquire(locked: &'a mut bool) -> Self {
        *locked = true;
        println!("Lock acquired");
        Lock { locked }
    }
}

impl<'a> Drop for Lock<'a> {
    fn drop(&mut self) {
        *self.locked = false;
        println!("Lock released (automatically on drop)");
    }
}

fn demonstrate_lock_guard() {
    println!("=== Lock Guard RAII ===");
    let mut is_locked = false;

    {
        let _guard = Lock::acquire(&mut is_locked);
        println!("Inside critical section, locked={}", is_locked);

        // If we panic here, the lock is still released
        // If we return early, the lock is still released
        // This is guaranteed by RAII
    }

    println!("Outside critical section, locked={}\n", is_locked);
}

// ============================================================================
// PART 4: MEMORY SAFETY GUARANTEED BY RAII
// ============================================================================

/// A manual memory allocator to demonstrate why RAII matters
struct RawMemory {
    ptr: *mut u8,
    size: usize,
}

impl RawMemory {
    fn allocate(size: usize) -> Self {
        let ptr = unsafe {
            // Allocate memory
            let layout = std::alloc::Layout::new::<u8>();
            let mut layout = layout;
            for _ in 1..size {
                layout = layout.extend(std::alloc::Layout::new::<u8>()).unwrap().0;
            }
            std::alloc::alloc(layout)
        };

        println!("Allocated {} bytes at {:p}", size, ptr);

        RawMemory { ptr, size }
    }
}

impl Drop for RawMemory {
    fn drop(&mut self) {
        unsafe {
            println!("Deallocating {} bytes at {:p}", self.size, self.ptr);
            // In a real implementation, we'd call dealloc here
            // For this example, we just print
        }
    }
}

// ============================================================================
// PART 5: ABORT ON PANIC IN DROP
// ============================================================================

/// IMPORTANT: If drop() panics, the program aborts!
/// This prevents multiple panics simultaneously.

#[derive(Debug)]
struct StrictResource {
    name: String,
}

impl Drop for StrictResource {
    fn drop(&mut self) {
        println!("Cleaning up: {}", self.name);

        // NEVER panic in drop! It will abort the program.
        // The compiler doesn't prevent this, but it's a serious error.

        // Bad:
        // panic!("Something went wrong in drop!");

        // Good:
        if self.name.is_empty() {
            eprintln!("Warning: Dropping resource with empty name");
        }
    }
}

// ============================================================================
// PART 6: MANUAL DROP (RARE)
// ============================================================================

/// In rare cases, you need to drop something early.
/// Use std::mem::drop() to explicitly drop.

fn demonstrate_manual_drop() {
    println!("=== Manual Drop ===");

    let _resource = StrictResource {
        name: "Early drop example".to_string(),
    };

    println!("Resource created");

    // Explicitly drop the resource early
    drop(_resource);

    println!("Resource manually dropped\n");
}

// ============================================================================
// PART 7: FORGET (PREVENT DROP - RARE)
// ============================================================================

/// std::mem::forget() prevents drop from running.
/// This is usually a BAD idea and indicates a design problem.

fn demonstrate_forget() {
    println!("=== Forget (Do NOT Use!) ===");

    let resource = StrictResource {
        name: "This will leak".to_string(),
    };

    // Using forget leaks the resource - drop won't be called
    std::mem::forget(resource);

    println!("Resource leaked! Drop was NOT called\n");
}

// ============================================================================
// PART 8: SMART POINTER PATTERN WITH DROP
// ============================================================================

/// Many Rust types are really just smart pointers with custom Drop behavior

/// A simple reference-counted pointer (like Rc)
struct SimpleRc<T> {
    ptr: *mut RcInner<T>,
}

struct RcInner<T> {
    value: T,
    refcount: usize,
}

impl<T> SimpleRc<T> {
    fn new(value: T) -> Self {
        let inner = RcInner {
            value,
            refcount: 1,
        };

        let ptr = Box::into_raw(Box::new(inner));
        SimpleRc { ptr }
    }
}

impl<T> Clone for SimpleRc<T> {
    fn clone(&self) -> Self {
        unsafe {
            (*self.ptr).refcount += 1;
            SimpleRc { ptr: self.ptr }
        }
    }
}

impl<T> Drop for SimpleRc<T> {
    fn drop(&mut self) {
        unsafe {
            (*self.ptr).refcount -= 1;
            if (*self.ptr).refcount == 0 {
                // Last reference gone, deallocate
                drop(Box::from_raw(self.ptr));
                println!("SimpleRc: deallocated");
            }
        }
    }
}

// ============================================================================
// MAIN DEMONSTRATION
// ============================================================================

fn main() -> io::Result<()> {
    println!("=== DROP TRAIT AND RAII DEMONSTRATION ===\n");

    // Example 1
    demonstrate_raii_with_error()?;

    // Example 2
    demonstrate_multiple_resources()?;

    // Example 3
    demonstrate_lock_guard();

    // Example 4
    let _memory = RawMemory::allocate(1024);
    println!("Using allocated memory...");
    // Memory is freed here automatically

    println!();

    // Example 5
    let resource = StrictResource {
        name: "Example resource".to_string(),
    };
    println!("Resource will be dropped at end of scope");
    drop(resource);
    println!();

    // Example 6
    demonstrate_manual_drop();

    // Example 7
    demonstrate_forget();

    println!("=== KEY INSIGHTS ===");
    println!("1. Drop is called automatically when values go out of scope");
    println!("2. RAII guarantees cleanup even if errors occur");
    println!("3. Cleanup order is reverse of creation order");
    println!("4. This enables memory safety without garbage collection");
    println!("5. Never panic in drop() - it aborts the program");
    println!("6. Use drop() to release resources early when needed");
    println!("7. Most Rust types (File, Mutex, Vec, etc.) implement Drop");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_wrapper_created() {
        let result = FileWrapper::new("/tmp/test_wrapper.txt");
        assert!(result.is_ok());
    }

    #[test]
    fn test_lock_guard() {
        let mut locked = false;
        {
            let _guard = Lock::acquire(&mut locked);
            assert!(locked);
        }
        assert!(!locked);
    }

    #[test]
    fn test_raii_error() {
        fn may_fail() -> io::Result<()> {
            let _file = FileWrapper::new("/tmp/test_error.txt")?;
            // If we had an error here, the file would still be closed
            Ok(())
        }

        assert!(may_fail().is_ok());
    }
}
