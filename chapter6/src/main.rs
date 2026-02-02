// fn main() {
//     // Infinite loop (use 'break' to exit)
//     let mut counter = 0;
//     loop {
//         counter += 1;
//         println!("Counter: {}", counter);
        
//         if counter == 5 {
//             break;  // Exit the loop
//         }
//     }
    
//     // Loop with return value
//     let mut num = 0;
//     let result = loop {
//         num += 1;
//         if num == 10 {
//             break num * 2;  // Return value from loop
//         }
//     };
//     println!("Result: {}", result);  // 20
    
//     // While loop
//     let mut countdown = 5;
//     while countdown > 0 {
//         println!("{}...", countdown);
//         countdown -= 1;
//     }
//     println!("Liftoff!");
    
//     // Continue keyword (skip current iteration)
//     let mut n = 0;
//     while n < 10 {
//         n += 1;
//         if n % 2 == 0 {
//             continue;  // Skip even numbers
//         }
//         println!("Odd: {}", n);
//     }



// //     loop = infinite loop, must use break to exit
// // break can return a value: break value;
// // while condition { } = loop while condition is true
// // continue = skip rest of current iteration, go to next
// // break = exit loop completely




//     // For loop with range (0 to 4, excludes 5)
//     for i in 0..5 {
//         println!("i = {}", i);  // 0, 1, 2, 3, 4
//     }
    
//     // Inclusive range (includes 5)
//     for i in 0..=5 {
//         println!("i = {}", i);  // 0, 1, 2, 3, 4, 5
//     }
    
//     // Reverse loop
//     for i in (1..=5).rev() {
//         println!("Countdown: {}", i);  // 5, 4, 3, 2, 1
//     }
    
//     // Loop through array
//     let numbers = [10, 20, 30, 40, 50];
//     for num in numbers {
//         println!("Number: {}", num);
//     }
    
//     // Loop with index
//     for (index, value) in numbers.iter().enumerate() {
//         println!("Index {}: Value {}", index, value);
//     }
    
//     // Nested loops
//     for i in 1..=3 {
//         for j in 1..=3 {
//             print!("{} ", i * j);
//         }
//         println!();  // New line after each row
//     }
//     // Output:
//     // 1 2 3
//     // 2 4 6
//     // 3 6 9
// }


//  exercise time  
//Sum calculator

// use a while loop 
// sull all the nmber



fn main(){


let mut num=0;
let mut sum=0;

while num<=100{
 
   sum=sum+num;
    num+=1;


}


println!("sum is{}",sum);


//  a  tyr and go  
 let _sceret=12;
 let mut counter=0;
 loop {

    counter+=1;

if counter==_sceret{
    break;
    
}


}
println!("yupp  got the secret*****");



//  multipliocation tavbble
for i in 0..11{
    println!("I= {}",i*5)
}
//  for even number

for i in 1..20{
    if i%2==0{
        println!("{}",i);
    }
}
// factorial 
let mut fact=1;
for i in 1..6{
    fact=fact*i;    
}
println!("factorial is {}",fact);




for i in 1..=30 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");  // Divisible by BOTH 3 AND 5
        }
        else if i % 3 == 0 {
            println!("Fizz");      // Divisible by 3
        }
        else if i % 5 == 0 {
            println!("Buzz");      // Divisible by 5
        }
        else {
            println!("{}", i);     // Just the number
        }
    }
}