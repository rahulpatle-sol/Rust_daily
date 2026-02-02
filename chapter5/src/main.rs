// fn main() {
//     // Calling functions
//     greet();
//     greet_person("Rahul");
    
//     let result = add(5, 3);
//     println!("5 + 3 = {}", result);
    
//     let (sum, product) = calculate(10, 2);
//     println!("Sum: {}, Product: {}", sum, product);
// }

// // Function with no parameters, no return value
// fn greet() {
//     println!("Hello from a function!");
// }

// // Function with parameters
// fn greet_person(name: &str) {
//     println!("Hello, {}!", name);
// }

// // Function that returns a value
// fn add(a: i32, b: i32) -> i32 {
//     a + b  // No semicolon! This is the return value
//     // Or you can use: return a + b;
// }

// // Function returning multiple values (tuple)
// fn calculate(x: i32, y: i32) -> (i32, i32) {
//     let sum = x + y;
//     let product = x * y;
//     (sum, product)  // Return tuple
// }

// // Function with early return
// fn is_even(num: i32) -> bool {
//     if num % 2 == 0 {
//         return true;  // Early return with 'return' keyword
//     }
//     false  // Last expression is returned


// //     fn keyword to declare functions
// // Parameters need types: name: &str
// // Return type after ->: -> i32
// // Last expression (without ;) is the return value
// // Or use return keyword explicitly
// // Can return tuples for multiple values
// }

// fn main() {
//     let number = 7;
    
//     // Basic if/else
//     if number > 5 {
//         println!("Number is greater than 5");
//     } else {
//         println!("Number is 5 or less");
//     }
    
//     // if/else if/else
//     let score = 85;
//     if score >= 90 {
//         println!("Grade: A");
//     } else if score >= 80 {
//         println!("Grade: B");
//     } else if score >= 70 {
//         println!("Grade: C");
//     } else {
//         println!("Grade: F");
//     }
    
//     // Multiple conditions with && (AND) and || (OR)
//     let age = 20;
//     let has_id = true;
    
//     if age >= 18 && has_id {
//         println!("You can enter");
//     } else {
//         println!("Access denied");
//     }
    
//     // if in a let statement (if is an expression!)
//     let condition = true;
//     let number = if condition { 5 } else { 6 };
//     println!("Number is: {}", number);
    
//     // Comparison operators
//     let x = 10;
//     let y = 20;
    
//     println!("x == y: {}", x == y);  // Equal
//     println!("x != y: {}", x != y);  // Not equal
//     println!("x < y: {}", x < y);    // Less than
//     println!("x > y: {}", x > y);    // Greater than
//     println!("x <= y: {}", x <= y);  // Less or equal
//     println!("x >= y: {}", x >= y);  // Greater or equal
// }


// exercise 

fn main() {
    // Test temperature conversion
    let temp = celsius_to_fahrenheit(25.0);
    println!("25°C = {}°F", temp);
    
    // Test calculator functions
    println!("5 + 3 = {}", add(5, 3));
    println!("5 - 3 = {}", subtract(5, 3));
    
    // Test age checker
    check_age(10);
    check_age(16);
    check_age(25);
    
    // Test grade
    let grade = get_grade(85);
    println!("Grade: {:?}", grade);
    
    // Test leap year
    println!("2024 is leap: {}", is_leap_year(2024));
}


fn celsius_to_fahrenheit(celcius:f64)->f64{

    let ferenhite=celcius*9.0/5.0+32.0;
    return ferenhite;
}


fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn divide(a: i32, b: i32) -> i32 {
    a / b
}


fn check_age(age:u32){


    if age<13 {
    println!("you  are child");

}
else if age >=13 && age <=19 {
    println!("you are teenager");
}

else{
    print!("you  are adult");
}
}
fn get_grade(score:i32)->char{
  if score>=90{
   return 'A';

  }
  else if score>=80 {
return 'B';


  }
  else if score>=70 {

return 'C';
  }
 
  else {
     
return 'F';
  }
}
fn is_leap_year(year: i32) -> bool {
    if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
        return true;
    } else {
        return false;
    }
    // Or shorter: year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}
