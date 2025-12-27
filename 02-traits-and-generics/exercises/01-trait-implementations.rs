/// ============================================================================
/// TRAITS AND GENERICS EXERCISES
/// ============================================================================
///
/// Master Rust's abstraction system through hands-on practice

// ============================================================================
// EXERCISE 1: BASIC TRAIT DEFINITION AND IMPLEMENTATION
// ============================================================================

pub trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;

    // Provided method with default implementation
    fn describe(&self) -> String {
        format!("Shape with area: {:.2}, perimeter: {:.2}",
                self.area(), self.perimeter())
    }
}

pub struct Circle {
    radius: f64,
}

pub struct Rectangle {
    width: f64,
    height: f64,
}

// Implement Shape for Circle
impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

// Implement Shape for Rectangle
impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

pub fn exercise_1_trait_implementation() {
    println!("=== EXERCISE 1: Trait Implementation ===");

    let circle = Circle { radius: 5.0 };
    println!("Circle: {}", circle.describe());

    let rect = Rectangle { width: 4.0, height: 6.0 };
    println!("Rectangle: {}", rect.describe());
}

// ============================================================================
// EXERCISE 2: GENERIC FUNCTIONS WITH TRAIT BOUNDS
// ============================================================================

pub fn print_shape<T: Shape>(shape: &T) {
    println!("Area: {:.2}", shape.area());
}

pub fn largest_area<T: Shape>(shapes: &[T]) -> f64 {
    shapes.iter()
        .map(|s| s.area())
        .fold(0.0, f64::max)
}

pub fn exercise_2_generic_functions() {
    println!("\n=== EXERCISE 2: Generic Functions ===");

    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 3.0 }),
        Box::new(Rectangle { width: 2.0, height: 5.0 }),
    ];

    for shape in &shapes {
        print_shape(shape);
    }
}

// ============================================================================
// EXERCISE 3: TRAIT OBJECTS (DYNAMIC DISPATCH)
// ============================================================================

pub trait Animal {
    fn speak(&self) -> String;
    fn name(&self) -> &str;
}

pub struct Dog {
    name: String,
}

pub struct Cat {
    name: String,
}

impl Animal for Dog {
    fn speak(&self) -> String {
        "woof".to_string()
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl Animal for Cat {
    fn speak(&self) -> String {
        "meow".to_string()
    }

    fn name(&self) -> &str {
        &self.name
    }
}

pub fn make_animal_speak(animal: &dyn Animal) {
    println!("{} says: {}", animal.name(), animal.speak());
}

pub fn exercise_3_trait_objects() {
    println!("\n=== EXERCISE 3: Trait Objects ===");

    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog { name: "Buddy".to_string() }),
        Box::new(Cat { name: "Whiskers".to_string() }),
        Box::new(Dog { name: "Rex".to_string() }),
    ];

    for animal in &animals {
        make_animal_speak(animal);
    }
}

// ============================================================================
// EXERCISE 4: GENERIC STRUCTS
// ============================================================================

pub struct Container<T> {
    items: Vec<T>,
}

impl<T> Container<T> {
    pub fn new() -> Self {
        Container { items: Vec::new() }
    }

    pub fn add(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items.iter()
    }
}

// Specialized implementation for Container<i32>
impl Container<i32> {
    pub fn sum(&self) -> i32 {
        self.items.iter().sum()
    }

    pub fn average(&self) -> f64 {
        if self.items.is_empty() {
            0.0
        } else {
            self.sum() as f64 / self.items.len() as f64
        }
    }
}

pub fn exercise_4_generic_structs() {
    println!("\n=== EXERCISE 4: Generic Structs ===");

    let mut int_container = Container::new();
    int_container.add(10);
    int_container.add(20);
    int_container.add(30);

    println!("Sum: {}", int_container.sum());
    println!("Average: {:.2}", int_container.average());

    let mut string_container = Container::new();
    string_container.add("hello");
    string_container.add("world");

    println!("String container length: {}", string_container.len());
    for s in string_container.iter() {
        println!("  - {}", s);
    }
}

// ============================================================================
// EXERCISE 5: ASSOCIATED TYPES
// ============================================================================

pub trait Iterator2 {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    fn count(mut self) -> usize {
        let mut count = 0;
        while self.next().is_some() {
            count += 1;
        }
        count
    }
}

pub struct CountUp {
    current: i32,
    max: i32,
}

impl Iterator2 for CountUp {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        if self.current < self.max {
            let result = self.current;
            self.current += 1;
            Some(result)
        } else {
            None
        }
    }
}

pub fn exercise_5_associated_types() {
    println!("\n=== EXERCISE 5: Associated Types ===");

    let counter = CountUp { current: 1, max: 5 };
    let count = counter.count();
    println!("Counted: {}", count);

    let mut counter2 = CountUp { current: 1, max: 5 };
    while let Some(num) = counter2.next() {
        print!("{} ", num);
    }
    println!();
}

// ============================================================================
// EXERCISE 6: TRAIT BOUNDS WITH WHERE CLAUSE
// ============================================================================

use std::fmt::Display;

pub fn print_collection<T>(collection: &[T])
where
    T: Display,
{
    for item in collection {
        println!("{}", item);
    }
}

pub fn find_first<T, P>(collection: &[T], predicate: P) -> Option<&T>
where
    P: Fn(&T) -> bool,
{
    collection.iter().find(|item| predicate(item))
}

pub fn exercise_6_where_clause() {
    println!("\n=== EXERCISE 6: Where Clause ===");

    let numbers = vec![1, 2, 3, 4, 5];
    print_collection(&numbers);

    if let Some(first_even) = find_first(&numbers, |x| x % 2 == 0) {
        println!("First even: {}", first_even);
    }
}

// ============================================================================
// EXERCISE 7: MULTIPLE TRAIT BOUNDS
// ============================================================================

use std::fmt::Debug;

pub fn compare_and_print<T: PartialOrd + Display + Debug>(a: T, b: T) {
    println!("Comparing: {:?} and {:?}", a, b);
    if a < b {
        println!("{} < {}", a, b);
    } else {
        println!("{} >= {}", a, b);
    }
}

pub fn exercise_7_multiple_bounds() {
    println!("\n=== EXERCISE 7: Multiple Trait Bounds ===");

    compare_and_print(5, 10);
    compare_and_print("apple", "banana");
}

// ============================================================================
// EXERCISE 8: TRAIT COMPOSITION (TRAIT INHERITANCE)
// ============================================================================

pub trait Pet: Animal {
    fn get_owner(&self) -> &str;

    fn full_info(&self) -> String {
        format!("{} owned by {}", self.name(), self.get_owner())
    }
}

pub struct PetDog {
    name: String,
    owner: String,
}

impl Animal for PetDog {
    fn speak(&self) -> String {
        "woof".to_string()
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl Pet for PetDog {
    fn get_owner(&self) -> &str {
        &self.owner
    }
}

pub fn exercise_8_trait_composition() {
    println!("\n=== EXERCISE 8: Trait Composition ===");

    let pet = PetDog {
        name: "Buddy".to_string(),
        owner: "Alice".to_string(),
    };

    println!("{}", pet.full_info());
}

// ============================================================================
// EXERCISE 9: GENERIC ENUM
// ============================================================================

pub enum Result2<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E: Display> Result2<T, E> {
    pub fn is_ok(&self) -> bool {
        matches!(self, Result2::Ok(_))
    }

    pub fn is_err(&self) -> bool {
        matches!(self, Result2::Err(_))
    }

    pub fn map<U, F: FnOnce(T) -> U>(self, op: F) -> Result2<U, E> {
        match self {
            Result2::Ok(t) => Result2::Ok(op(t)),
            Result2::Err(e) => Result2::Err(e),
        }
    }
}

pub fn exercise_9_generic_enum() {
    println!("\n=== EXERCISE 9: Generic Enum ===");

    let success: Result2<i32, String> = Result2::Ok(42);
    println!("Is OK: {}", success.is_ok());

    let result: Result2<i32, String> = success.map(|x| x * 2);
    match result {
        Result2::Ok(val) => println!("Result: {}", val),
        Result2::Err(e) => println!("Error: {}", e),
    }
}

// ============================================================================
// EXERCISE 10: PHANTOM TYPES
// ============================================================================

use std::marker::PhantomData;

pub struct TaggedValue<T, Tag> {
    value: T,
    _tag: PhantomData<Tag>,
}

pub struct Celsius;
pub struct Fahrenheit;

impl<Tag> TaggedValue<f64, Tag> {
    pub fn new(value: f64) -> Self {
        TaggedValue {
            value,
            _tag: PhantomData,
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

pub fn exercise_10_phantom_types() {
    println!("\n=== EXERCISE 10: Phantom Types ===");

    let celsius: TaggedValue<f64, Celsius> = TaggedValue::new(25.0);
    let fahrenheit: TaggedValue<f64, Fahrenheit> = TaggedValue::new(77.0);

    println!("Celsius: {}", celsius.value());
    println!("Fahrenheit: {}", fahrenheit.value());

    // These are different types - compiler prevents mixing
}

// ============================================================================
// EXERCISE 11: CONDITIONAL TRAIT IMPLEMENTATION
// ============================================================================

pub trait Printable {
    fn print(&self);
}

// Only implement Printable for Container<T> if T implements Display
impl<T: Display> Printable for Container<T> {
    fn print(&self) {
        for (i, item) in self.iter().enumerate() {
            println!("[{}] {}", i, item);
        }
    }
}

pub fn exercise_11_conditional_impl() {
    println!("\n=== EXERCISE 11: Conditional Implementation ===");

    let mut container = Container::new();
    container.add("hello");
    container.add("world");

    container.print();

    // Container<i32> would also implement Printable
    // but Container<SomeNonDisplayType> wouldn't
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    exercise_1_trait_implementation();
    exercise_2_generic_functions();
    exercise_3_trait_objects();
    exercise_4_generic_structs();
    exercise_5_associated_types();
    exercise_6_where_clause();
    exercise_7_multiple_bounds();
    exercise_8_trait_composition();
    exercise_9_generic_enum();
    exercise_10_phantom_types();
    exercise_11_conditional_impl();

    println!("\n=== ALL EXERCISES COMPLETE ===");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_area() {
        let circle = Circle { radius: 1.0 };
        assert!((circle.area() - std::f64::consts::PI).abs() < 0.01);
    }

    #[test]
    fn test_rectangle_area() {
        let rect = Rectangle { width: 3.0, height: 4.0 };
        assert_eq!(rect.area(), 12.0);
    }

    #[test]
    fn test_container() {
        let mut container = Container::new();
        container.add(1);
        container.add(2);
        container.add(3);
        assert_eq!(container.len(), 3);
        assert_eq!(container.sum(), 6);
    }

    #[test]
    fn test_result_map() {
        let result: Result2<i32, String> = Result2::Ok(5);
        let mapped = result.map(|x| x * 2);
        assert!(mapped.is_ok());
    }
}
