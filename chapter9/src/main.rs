// // Define a struct
// struct User {
//     username: String,
//     email: String,
//     age: u32,
//     active: bool,
// }

// struct User23 {
//     username: String,
//     age: u32,
// }


// fn main() {
//     // Create an instance
//     let user1 = User {
//         username: String::from("rahul_patle"),
//         email: String::from("rahul@example.com"),
//         age: 23,
//         active: true,


        
//     };
//     let mut user_m=User23{
//         username:String::from("Harshit  bhai "),
//         age:34
//     };

    
//     // Access fields with dot notation
//     println!("Username: {}", user1.username);
//     println!("Email: {}", user1.email);
//     println!("Age: {}", user1.age);
//     //  
//     println!("Mutabel  user age {}",user_m.age);
//     println!("mutbale usr name is here{}",user_m.username);
//     user_m.age=45;
//     println!("Mutabel  user age after change  {}",user_m.age);


// }


// struct User {
//     username: String,
//     email: String,
//     age: u32,
// }

// // fn main() {
// //     let user1 = User {
// //         username: String::from("rahul"),
// //         email: String::from("rahul@example.com"),
// //         age: 23,
// //     };
    
// //     // Create user2 with some values from user1
// //     let user2 = User {
// //         username: String::from("karan"),
// //         ..user1  // Take rest from user1
// //         // ... is awking the info  from  samew objection 
// //     };
    
// //     println!("User2 username: {}", user2.username);  // karan
// //     println!("User2 email: {}", user2.email);  // rahul@example.com (from user1)
    
// //     // NOTE: user1.email is MOVED to user2 (String doesn't implement Copy)
// //     // println!("{}", user1.email);  // ❌ ERROR!
// // }



// //  tuple struct dude 
// struct Color(i32, i32, i32);  // RGB
// struct Point(i32, i32);       // X, Y

// fn main() {
//     let black = Color(0, 0, 0);
//     let origin = Point(0, 0);
    
//     // Access by index
//     println!("Red: {}", black.0);
//     println!("Green: {}", black.1);
//     println!("Blue: {}", black.2);
// }

// Add this attribute to enable debug printing
// #[derive(Debug)]
// struct User {
//     username: String,
//     age: u32,
// }

// fn main() {
//     let user = User {
//         username: String::from("rahul"),
//         age: 23,
//     };
    
//     // Pretty print
//     println!("{:?}", user);    // Single line
//     println!("{:#?}", user);   // Multi-line (pretty)
// }

//  methods in  stutcs 


//  basic menthods




// Create a struct Person with:
// - name: String
// - age: u32
// - city: String
// 
// Create 2 instances and print them using Debug

#[derive(Debug)]
struct Person {
    // Your code
    name: String,
    age:u8,
    address:String,
    income:u32,


}
// / Create a BankAccount struct with:
// // - account_number: u32
// // - balance: f64
// //
// // Make it mutable and add/subtract from balance
  struct BankAccount{
    account_number:u32,
    balance:f64,

  }

//   / Create Rectangle struct with width and height
// // Add these methods:
// // - area() -> u32
// // - is_square() -> bool (returns true if width == height)
// // - scale(factor: u32) -> changes width and height

struct Rectangle{
    width:f64,
    height:f64,

}

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }



    fn is_square (&self)->bool{

        self.height==self.width

    }
    // If you want to return both width and height as a tuple:
    fn scale(&self) -> (f64, f64) {
        (self.width, self.height)
    }

    // Alternatively, if you want to scale the rectangle in place, use:
    // fn scale(&mut self, factor: f64) {
    //     self.width *= factor;
    //     self.height *= factor;
    // }
}



// Create Temperature struct with celsius: f64
// Add these methods:
// - new(celsius: f64) -> Temperature (associated function)
// - to_fahrenheit(&self) -> f64
// - to_kelvin(&self) -> f64
struct Temperature {
    // Your code
    celsius:f64,

}

impl Temperature {
    // Your code
    fn new(celsius:f64)->Temperature{
        Temperature{
            celsius,
        }
    }
    fn to_fahrenheit(&self)->f64{
        (self.celsius * 9.0/5.0) +32.0
    }
    fn to_kelvin(&self)->f64{
        self.celsius + 273.15
    }


}
// Create Counter struct with count: i32
// Add methods:
// - new() -> Counter (starts at 0)
// - increment(&mut self)
// - decrement(&mut self)
// - reset(&mut self)
// - get(&self) -> i32

struct Counter {
    // Your code
    count:i32,

}

impl Counter {
    // Your code
    fn new()->Counter{
        Counter{
            count:0,
        }
    }
    fn increment(&mut self){
        self.count +=1;
    }
    fn decrement(&mut self){
        self.count -=1;
    }
    fn reset(&mut self){
        self.count=0;
    }
    fn get(&self)->i32{
        self.count
    }

}

// BONUS Challenge:
// Create a Circle struct with radius: f64 and these methods:

// new(radius: f64) -> Circle
// area(&self) -> f64
// circumference(&self) -> f64
// compare(&self, other: &Circle) -> String - returns "Bigger", "Smaller", or "Equal"


struct Circle {
    radius:f64,
}
impl Circle {
    fn new(radius:f64)->Circle{
        Circle{
            radius,
        }
    }
    fn area(&self)->f64{
        3.14 * self.radius * self.radius
    }
    fn circumference(&self)->f64{
        2.0 * 3.14 * self.radius
    }
    fn compare(&self,other:&Circle)->String{
        if self.radius > other.radius {
            String::from("Bigger")
        } else if self.radius < other.radius {
            String::from("Smaller")
        } else {
            String::from("Equal")
        }
    }
}

fn main() {
    

    let Person1=Person{
        name:String::from("rahal"),
        age:23,
        address:String::from("indore"),
        income:23223,
    };
    println!("name is {}",Person1.name);

    let mut  Account1=BankAccount{
        account_number:232324224,
        balance:4544.678,

    };
    Account1.balance=3433454.56;

    println!("new balance is {}",Account1.balance);
     let rect1=Rectangle{
        width:34.5,
        height:34.5,

     };
        println!("area of rectangle is {}",rect1.area());
        println!("is rectangle square  {}",rect1.is_square());
        let (w,h)=rect1.scale();
        println!("width is {} and height is {}",w,h);
        let new_rect=Rectangle{
            width:w*2.0,
            height:h*2.0,
        };
        println!("new rectangle width is {} and height is {}",new_rect.width,new_rect.height);
let temp = Temperature::new(25.0);
    println!("Celsius: {}", temp.celsius);
    println!("Fahrenheit: {}", temp.to_fahrenheit());
    println!("Kelvin: {}", temp.to_kelvin());

    let mut counter = Counter::new();
    counter.increment();
    counter.increment();
    counter.increment();
    println!("Count: {}", counter.get());  // 3
    
    counter.decrement();
    println!("Count: {}", counter.get());  // 2
    
    counter.reset();
    println!("Count: {}", counter.get());  // 0


    let circle1 = Circle::new(5.0);
    let circle2 = Circle::new(7.0);
    println!("Circle 1 Area: {}", circle1.area());
    println!("Circle 1 Circumference: {}", circle1.circumference());
    println!("Circle 1 is {}", circle1.compare(&circle2));  // Smaller
}