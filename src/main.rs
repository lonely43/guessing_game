use std::io;
use std::cmp::Ordering;
use rand::random_range;

fn main() {
    println!("Guessing Game!");

    let secret_number: u32 = random_range(1..=100);

    loop {
        println!("Please input the number"); // comment :3
        
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small guess"),
            Ordering::Greater => println!("Too big guess"),
            Ordering::Equal => {
                println!("You are winner!");
                break;
            },
        }
    }
    
    println!("Secret number is {}", secret_number)
}