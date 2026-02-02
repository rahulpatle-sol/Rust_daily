fn main() {
    println!("Hello, world!");
    // common programi8ng concept ihn  rust
    // 
    // keywords
    //variable and mutablity
    // As default all variable are the  immutabnle 
    // the value could be changed as well
    // to change the value convert variable into mutable
    let mut x = 5;
    x = 6;
    println!("x is {}", x);
    
    // Constants
    // Constants are immutable by default
    // They must be annotated with a type
    // Constants can be declared in any scope, including global
    const MAX_POINTS: u32 = 100_000;
    println!("The maximum number of points is {}", MAX_POINTS);
    
    // Shadowing
    // Shadowing is a feature that allows you to declare a new variable with the same name as a previous variable
    // The new variable hides the previous variable
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("x is {}", x);
    
    // Shadowing with different types
    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces is {}", spaces);
    // Data types 
    // 
    // In Rust have 2 types of data types 
    // 1. Scalar value data types 
    // 2. Compound data types
    // 3. Scalar value data types
    // 
    // 
    // 
    // 
    let a:u8=32;
    let b:i32=324;  
    let c:f64=324.0;
    let d = 324;
    let char = 'a';
    let bool = true;
    let tuple = (1, 2, 3);
    //  tuple me (types in  tup)=(values )
    
    //  arry me [data types , amonut]=[ values as well]
    // Compound data types
    // 
    // 
    // 
    // 
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    
    // Array
    // 
    // 
    // 
    // 
    let arr = [1, 2, 3, 4, 5];
    let first = arr[0];
    let second = arr[1];
    println!("The first element is {} and the second element is {}", first, second);
    
    // Slices
    // 
    // 
    // 
    // 
    let slice = &arr[1..3];
    let slice2=&arr[4..5];
    let slice3=&arr[0..2];
    let slice4=&arr[2..4];
    println!("Slice: {:?}", slice);
    println!("Slice: {:?}", slice);
    println!("Slice: {:?}", slice);
    println!("Slice: {:?}", slice);


/*
 slice methods in action  
all  methods of slices
  // compund data types 
  // struct  ,vec, hashmaps, enums, trait , option, enums , etc 
  // cooming soon 
  // 
1) convert a aaray  into  a slicce

*/

      //  chapter 2 doe 
      // learn about the 
      // variable ,const
// mutable immutable
// data types
// 


//  some exericse 

let  age:u8=23;
let mut cur_year:u16=2026;
println!("my age is {},the current year {}",age,cur_year);

cur_year=2027;
print!("u[pdated year{}",cur_year);
// 2.2
let  mut ert:u8=34;
ert=23;
println!("the value of ert{}",ert);
// maybe it is overwritten before being read?
//  try  to  make it mutable varible using mut 

}