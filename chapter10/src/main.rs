



// // // // ENUMS (Enumerations)
// // // // Enums let you define a type with multiple possible variants.
// // // // In Anchor:

// // // // Instructions = Enums (Initialize, Deposit, Withdraw)
// // // // Errors = Enums (InsufficientFunds, Unauthorized)
// // // // State = Enums (Active, Paused, Closed)




// // // enum Direction {
// // //     North,
// // //     South,
// // //     East,
// // //     West,
// // // }
// // // enum Message {
// // //     Quit,                       // No data
// // //     Move { x: i32, y: i32 },   // Named fields (like struct)
// // //     Write(String),              // Single value
// // //     ChangeColor(i32, i32, i32), // Tuple
// // // }
// // // fn main() {
// // //     let player_direction = Direction::West;
// // //     let enemy_direction = Direction::West;
    
// // //     // Use with match
// // //     match player_direction {
// // //         Direction::North => println!("Going North!"),
// // //         Direction::South => println!("Going South!"),
// // //         Direction::East => println!("Going East!"),
// // //         Direction::West => println!("Going West!"),
// // //     }



// // //      let msg1 = Message::Quit;
// // //     let msg2 = Message::Move { x: 10, y: 20 };
// // //     let msg3 = Message::Write(String::from("Hello"));
// // //     let msg4 = Message::ChangeColor(255, 0, 0);
    
// // //     process_message(msg2);
// // // }



// // enum Message {
// //     Quit,                       // No data
// //     Move { x: i32, y: i32 },   // Named fields (like struct)
// //     Write(String),              // Single value
// //     ChangeColor(i32, i32, i32), // Tuple
// // }

// // fn main() {
// //     let msg1 = Message::Quit;
// //     let msg2 = Message::Move { x: 10, y: 20 };
// //     let msg3 = Message::Write(String::from("Hello"));
// //     let msg4 = Message::ChangeColor(255, 0, 0);
    
// //     process_message(msg2);
// // }

// // fn process_message(msg: Message) {
// //     match msg {
// //         Message::Quit => {
// //             println!("Quit received");
// //         }
// //         Message::Move { x, y } => {
// //             println!("Move to x: {}, y: {}", x, y);
// //         }
// //         Message::Write(text) => {
// //             println!("Text: {}", text);
// //         }
// //         Message::ChangeColor(r, g, b) => {
// //             println!("Color: RGB({}, {}, {})", r, g, b);
// //         }
// //     }
// // }

// //  methods in  ennum 


// #[derive(Debug)]
// enum Traffic_light{
//     Red,
//     Yellow,
//     Green,

// }
// enum  Coin{
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }


// fn  value_in_cents(coin: Coin)->u32{
//     match coin{
//         Coin::Penny=>1,
//         Coin::Nickel=>5,
//         Coin::Dime=>10,
//         Coin::Quarter=>25,

//     }
// }   
// impl Traffic_light{
//     fn time_to_wait(&self)->u32{
//         match self{
//             Traffic_light::Red=>60,
//             Traffic_light::Yellow=>5,
//             Traffic_light::Green=>90,

//         }


//     }

//     fn can_go(&self)->bool{
//         match  self{
//             Traffic_light::Green=>true,
//             _=>false,

//         }

//     }
// }
// //
//  //  ip  address
//  enum  Ip_address{
//     V4(u32,u32,u32,u32),
//     V6(String),
//  }

//  //  match  with  conditions 
 
// fn main (){
//     let light = Traffic_light::Green;
//     println!("Wait time: {} seconds", light.time_to_wait());
//     println!("Can go? {}", light.can_go());

//     let coin = Coin::Dime;
//     println!("Value in cents: {}", value_in_cents(coin));
//     //  let my  home ip  address 

//     let home =Ip_address::V4(127,0, 9, 78);
//     let loopback=Ip_address::V6(String::from("ip  address configured successfully ::1 "));

// match loopback {
//         Ip_address::V4(a, b, c, d) => {
//             println!("IPv4: {}.{}.{}.{}", a, b, c, d);
//         }
//         Ip_address::V6(addr) => {
//             println!("IPv6: {}", addr);
//         }
//     }

// let number=23;
// match number {
//     1=>println!("one"),
//     2|3|5|7|11=>println!("prime  number is here "),
//     13..=23=> println!("teen  number"),
//     n if n%2==0=>println!("even number "),
//     _=>println!("some thing ele ")

    
// }




// }
// //  option  and result enum  as well  
// //  option have 2 things some and none that's enough




//  exercise 


// Create an enum Shape with variants:
// - Circle(f64) - stores radius
// - Rectangle(f64, f64) - stores width, height
// - Triangle(f64, f64, f64) - stores three sides

enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

// Add a method area(&self) -> f64 that calculates area
impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => std::f64::consts::PI * r * r,
            Shape::Rectangle(width, height) => width * height,
            Shape::Triangle(a, b, c) => {
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
    }
}
//  paymet method enum
enum PaymentMethod {
    Cash,
    CreditCard { number: String, cvv: u16 },
    UPI { id: String },
}
impl  PaymentMethod{
    fn process(&self,amount:f64){
        match self{
            PaymentMethod::Cash => {
                println!("Processing cash payment of ${}", amount);
            }
            PaymentMethod::CreditCard { number, cvv } => {
                println!("Processing card payment of ${} using card {}", amount, number);
            }
            PaymentMethod::UPI { id } => {
                println!("Processing UPI payment of ${} using ID {}", amount, id);
            }
        }
    }
}


// Write a function find_index(vec: &Vec<i32>, target: i32) -> Option<usize>
// Returns Some(index) if found, None otherwise

fn find_index(vec: &Vec<i32>, target: i32) -> Option<usize> {
    // Your code - loop through vec, return Some(index) when found
    for (i, &value) in vec.iter().enumerate() {
        if value == target {
            return Some(i);
        }
    }
    None
}
fn main() {
    let circle = Shape::Circle(5.0);
    let rect = Shape::Rectangle(10.0, 20.0);
    
    println!("Circle area: {}", circle.area());
    println!("Rectangle area: {}", rect.area());
    


    let cash = PaymentMethod::Cash;
    let card = PaymentMethod::CreditCard {
        number: String::from("1234-5678-9012-3456"),
        cvv: 123,
    };
    
    cash.process(100.0);
    card.process(500.0);
     let result1 = safe_divide(10.0, 2.0);
    let result2 = safe_divide(10.0, 0.0);
    println!("Result 1: {:?}", result1);
    println!("Result 2: {:?}", result2);

    //  find elemnt in vector 
     let numbers = vec![10, 20, 30, 40, 50];
    
    match find_index(&numbers, 30) {
        Some(idx) => println!("Found at index: {}", idx),
        None => println!("Not found"),
    }



    let mut light = TrafficLight::Red;
    
    for _ in 0..5 {
        println!("{:?} for {} seconds", light, light.duration());
        light = light.next();
    }
    let ops = vec![
         operation::Add(5.0, 3.0),
        operation::Subtract(10.0, 4.0),
        operation::Multiply(3.0, 7.0),
        operation::Divide(20.0, 0.0),  // Should return None!
    ];
    
    for op in ops {
        match op.execute() {
            Result::Ok(result) => println!("Result: {}", result),
            Result::Err(e) => println!("Error: {}", e),
        }
    }
}


// Write a function safe_divide(a: f64, b: f64) -> Option<f64>
// Returns Some(result) if b != 0, None otherwise

fn safe_divide(a: f64, b: f64) -> Option<f64> {
match  b{
    b if b!=0.0 =>Some(a/b),
    _=>None,    
}

}

//  enum  
  #[derive(Debug)]
  enum  TrafficLight{
    Red,
    Yellow,
    Green,
  }

impl TrafficLight {
    fn next(&self) -> TrafficLight {
        // Your code
        match self {
            TrafficLight::Red => TrafficLight::Green,
            TrafficLight::Yellow => TrafficLight::Red,
            TrafficLight::Green => TrafficLight::Yellow,
        }
    }
    
    fn duration(&self) -> u32 {
        // Red: 60, Yellow: 5, Green: 55
        // Your code
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 55,
        }
    }
}//
//  bonus 
enum Result<T,E>{
    Ok(T),
    Err(E),
}

enum  operation{
    Add(f64,f64),
    Subtract(f64,f64),
    Multiply(f64,f64),
    Divide(f64,f64),
}
impl  operation{
    fn execute(&self)->Result<f64,String>{
        match self{
            operation::Add(a,b)=>Result::Ok(a+b),
            operation::Subtract(a,b)=>Result::Ok(a-b),
            operation::Multiply(a,b)=>Result::Ok(a*b),
            operation::Divide(a,b)=>{
                if *b==0.0{
                    Result::Err(String::from("Cannot divide by zero"))
                }else{
                    Result::Ok(a/b)
                }
            }
        }
    }
}