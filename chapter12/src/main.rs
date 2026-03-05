// trait Describe {
//     fn des(&self) -> String;
// }

// struct Developer {
//     name: String,
//     age: u32,
//     skill: String,
// }

// impl Describe for Developer {
//     fn des(&self) -> String {
//         format!(
//             "{} is {} years old and skilled in {}",
//             self.name, self.age, self.skill
//         )
//     }
// }

// struct Book {
//     title: String,
//     pages: u32,
// }

// impl Describe for Book {
//     fn des(&self) -> String {
//         format!("{} has {} pages", self.title, self.pages)
//     }
// }

// fn main() {
//     println!("Trait example");

//     let ajit = Developer {
//         name: String::from("Ajit Singh Ranhaa"),
//         age: 22,
//         skill: String::from("Rust, JavaScript, TypeScript, React.js"),
//     };

//     println!("{}", ajit.des());

//     let math_book = Book {
//         title: String::from("Math for Developers"),
//         pages: 346,
//     };

//     println!("{}", math_book.des());
//     let circle = Circle { radius: 5.0 };
//     let rect = Rectangle { width: 10.0, height: 20.0 };
    
//     println!("Circle: {}", circle.description());
//     println!("Rectangle: {}", rect.description());

//     // new trait as function partaamneter 

//  let user = User { name: String::from("Rahul") };
//     let product = Product { 
//         name: String::from("Laptop"),
//         price: 999.99,
//     };
    
//     display_item1(&user);
//     display_item1(&product)
    
// }

// //  trait with  multiple methods 

// trait Shape{
//     fn area(&self) -> f64;
//     fn perimeter(&self) -> f64;

//     fn description(&self) -> String {
//         format!("Area: {}, Perimeter: {}", self.area(), self.perimeter())
//     }
// }

// struct Circle{
//     radius: f64,
// }
// impl Shape for Circle{
//     fn area(&self) -> f64 {
//         std::f64::consts::PI * self.radius * self.radius
//     }
//     fn perimeter(&self) -> f64 {
//         2.0 * std::f64::consts::PI * self.radius
//     }
// }

// struct Rectangle{
//     width: f64,
//     height: f64,
// }
// impl Shape for Rectangle{
//     fn area(&self) -> f64 {
//         self.width * self.height
//     }
//     fn perimeter(&self) -> f64 {  
//         2.0 * (self.width + self.height)
//     }
// }

// //  trait as fuction paramneter

// trait Printable{
//     fn print(&self);

// }

// struct laptop{
//     name:String,

// }

// impl Printable for laptop{
//     fn print(&self){
//         println!("laptop name is {}",self.name);

//     }
// }

// struct product {
//     name:String,
//     price:f64,

// }
// impl  Printable for product{
//     fn print(&self){
//     println!("the product is name  here :: {} ... price : {}",self.name,self.price )
//     }
// }
// // 
//    //  creasting a function to desplay  the value and thier 

//    fn desplay_item1(item:&impl Printable){
//     item.print();


//    }












//  exercise


// Create a trait Animal with method speak(&self) -> String
// Implement for Dog (returns "Woof!") and Cat (returns "Meow!")
#[derive(Debug, Clone)]
trait Animal {
    fn speak(&self) -> String;
}

struct Dog {
    name: String,
}

struct Cat {
    name: String,
}

// Your impl blocks here
impl  Animal for Dog{
    fn speak(&self) -> String {
        format!(" dog is barking name {} is speaks woof",self.name)
    }
}
impl Animal for Cat{
    fn speak(&self) -> String {

 println!(" cat  is  cute name {} meow",self.name)
    }
}
fn main() {
    let dog = Dog { name: String::from("Buddy") };
    let cat = Cat { name: String::from("Whiskers") };
    
    println!("{} says: {}", dog.name, dog.speak());
    println!("{} says: {}", cat.name, cat.speak());

    //  2 
    let a = Number { value: 10 };
    let b = Number { value: 5 };
    
    let sum = a.add(&b);
    let diff = a.subtract(&b);
    
    println!("Sum: {}", sum.value);
    println!("Diff: {}", diff.value);


    //  3 
    let book1 = Book {
        title: String::from("Rust Book"),
        pages: 500,
    };
    
    // Test clone
    let book2 = book1.clone();
    
    // Test debug print
    println!("{:?}", book1);
    
    // Test equality
    if book1 == book2 {
        println!("Books are the same!");
    } else {
        println!("Books are different!");
    }


    //  4
    let article = Article {
        title: String::from("Rust Traits"),
        author: String::from("Rahul"),
    };
    
    let tweet = Tweet {
        username: String::from("@rustacean"),
        content: String::from("Traits are awesome!"),
    };
    
    print_summary(&article);
    print_summary(&tweet);


    //  bonus 
    let card = CreditCard { number: String::from("1234") };
    let upi = UPI { id: String::from("user@upi") };
    
    match card.process_payment(5000.0) {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("Error: {}", e),
    }
    
    match upi.process_payment(60000.0) {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("Error: {}", e),
    }
}

trait Calculatable {
    // Your methods
    fn  add(&self)->f64;
    fn  subtract(&self)->f64;

}

struct Number {
    value: i32,
}

impl Calculatable for Number {
    // Your implementation

    fn add(&self)->f64{
        println!(" addition is : {}",self.value + self.value);
        (self.value + self.value) as f64
    }
    fn subtract(&self)->f64{
        println!(" subtraction is : {}",self.value - self.value);
        (self.value - self.value) as f64
    }
}


struct Book {
    title: String,
    pages: u32,
}


// Create trait Summary with method summarize(&self) -> String
// Implement for Article and Tweet
// Create a function that accepts any type implementing Summary




trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    title: String,
    author: String,
}

struct Tweet {
    username: String,
    content: String,
}
// Your impl blocks here
impl Summary for Article{
    fn summarize(&self) -> String {
        format!("Article: '{}' by {}",self.title,self.author)
    }       }

fn print_summary(item: &impl Summary) {
    println!("{}", item.summarize());
}


// Create trait PaymentProcessor with:
// - process_payment(&self, amount: f64) -> Result<String, String>
//
// Implement for CreditCard and UPI
// CreditCard: succeeds if amount < 10000
// UPI: succeeds if amount < 50000

trait PaymentProcessor {
    fn process_payment(&self, amount: f64) -> Result<String, String>;
}

struct CreditCard {
    number: String,
}

struct UPI {
    id: String,
}
//  impl 
impl PaymentProcessor for CreditCard {
    fn process_payment(&self, amount: f64) -> Result<String, String> {
        if amount < 10000.0 {
            Ok(format!("CreditCard payment of {} processed.", amount))
        } else {
            Err(String::from("CreditCard payment failed: amount exceeds limit."))
        }
    }
}
impl PaymentProcessor for UPI {
    fn process_payment(&self, amount: f64) -> Result<String, String> {
        if amount < 50000.0 {
            Ok(format!("UPI payment of {} processed.", amount))
        } else {
            Err(String::from("UPI payment failed: amount exceeds limit."))
        }
    }
}
fn safe_sqrt(value: f64) -> Result<f64, String> {
    if value < 0.0 {
        Err(String::from("Cannot compute square root of negative number"))
    } else {
        Ok(value.sqrt())
    }
}
fn double_if_positive() -> Result<f64, String> {
    let num = 10.0; // Example number
    if num > 0.0 {
        Ok(num * 2.0)
    } else {
        Err(String::from("Number is not positive"))
    }
}