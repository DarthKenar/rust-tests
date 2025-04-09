use std::io; //std de standard library, el modulo IO de Input / Output

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You gu{}essed: {}", guess, guess);
}