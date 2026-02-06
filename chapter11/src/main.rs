


// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// fn main() {
//     let x: Result<i32, String> = Result::Ok(42);
//     let y: Result<i32, String> = Result::Err(String::from("An error occurred"));

//     match x {
//         Result::Ok(value) => println!("Success: {}", value),
//         Result::Err(err) => println!("Error: {}", err),
//     }

//     match y {
//         Result::Ok(value) => println!("Success: {}", value),
//         Result::Err(err) => println!("Error: {}", err),
//     }
//     match read_number_from_file("number.txt") {
//         Ok(n) => println!("The number is: {}", n),
//         Err(e) => println!("Failed to read number: {}", e),
//     }

// }

// fn read_number_from_file(path: &str) -> Result<i32, String> {
//     let content = match std::fs::read_to_string(path) {
//         Ok(c) => c,
//         Err(e) => return Err(e.to_string()),
//     };
    
//     let number = match content.trim().parse::<i32>() {
//         Ok(n) => n,
//         Err(e) => return Err(e.to_string()),
//     };
    
//     Ok(number)
// }

// Write a function safe_sqrt(x: f64) -> Result<f64, String>
// Return Err if x is negative
// Return Ok(result) otherwise

fn safe_sqrt(x: f64) -> Result<f64, String> {
    if x < 0.0 {
        Err(String::from("Cannot compute square root of a negative number"))
    } else {
        Ok(x.sqrt())
    }
}


fn parse_age(age_str: &str) -> Result<u32, String> {
    match age_str.parse::<u32>() {
        Ok(age) => {
            if age > 120 {
                Err(String::from("Age cannot be greater than 120"))
            } else {
                Ok(age)
            }
        },
        Err(_) => Err(String::from("Invalid age format")),
    }
}
fn main() {
    match safe_sqrt(16.0) {
        Ok(result) => println!("√16 = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    match safe_sqrt(25.0) {
        Ok(result) => println!("√25= {}", result),
        Err(e) => println!("Error: {}", e),
    }


     let ages = vec!["25", "150", "abc", "30"];
    
    for age_str in ages {
        match parse_age(age_str) {
            Ok(age) => println!("Valid age: {}", age),
            Err(e) => println!("Invalid: {}", e),
        }
    }
let number_result = get_number();
match number_result {
    Ok(num) => println!("Got number: {}", num),
    Err(e) => println!("Error: {}", e),
}
    let result = double_if_positive();
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(e) => println!("Error: {}", e), }



 let balance = 100.0;
 let amount = 150.0;
 match withdraw(balance, amount) {
    Ok(new_balance) => println!("Withdrawal successful. New balance: ${}", new_balance),
    Err(e) => println!("Withdrawal failed: {:?}", e),

}}

// Create two functions that return Result
// Use ? to chain them together

fn get_number() -> Result<i32, String> {
    Ok(42)
}

fn double_if_positive() -> Result<i32, String> {
    let num = get_number()?;  // Use ? here
    
    if num < 0 {
        Err(String::from("Number is negative"))
    } else {
        Ok(num * 2)
    }
}

// In Anchor, you'll write:
// You need to do this one:
#[derive(Debug)]
enum BankError {
    InsufficientFunds,
    AccountNotFound,
    InvalidAmount,
}

fn withdraw(balance: f64, amount: f64) -> Result<f64, BankError> {
 
 
 // first  account for invalid amount, then insufficient funds
 if balance < 0.0 {
    return Err(BankError::AccountNotFound);
 }
 if amount <=0.0{
    return Err(BankError::InvalidAmount);
 }
 if amount > balance {
    Err(BankError::InsufficientFunds)
 }
 
 else {
    Ok(balance - amount)
 }

}