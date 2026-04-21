use std::io;

fn main() {
    let mut num = String::new();

    println!("Please enter the numbers:");

    io::stdin()
        .read_line(&mut num)
        .expect("Data reading failed");
    
    let number: i32 = num
        .trim()
        .parse()
        .expect("Please enter is as number");

    println!("The received value is {}", number);

    let result = if number % 2 == 0 {
        "even"
    } else {
       "Odd"
    };
    println!("{}",result)
}
