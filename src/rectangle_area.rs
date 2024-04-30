use std::io;

fn int_input() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
        .trim()
        .parse::<i32>()
        .expect("Please enter a valid integer!")
}
fn main() {
    println!("Please input the width of your rectangle");
    let width = int_input();

    println!("Please input the height of your rectangle");
    let height = int_input();

    let rectangle_area = width * height;

    println!("The rectangle is of area: {rectangle_area}");
}
