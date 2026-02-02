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


struct User {
    username: String,
    email: String,
    age: u32,
}

// fn main() {
//     let user1 = User {
//         username: String::from("rahul"),
//         email: String::from("rahul@example.com"),
//         age: 23,
//     };
    
//     // Create user2 with some values from user1
//     let user2 = User {
//         username: String::from("karan"),
//         ..user1  // Take rest from user1
//         // ... is awking the info  from  samew objection 
//     };
    
//     println!("User2 username: {}", user2.username);  // karan
//     println!("User2 email: {}", user2.email);  // rahul@example.com (from user1)
    
//     // NOTE: user1.email is MOVED to user2 (String doesn't implement Copy)
//     // println!("{}", user1.email);  // ❌ ERROR!
// }



//  tuple struct dude 
struct Color(i32, i32, i32);  // RGB
struct Point(i32, i32);       // X, Y

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0);
    
    // Access by index
    println!("Red: {}", black.0);
    println!("Green: {}", black.1);
    println!("Blue: {}", black.2);
}