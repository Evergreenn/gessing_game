extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number !");

    let secret_member = rand::thread_rng().gen_range(1, 101);

    println!("Please input your guess.");

    println!("The secret number is {}", secret_member);

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
 
    println!("You guessed: {}", guess);
}
