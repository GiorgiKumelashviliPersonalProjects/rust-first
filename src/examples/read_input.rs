#![allow(non_snake_case)]
#![allow(unused_assignments)]

// use rand::Rng;
// use std::io;

use std::io::{self, Write};

fn read_input() {
    let mut input: String = String::new();
    print!("Enter input for string: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    println!("{}", input);
}

fn main() {}
