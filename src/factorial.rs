use std::io;

fn int_input() -> i32 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let num: i32 = input
        .trim()
        .parse::<i32>()
        .expect("Please enter a valid integer!");
    num.abs() // Makes sure the number is always positive
}

fn factorial(n: i32) -> i32 {
    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }
    result
}

fn main() {
    // Get an input number from the user
    println!("Please enter the number you want to know the factorial of!");
    let number = int_input();

    // Calculate the factorial of the number
    let result = factorial(number);

    // Print the factorial of the number
    println!("The factorial of {} is {}", number, result);
}
