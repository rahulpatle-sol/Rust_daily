// // Without generics - need different functions for each type
// fn print_i32(value: i32) {
//     println!("{}", value);
// }

// fn print_string(value: String) {
//     println!("{}", value);
// }

// // With generics - ONE function for all types!
// fn print_value<T: std::fmt::Display>(value: T) {
//     println!("{}", value);
// }
// // lets crete an anothrerf generic
// fn say_hello<T: std::fmt::Display>(name: T) {
//     println!("Hello, {}!", name);
// }
// fn main() {
//     print_value(42);
//     print_value("Hello");
//     print_value(3.14);
//     say_hello("Alice");
//     let point = Point { x: 1.0, y: 2.0 };
//     println!("Point: ({}, {})", point.x, point.y);
//     let some_value = Option::Some(10);
//     let no_value: Option<i32> = Option::None;
//     match some_value {
//         Option::Some(val) => println!("Got a value: {}", val),
//         Option::None => println!("No value"),   }


//         let pair1 = Pair::new(1, "one");
//     let pair2 = Pair::new(3.14, true);
    
//     println!("First: {}, Second: {}", pair1.first, pair1.second);
//     println!("First: {}, Second: {}", pair2.first, pair2.second);
//     let max = largest(10, 20);
//     println!("Largest: {}", max);
//     //  hashmap example
//     let mut map: HashMap<String, i32> = HashMap::new();
//     map.insert("Alice".to_string(), 30);
//     map.insert("Bob".to_string(), 25);
//     for (key, value) in &map {
//         println!("{}: {}", key, value);     
// }

// // generic is a powerful feature that allows us to write flexible and reusable code. By using generics, we can create functions and data structures that work with any type, without sacrificing type safety. This leads to cleaner and more maintainable code, as we can avoid duplication and handle a wide variety of types with a single implementation.
// //  generics are widely used in Rust's standard library, and they are a fundamental part of the language. They enable us to write code that is both efficient and easy to read, making Rust a great choice for a wide range of applications.
// // generics for structs and enums
// struct Point<T> {
//     x: T,
//     y: T,
// }
// impl<T> Point<T> {
//     fn new(x: T, y: T) -> Self {
//         Point { x, y }
//     }
// }
// enum Option<T> {
//     Some(T),
//     None,
// }

// // multiple type parameters
// struct Pair<T, U> {
//     first: T,
//     second: U,
// }
// impl<T, U> Pair<T, U> {
//     fn new(first: T, second: U) -> Self {
//         Pair { first, second }
//     }
// }

// //  generic with  trait bounds

// fn  largest<T: PartialOrd>(a: T, b: T) -> T {
//     if a > b {
//         a
//     } else {
//         b
//     }
// }

// // hashmap example
// use std::collections::HashMap;




//  lets some exercises



// Write a function that swaps two values of any type
fn swap<T>(a: &mut T, b: &mut T) {
    // Your code - hint: use std::mem::swap
    mem::swap(a, b);
}


// Create generic Point<T> struct with x and y
// Add method distance_from_origin() that works only for f64

struct Point<T> {
    x: T,
    y: T,
}

impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        // Your code: sqrt(x² + y²)
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
use std::mem;
use std::collections::HashMap;
fn main() {
    let mut x = 5;
    let mut y = 10;
    swap(&mut x, &mut y);
    println!("x: {}, y: {}", x, y);  // x: 10, y: 5

     let p = Point { x: 3.0, y: 4.0 };
    println!("Distance: {}", p.distance_from_origin());

    //  creating a phone book
    let mut phone_book: HashMap<String, String> = HashMap::new();
    phone_book.insert("Alice".to_string(), "123-456-7890".to_string());
    phone_book.insert("Bob".to_string(), "987-654-3210".to_string());
    phone_book.insert("Charlie".to_string(), "555-555-5555".to_string());
    for (name, number) in &phone_book {
        println!("{}: {}", name, number);   }
        // print the phone number for Alice
        if let Some(number) = phone_book.get("Alice") {
            println!("Alice's number: {}", number);
        } else {
            println!("Alice's number not found");       }

            // print all  the phone book
            for (name, number) in &phone_book {
                println!("{}: {}", name, number);       } 

                let text = "hello";
    let counts = count_chars(text);
    println!("{:?}", counts);


}
//  charcater counter using a HashMap

fn count_chars(s: &str) -> HashMap<char, i32> {
    // Your code - count how many times each character appears
    let mut char_count: HashMap<char, i32> = HashMap::new();
    for c in s.chars() {
        *char_count.entry(c).or_insert(0) += 1; }

}