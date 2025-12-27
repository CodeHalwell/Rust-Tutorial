/// ============================================================================
/// CUSTOM ITERATORS: Iterator Trait and Composition
/// ============================================================================
///
/// This module demonstrates implementing the Iterator trait for custom types
/// and using iterator adapters for functional-style programming.
///
/// Key insight: Iterators in Rust are zero-cost abstractions that compile
/// to the same code as hand-written loops.

// ============================================================================
// PART 1: ITERATOR TRAIT BASICS
// ============================================================================

/// The core trait for iterables in Rust:
///
/// pub trait Iterator {
///     type Item;
///     fn next(&mut self) -> Option<Self::Item>;
///     // ... many default methods ...
/// }
///
/// Any type implementing Iterator gets automatic methods:
/// - filter()
/// - map()
/// - for_each()
/// - collect()
/// - find()
/// - etc.

/// Example 1: Simple counter iterator
struct CountUp {
    current: u32,
    max: u32,
}

impl CountUp {
    fn new(max: u32) -> Self {
        CountUp { current: 0, max }
    }
}

impl Iterator for CountUp {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.max {
            let result = self.current;
            self.current += 1;
            Some(result)
        } else {
            None
        }
    }
}

// Once we implement Iterator, we get free methods:
// counter.filter(|x| x % 2 == 0).map(|x| x * 2).collect::<Vec<_>>()

// ============================================================================
// PART 2: ITERATOR OVER A DATA STRUCTURE
// ============================================================================

/// A simple linked list that we'll make iterable
#[derive(Debug)]
struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn push(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    fn iter(&self) -> LinkedListIter<T> {
        LinkedListIter {
            next: self.head.as_ref().map(|node| &**node),
        }
    }

    fn iter_mut(&mut self) -> LinkedListIterMut<T> {
        LinkedListIterMut {
            next: self.head.as_mut().map(|node| &mut **node),
        }
    }

    fn into_iter(self) -> LinkedListIntoIter<T> {
        LinkedListIntoIter { next: self.head }
    }
}

/// Immutable iterator: borrows values
struct LinkedListIter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for LinkedListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|n| &**n);
            &node.value
        })
    }
}

/// Mutable iterator: borrows mutable values
struct LinkedListIterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for LinkedListIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|n| &mut **n);
            &mut node.value
        })
    }
}

/// Consuming iterator: takes ownership
struct LinkedListIntoIter<T> {
    next: Option<Box<Node<T>>>,
}

impl<T> Iterator for LinkedListIntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next;
            node.value
        })
    }
}

// ============================================================================
// PART 3: ITERATOR ADAPTERS (HIGH-LEVEL)
// ============================================================================

/// Iterator adapters transform iterators without allocating.
/// They use lazy evaluation - no work happens until you consume the iterator.

fn demonstrate_adapters() {
    println!("=== Iterator Adapters ===");

    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Lazy: nothing happens yet
    let iter = nums
        .iter()
        .filter(|x| *x % 2 == 0)  // Only even numbers
        .map(|x| x * x);           // Square them

    // Now the computation actually happens
    let result: Vec<_> = iter.collect();
    println!("Squared evens: {:?}", result);
    // Output: [4, 16, 36, 64, 100]
}

// ============================================================================
// PART 4: ZERO-COST ABSTRACTION
// ============================================================================

/// Demonstrate that iterators compile to the same code as manual loops.
/// This is guaranteed by Rust's zero-cost abstraction principle.

fn iterator_style(nums: &[u32]) -> u32 {
    nums.iter()
        .filter(|x| *x % 2 == 0)
        .map(|x| x * x)
        .sum()
}

fn loop_style(nums: &[u32]) -> u32 {
    let mut sum = 0;
    for n in nums {
        if n % 2 == 0 {
            sum += n * n;
        }
    }
    sum
}

// Both functions compile to identical assembly!
// The iterator version is more readable but just as fast.

// ============================================================================
// PART 5: CUSTOM ITERATOR ADAPTERS
// ============================================================================

/// You can implement your own iterator adapters using IntoIterator

struct Window<T> {
    items: Vec<T>,
    size: usize,
}

impl<T: Clone> Window<T> {
    fn new(items: Vec<T>, size: usize) -> Self {
        Window { items, size }
    }

    fn iter(&self) -> WindowIter<T> {
        WindowIter {
            items: &self.items,
            size: self.size,
            position: 0,
        }
    }
}

struct WindowIter<'a, T> {
    items: &'a [T],
    size: usize,
    position: usize,
}

impl<'a, T: Clone> Iterator for WindowIter<'a, T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position + self.size <= self.items.len() {
            let window = self.items[self.position..self.position + self.size].to_vec();
            self.position += 1;
            Some(window)
        } else {
            None
        }
    }
}

// ============================================================================
// PART 6: COMMON ITERATOR METHODS
// ============================================================================

fn demonstrate_common_methods() {
    println!("\n=== Common Iterator Methods ===");

    let nums = vec![1, 2, 3, 4, 5, 6];

    // find: Get first matching element
    let first_even = nums.iter().find(|x| *x % 2 == 0);
    println!("First even: {:?}", first_even); // Some(&2)

    // any/all: Check conditions
    let has_odd = nums.iter().any(|x| x % 2 == 1);
    println!("Has odd: {}", has_odd); // true

    let all_positive = nums.iter().all(|x| x > &0);
    println!("All positive: {}", all_positive); // true

    // fold: Accumulate with a function
    let product: i32 = nums.iter().fold(1, |acc, x| acc * x);
    println!("Product: {}", product); // 720

    // take/skip: Get first N or skip first N
    let first_three: Vec<_> = nums.iter().take(3).copied().collect();
    println!("First three: {:?}", first_three); // [1, 2, 3]

    let skip_two: Vec<_> = nums.iter().skip(2).copied().collect();
    println!("Skip two: {:?}", skip_two); // [3, 4, 5, 6]

    // enumerate: Get index and value
    for (i, val) in nums.iter().enumerate() {
        if i < 3 {
            println!("  [{}] = {}", i, val);
        }
    }

    // zip: Combine two iterators
    let letters = vec!['a', 'b', 'c'];
    let pairs: Vec<_> = nums.iter().zip(letters.iter()).collect();
    println!("Zipped: {:?}", pairs);
}

// ============================================================================
// PART 7: FUNCTION COMPOSITION WITH ITERATORS
// ============================================================================

fn demonstrate_composition() {
    println!("\n=== Function Composition ===");

    let data: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Compose multiple operations
    let result: Vec<i32> = data
        .iter()
        // Filter evens
        .filter(|x| *x % 2 == 0)
        // Square them
        .map(|x| x * x)
        // Keep only those > 20
        .filter(|x| x > &20)
        // Multiply by 2
        .map(|x| x * 2)
        // Collect into vector
        .collect();

    println!("Complex composition: {:?}", result);
    // Step by step:
    // [1..10] -> [2,4,6,8,10] -> [4,16,36,64,100] -> [36,64,100] -> [72,128,200]
}

// ============================================================================
// PART 8: INFINITE ITERATORS
// ============================================================================

struct Infinite {
    current: u32,
}

impl Infinite {
    fn new() -> Self {
        Infinite { current: 0 }
    }
}

impl Iterator for Infinite {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.current;
        self.current += 1;
        Some(result)
    }
}

fn demonstrate_infinite() {
    println!("\n=== Infinite Iterator ===");

    // This is safe because we use take()
    let sum: u32 = Infinite::new()
        .take(10)                      // Take first 10
        .filter(|x| x % 2 == 0)        // Even only
        .sum();

    println!("Sum of first 10 evens from infinite: {}", sum); // 0+2+4+6+8 = 20
}

// ============================================================================
// MAIN DEMONSTRATION
// ============================================================================

fn main() {
    println!("=== CUSTOM ITERATORS AND ADAPTERS ===\n");

    // Example 1: Simple custom iterator
    println!("1. CountUp Iterator:");
    for num in CountUp::new(5) {
        print!("{} ", num);
    }
    println!("\n");

    // Example 2: Linked list iteration
    println!("2. LinkedList Iteration:");
    let mut list = LinkedList::new();
    list.push(3);
    list.push(2);
    list.push(1);

    print!("  Immutable: ");
    for val in list.iter() {
        print!("{} ", val);
    }
    println!();

    print!("  Mutable doubling: ");
    for val in list.iter_mut() {
        *val *= 2;
        print!("{} ", val);
    }
    println!("\n");

    // Example 3: Consuming iterator
    let list2 = LinkedList::new();
    // list2 moved here, can't use it after
    for _val in list2.into_iter() {
        // Takes ownership of each value
    }

    // Example 4: Iterator adapters
    demonstrate_adapters();

    // Example 5: Zero-cost abstraction
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("\nIterator vs loop (same result):");
    println!("Iterator style: {}", iterator_style(&nums));
    println!("Loop style: {}", loop_style(&nums));

    // Example 6: Custom adapters
    println!("\n3. Window Iterator:");
    let window = Window::new(vec![1, 2, 3, 4, 5, 6], 3);
    for w in window.iter() {
        println!("  Window: {:?}", w);
    }

    // Example 7: Common methods
    demonstrate_common_methods();

    // Example 8: Composition
    demonstrate_composition();

    // Example 9: Infinite iterators
    demonstrate_infinite();

    println!("\n=== KEY INSIGHTS ===");
    println!("1. Iterator trait provides a standard way to iterate");
    println!("2. Many free methods available: map, filter, fold, etc.");
    println!("3. Adapters are lazy - work happens when you consume");
    println!("4. Zero-cost abstraction: as fast as hand-written loops");
    println!("5. You can compose multiple operations functionally");
    println!("6. IntoIterator trait enables for..in syntax");
    println!("7. Three kinds: iter (borrow), iter_mut (mut borrow), into_iter (own)");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_up() {
        let nums: Vec<_> = CountUp::new(5).collect();
        assert_eq!(nums, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_linked_list_iter() {
        let mut list = LinkedList::new();
        list.push(3);
        list.push(2);
        list.push(1);

        let values: Vec<_> = list.iter().copied().collect();
        assert_eq!(values, vec![1, 2, 3]);
    }

    #[test]
    fn test_iterator_filter_map() {
        let nums = vec![1, 2, 3, 4, 5, 6];
        let result: Vec<_> = nums
            .iter()
            .filter(|x| *x % 2 == 0)
            .map(|x| x * x)
            .collect();

        assert_eq!(result, vec![4, 16, 36]);
    }

    #[test]
    fn test_window_iterator() {
        let window = Window::new(vec![1, 2, 3, 4, 5], 3);
        let windows: Vec<_> = window.iter().collect();

        assert_eq!(windows.len(), 3);
        assert_eq!(windows[0], vec![1, 2, 3]);
        assert_eq!(windows[1], vec![2, 3, 4]);
        assert_eq!(windows[2], vec![3, 4, 5]);
    }

    #[test]
    fn test_infinite_with_take() {
        let sum: u32 = Infinite::new().take(5).sum();
        assert_eq!(sum, 10); // 0+1+2+3+4
    }
}
