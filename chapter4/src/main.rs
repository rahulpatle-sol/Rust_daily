// fn main() {
//     // String slice (&str) - immutable, fixed size, stored in program binary
//     let name: &str = "Rahul";
    
//     // String - growable, mutable, stored on heap
//     let mut greeting = String::from("Hello");
    
//     // You can grow a String
//     greeting.push_str(", world!");
//     greeting.push('!');  // push single char
    
//     println!("{}", greeting);  // Hello, world!!
    
//     // Concatenation
//     let first = String::from("Rust");
//     let second = String::from(" is awesome");
//     let combined = first + &second;  // Note: first is moved here!
//     println!("{}", combined);
    
//     // Format macro (doesn't move values)
//     let lang = "Rust";
//     let year = 2026;
//     let message = format!("{} in {}", lang, year);
//     println!("{}", message);
    
//     // String length
//     let text = "Hello";
//     println!("Length: {}", text.len());  // 5
    
//     // String contains
//     let sentence = "I love Rust programming";
//     println!("Contains 'Rust': {}", sentence.contains("Rust"));  // true



// }

use std::fmt::format;



fn main (){

    /* this is my  main  part comes here 
    * i haope d  you  like 
    
    */ 
    let mut my_name:String=String::from("Rahul");
    my_name.push_str("Patle");
    println!("my name is : {}",my_name);
    //  string operation 
    let my_food:&str="Rabadi";
    let my_drink:&str="Mango Juice";
    let combo=format!("I like {},and {}",my_food,my_drink);
    let some=combo.contains("Mango Juice.");
    println!("my info{}",combo);
    println!(" combo  contains {}",some);

    // mad Libs typo
    //  In  this example i have created a mad labs style 
    let  name="karan ";
    let adjective="is walkman";
    let  noun="police";
    let verb="works as ";
    let merge=format!("the merge is {},{},{},{}",name,adjective,verb,noun);
    println!("thre final is {}",merge);

}