
//  exercise 1
// tempratiure convertor

fn celcius_to_franhite(celcius:f64)->f64{
    celcius*9.0/5.0+32.0
}
fn farenhite_to_celcius(farenhite:f64)->f64{
    (farenhite-32.0)*5.0/9.0
}
//fizz buzz 
fn  fizz_buzz(num:u32){
for i in 1..num{
    if i%3==0 && i%5==0{
        println!("fizz")
    }
    else if i%3==0{
        println!("buzz")
    }
    else if i%5==0{
        println!("fizzbuzz")
    }
    else{
        println!("{}",i)
    }
}
}
//  task  4
// prime number checker 
fn is_prime(num:u32){
    
    if num<=1{
        println!("not prime");
    }
    else if num==2{
        println!("prime");
    }
    else{
        for i in 2..num{
            if num%i==0{
                println!("not prime");
                return;
            }
        }
        println!("prime");
    }
}
//  task 4 fibonaci  generator 
fn fibonaci(num:u32)->u32{
    if num==0{
        0
    }
    else if num==1{
        1
    }
    else{
        fibonaci(num-1)+fibonaci(num-2) 
    }
}
//  task  5
// some of disgits 
fn  sum_of_digits(num:u32)->u32{
    let mut sum=0;
    let mut n=num;
    while n>0{
        sum+=n%10;
        n/=10;
    }
    sum 
}
//  task 6 reverse  a number 
fn  reverse_number(num:u32)->u32{
    let mut reversed=0;
    let mut n=num;
    while n>0{
        reversed=reversed*10+n%10;
        n/=10;
    }
    reversed
}
//  task 7 
fn pattern(num:i32)->i32{
    let mut pattern=0;
    let mut n=num;
    while n>0{
    pattern=pattern*10+n%10;
    n/=10;
    
    }
    pattern
}
fn main() {
    println!("Hello, world!");
    println!(" the temp in farenhite{}",celcius_to_franhite(23.78));
    println!(" the temp in celcius{}",farenhite_to_celcius(74.8));
     println!(" the temp in celcius{:?}",fizz_buzz(75));
     is_prime(7);
     is_prime(10);
println!("prime number bro  {:?}",is_prime(76)); //  :?  for pritty  or multi line print
eprintln!("fib  of 10  {}",fibonaci(10));
eprintln!("sum of digits of 12345 {}",sum_of_digits(12345));
eprintln!("reverse of 12345 {}",reverse_number(12345));
eprintln!("reverse of 12345 {}",reverse_number(12345));
eprintln!("pattern of 12345 {}",pattern(12345));
eprintln!("pattern of 12345 {}",pattern(12345));



}       

