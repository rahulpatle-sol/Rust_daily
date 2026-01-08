fn  main(){
    // ownership 3 golden rules
    // 1. Each value in Rust has a variable that’s called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.
    // 
    // 
    // 
    // 
    let x = 5;
    // x is an integer value with the value of 5
    // x  is the owners 
    // x is the owner of the value 5
    println!("The value of x is: {}", x);
    
    let y = x;
    // y is an integer value with the value of 5
    // y is the owner of the value 5
    println!("The value of y is: {}", y);
    
    // x is no longer the owner of the value 5
    // y is the owner of the value 5
    println!("The value of x is: {}", x);


}