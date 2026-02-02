// fn main() {
//     println!("function guide in  rust");
//     println!("function guide in  rust");
//     function_guide();
//     function_guide2();
//     let result = mark(5);
//     println!("Result: {}", result);
//     function_guide3(1,2);
//     function_guide4(1,2,3);
//     function_guide5(1,2,3,4);
//     function_guide6(1,2,3,4,5);
//     auth("password");
//     print!("{},{}", auth2("password"),auth("password"));
//     // loops
//     // 
//     // while loop
//     // 
//     let mut count = 0;
//     while count < 5 {
//         println!("Count: {}", count);
//         count += 1;
//     }
//     // for loop
//     // 
//     for i in 0..5 {
//         println!("Count: {}", i);
//     }
//     // infinite loop
//     // 
//     // loop {
//     //     println!("Infinite loop");
//     // }
//     // break and continue
//     // 
//     let mut count = 0;
//     while count < 5 {
//         if count == 3 {
//             break;
//         }
//         println!("Count: {}", count);
//         count += 1;
//     }
//     // match
//     // 
//     let x = 5;
//     match x {
//         1 => println!("One"),
//         2 => println!("Two"),
//         _ => println!("Other"),
//     }
//     // looping with  tup,arry
    
//     mutIter();
//     tup();
    
// }



// fn function_guide() {
//     println!("function guide in  rust");
//     println!("function guide in  rust");
    
// }

// fn function_guide2() {
//     println!("function guide in  rust");
//     println!("function guide in  rust");
    
// }
// fn mark(s:i32)->i32{
//     s+1
    
// }
// // function with  multiparameter 
// // 
// fn function_guide3(a:i32,b:i32)->i32{
//     a+b
    
// }

// fn function_guide4(a:i32,b:i32,c:i32)->i32{
//     a+b+c
    
// }
// fn function_guide5(a:i32,b:i32,c:i32,d:i32)->i32{
//     a+b+c+d
    
// }
// fn function_guide6(a:i32,b:i32,c:i32,d:i32,e:i32)->i32{
//     a+b+c+d+e
    
// }

// //  control  flow
// // 
// fn auth(password:&str)->bool{
//     if password == "password" {
//         true
//     } else {
//         false
//     }
// }

// fn auth2(password:&str)->&str{
//     if password == "password" {
//         return "varified";
//     } else {
//         return "unvarified";
//     }}

    
// fn  mut_iter(){
//     let mut arr1 = [1, 2, 3, 4, 5];
//     for i in &mut arr1 {
//         *i += 1;
//     }
//     println!("{:?}", arr1);
// }
// fn tup(){
//     let tup = (1, 2, 3);
//     for i in &tup {
//         println!("Value: {}", i);
//     }
//     let mut tup1 = (1, 2, 3);
//     for i in &mut tup1 {
//         *i += 1;
//     }
//     println!("{:?}", tup1);
// }
// fn mutTup(){
//     let mut tup = (1, 2, 3);
//     for i in &mut tup {
//         *i += 1;
//     }
//     println!("{:?}", tup);
// }
// //  code havcing siome isue due to  ownership  
// // lets read 

use std::f64::consts::PI;


fn main(){
    //  lets do  here ..
    //  Data types and type interface
    let  my_birth_year=2003;
    let  current_year:i32=2026;
    println!("i was born in {},i am {} year old! ",my_birth_year,current_year-my_birth_year);
    let temp_in_celcius:f64=34.90;
    let ferenhite=temp_in_celcius*9.0/5.0+32.0;
    println!("temp  in celiuc is:{},convderted in ferenhite{}",temp_in_celcius,ferenhite);
    //
     let num1=34;
     let  num2=56;
     println!("sum is {}, diif is:{},multi isd:{},div is {},",num1+num2,num1-num2,num1*num2,num1/num2);
     //
     let radius:f64=34.23;
     let area:f64=PI*radius*radius;
     println!("my circle radius is: {}cm , and my  area of circle is: {} cm ",radius,area);
    let total_seconds: i32 = 3665;
let hours = total_seconds / 3600;
let minutes = (total_seconds % 3600) / 60;
let seconds = total_seconds % 60;

println!("{} seconds = {}h {}m {}s", total_seconds, hours, minutes, seconds);
}