
//  to  taking input from the user and displaying game state.
use std::io;

use rand::Rng;

fn main() {
    println!("+++++++++++ welcome to the Guessing Game +++++++++++
    ==========================================");

    println!("Please enter your num : ");

let mut input=String::new();
io::stdin().read_line(&mut input).expect("failed to  red line");
println!("you entered : {}",input);


///  gnerate a random number between 1 to 100
/// 
let secret_num=rand::thread_rng().gen_range(1..=100);
println!("the secret number is:{}",secret_num);

}


