use std::io;

fn multiply(a: i32, b: i32) -> i32{
    a * b
}

fn main() {
    let mut input = String::new();
    println!("Enter first number:");
    io::stdin().read_line(&mut input).expect("Read failed");
    let a: i32 = input.trim().parse().expect("Enter a number");
  
    input.clear();
    
    println!("Enter second number:");
    io::stdin().read_line(&mut input).expect("Read failed");
    let b: i32 = input.trim().parse().expect("Enter b number");
 

    let result = multiply(a, b);

    println!("Result = {}",result);
}
