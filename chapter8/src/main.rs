// // // Ownership is what makes Rust unique! No other language has this.
// // // The 3 Rules of Ownership:

// // // Each value has ONE owner
// // // There can only be ONE owner at a time
// // // When the owner goes out of scope, the value is dropped (freed from memory)






// // fn main (){
    


// //     //  simple ownership 
// //     let s = String::from("hello"); // s is the owner of the String value
// //     println!("{}", s); // we can use s here
// //     let s1=s; // ownership of the String value is moved to s1
// //     // println!("{}", s); // error! s is no longer the owner
// //     println!("{}", s1); // we can use s1 here

// //     //copy vs move 
// //     let x=45;
// //     let y=x;
// //     //  value moved here 
// //     println!("x: {}, y: {}", x, y); // both x and y can be used here because integers are Copy types
   
// // let num34=23;
// // copy(num34);
// // take_ownership(s1);


// // let data1=gives_own();
// // let data2=String::from("the new data is here ");
// // let out=take_g_own(data2);


// // }

// // //  ownership  and functions
// // fn take_ownership(input:String){
// // println!("sone onput {}",input)
// // }

// // fn copy(numerucial:i32){

// //     print!("{}",numerucial)

// // }


// // //  ownership  valus Transsfer ownership
// // fn gives_own()->String{
// //     let random_str=String::from("my ownership dta asas wello ");
// //     random_str
// // }

// // fn take_g_own(a_str:String)->String{

// // a_str

// // }

// // refrence and Borrowin
// // Moving ownership every time is annoying! What if we just want to "look" at data?
// // Solution: Use references (&) to borrow data without taking ownership!

// fn main(){


// let s1=String::from("hello  data");

// let len=calculate_len(&s1);

// println!("lenth of string:       {}",len);

// let mut m1=String::from("mutbale ref is ued her new data..");
// change(&mut m1);
// println!("the m1 is chbaged lets see{}",m1);


// }

// fn calculate_len(s:&String)->usize{
//     s.len()
// }




// // Mutrable refrence
// fn change(some_string:&mut String){
//     some_string.push_str("new data changed iser mutable ref");
// }


//  exercises
fn main() {
    let s = String::from("Rust");
    print_string(&s);
    println!("{}", s);
    
    
     let x = 5;
    let y = x;
    println!("{} {}", x, y);
 let text = String::from("Hello Rust");
    let len = string_length(&text);  // Fix this
    println!("'{}' is {} characters", text, len);


     let mut lang = String::from("Rust");
    add_programming(&mut lang);  // Fix this
    println!("{}", lang);  
    // Should print "Rust Programming"

    let  s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} {}", r1, r2);
}

fn print_string(text: &String) {
    println!("{}", text);






    
}//  
//  exercise 2
fn string_length(input:&String)->usize{
    input.len()

}fn add_programming(data:&mut String){
    data.push_str("Programin")

}