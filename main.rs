use std::io;
use rand::{thread_rng, Rng};

fn main(){
    println!("Hello World!");
    println!("Please input your guess!");

    let secret_num = thread_rng().gen_range(1..=100);
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed : {guess}");
    println!("secret Number : {secret_num}");
}