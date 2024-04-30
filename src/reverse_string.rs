use std::io;

fn read_string() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");

    input.trim().to_string()
}
fn main() {
    // Take input from the user
    println!("Type here the phrase you would like to reverse");

    let input = read_string();

    // Print the string
    println!(
        "\nThe reversed string you have just written is:\n{}",
        input.chars().rev().collect::<String>(),
    );
}
