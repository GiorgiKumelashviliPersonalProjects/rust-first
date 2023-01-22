#![allow(non_snake_case)]
#![allow(unused_assignments)]

use rand::Rng;
use std::io::{self, Write};
use std::{cmp::Ordering, io::Write};

fn read_input() {
    let mut input: String = String::new();
    print!("Enter input for string: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    println!("{}", input);
}

fn guessing_game() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut try_count = 0;

    println!("{secret_number}");

    loop {
        print!("Please input your guess: ");

        let mut guess = String::new();

        std::io::stdout().flush().unwrap();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You guessed: {guess}, Try count: {try_count}");
                break;
            }
            Ordering::Less => println!("More"),
            Ordering::Greater => println!("Low"),
        }

        try_count += 1;
    }
}
