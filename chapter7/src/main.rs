// fn main() {
//     // Declare array with type and size
//     let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    
//     // Or let Rust infer the type
//     let fruits = ["apple", "banana", "orange"];
    
//     // Access elements (index starts at 0)
//     println!("First: {}", numbers[0]);   // 1
//     println!("Third: {}", numbers[2]);   // 3
    
//     // Get array length
//     println!("Length: {}", numbers.len());  // 5
    
//     // Create array with same value repeated
//     let zeros = [0; 10];  // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    
//     // Modify array (must be mutable)
//     let mut scores = [10, 20, 30];
//     scores[1] = 25;  // Change second element
//     println!("Scores: {:?}", scores);  // [10, 25, 30]
    
//     // Loop through array
//     for num in numbers {
//         println!("Value: {}", num);
//     }
    
//     // Loop with index
//     for i in 0..numbers.len() {
//         println!("Index {}: {}", i, numbers[i]);
//     }
    
//     // Using iter() and enumerate()
//     for (index, value) in numbers.iter().enumerate() {
//         println!("numbers[{}] = {}", index, value);
//     }
// }


// exercise time 


// Exercise 11.1: Array basics

// Create an array of your 5 favorite numbers
// Print the first and last element
// Calculate and print the sum of all elements using a loop

fn main() {
let my_favorite_numbers = [7, 3, 42, 15, 8];
println!("First favorite number: {}", my_favorite_numbers[0]);
println!("Last favorite number: {}", my_favorite_numbers[4]);
let mut sum=0;
for num in my_favorite_numbers.iter(){
    sum+=num;

}
println!("Sum of favorite numbers: {}", sum);


// Exercise 11.2: Array modification



// Exercise 11.2: Array search

// Create an array: [10, 25, 30, 45, 50]
// Use a loop to find if 30 exists
// Print "Found!" or "Not found!"


let array2=[10,25,30,45,50];
for element in array2.iter(){
    if *element==30{
        println!("hello  got the num {}",element)
    }
}


// ecercise 3
// Exercise 12.1: Vector operations

// Create an empty vector
// Add numbers 1 to 10 using a loop and .push()
// Print the vector
// Remove the last 3 elements using .pop()
// Print the final vector


let mut vec1:Vec<i32>=Vec::new();
for i in 0..=10{
    vec1.push(i);

}

println!("first final vector {:?}",vec1);

for j in 0..=3{
    vec1.pop();
}

println!("the secon poped vector{:?}",vec1);


// Exercise 12.2: Vector filter

// Create a vector: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
// Create a NEW vector with only even numbers from the first vector
// Hint: Loop through first vector, if even, push to new vector
// Print both vectors

let vector = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
let mut even_vec = vec![];
for num in vector.iter(){
    if *num %2==0{
        even_vec.push(*num);
    }



}
println!("Original vector: {:?}", vector);
println!("Even numbers vector: {:?}", even_vec);

// Exercise 12.3: Grade manager

// Create a vector to store test scores
// Add 5 scores: 85, 92, 78, 90, 88
// Calculate and print the average
// Find and print the highest score


let mut Scores=vec![];
Scores.push(85);
Scores.push(92);
Scores.push(78);
Scores.push(90);
Scores.push(88);
let mut total=0;
let mut highest=0;
for score in Scores.iter(){
    total+=*score;
    if *score>highest{
        highest=*score; }



}


// BONUS Challenge:
// Create a function remove_duplicates(v: Vec<i32>) -> Vec<i32>

// Takes a vector with duplicates: vec![1, 2, 2, 3, 4, 4, 5]
// Returns a vector without duplicates: vec![1, 2, 3, 4, 5]
// Hint: Loop through input, check if number already exists in result before adding

// Print the result
remove_duplicates();


}

fn remove_duplicates(){

  let uplicates=vec![1, 2, 2, 3, 4, 4, 5];
  let mut pure=vec![];
  for num in uplicates.iter(){
    if !pure.contains(num){
        pure.push(*num);
    }
  }
  pure;


}

// fn main() {
//     // Create empty vector
//     let mut v1: Vec<i32> = Vec::new();
    
//     // Create with initial values (easier way)
//     let mut numbers = vec![1, 2, 3, 4, 5];
    
//     // Add elements
//     numbers.push(6);      // Add to end
//     numbers.push(7);
//     println!("After push: {:?}", numbers);  // [1, 2, 3, 4, 5, 6, 7]
    
//     // Remove last element
//     let last = numbers.pop();  // Returns Option<i32>
//     println!("Removed: {:?}", last);  // Some(7)
    
//     // Access elements
//     println!("First: {}", numbers[0]);
//     println!("Third: {}", numbers[2]);
    
//     // Safe access with .get() (returns Option)
//     match numbers.get(2) {
//         Some(value) => println!("Third element: {}", value),
//         None => println!("No element at index 2"),
//     }
    
//     // Get length
//     println!("Length: {}", numbers.len());
    
//     // Check if empty
//     println!("Is empty: {}", numbers.is_empty());
    
//     // Loop through vector
//     for num in &numbers {  // &numbers = borrow (we'll learn this soon!)
//         println!("Value: {}", num);
//     }
    
//     // Modify while looping
//     for num in &mut numbers {  // &mut = mutable borrow
//         *num *= 2;  // Double each value
//     }
//     println!("Doubled: {:?}", numbers);
    
//     // Insert at specific position
//     numbers.insert(0, 99);  // Insert 99 at index 0
//     println!("After insert: {:?}", numbers);
    
//     // Remove at specific position
//     numbers.remove(0);  // Remove first element
//     println!("After remove: {:?}", numbers);
    
//     // Clear all elements
//     numbers.clear();
//     println!("After clear: {:?}", numbers);  // []
// }