use std::io;

fn read_input() -> (char, usize) {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim().to_uppercase();

        if input.len() == 2 {
            let (letter, number) = input.split_at(1);
            if let Ok(number) = number.parse::<usize>() {
                if ["A", "B", "C"].contains(&letter) && number >= 1 && number <= 3 {
                    return (letter.chars().next().unwrap(), number);
                }
            }
        }
        println!(
            "Invalid input. Please enter a letter (A, B, or C) followed by a number (1, 2, or 3)."
        );
    }
}
fn read_char() -> char {
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read character.");

        let input = input.trim().to_uppercase();

        match input.chars().next() {
            Some('Y') | Some('N') => return input.chars().next().unwrap(),
            _ => println!("Invalid input. Please enter 'Y' or 'N'."),
        }
    }
}
fn main() {
    let mut play_again = true;

    while play_again {
        let mut game_board = vec![vec![' '; 3]; 3];
        let mut current_player = 'X';

        loop {
            println!(
                "Player {}, input a letter and number coordinate.",
                if current_player == 'X' { 1 } else { 2 }
            );
            let (letter, number) = read_input();
            let row = (letter as u8 - 'A' as u8) as usize;
            let col = number - 1;

            if game_board[row][col] == ' ' {
                game_board[row][col] = current_player;
            } else {
                println!("This spot is already taken. Please choose another one.");
                continue;
            }

            // Print the game board
            println!("   1 2 3");
            for (i, row) in game_board.iter().enumerate() {
                print!("{}  ", (65 + i as u8) as char);
                for cell in row {
                    print!("{} ", cell);
                }
                println!();
            }

            // Check for a win condition
            if has_won(&game_board, current_player) {
                println!(
                    "Player {} has won!",
                    if current_player == 'X' { 1 } else { 2 }
                );
                break;
            }

            // Switch players
            current_player = if current_player == 'X' { 'O' } else { 'X' };
        }

        println!("Do you want to play again? (Y/N)");
        let answer = read_char();
        if answer == 'N' {
            play_again = false;
        }
    }
}

fn has_won(board: &Vec<Vec<char>>, player: char) -> bool {
    // Check rows, columns and diagonals for a win condition
    for i in 0..3 {
        if (board[i][0] == player && board[i][1] == player && board[i][2] == player)
            || (board[0][i] == player && board[1][i] == player && board[2][i] == player)
        {
            return true;
        }
    }
    if (board[0][0] == player && board[1][1] == player && board[2][2] == player)
        || (board[0][2] == player && board[1][1] == player && board[2][0] == player)
    {
        return true;
    }
    false
}
