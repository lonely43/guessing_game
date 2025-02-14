use std::io;

fn main() {
    println!("Guessing Game!");
    println!("Please, input the number"); // comment :3

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Opps... something went wrong");

    println!("You guessed: {guess}")
}