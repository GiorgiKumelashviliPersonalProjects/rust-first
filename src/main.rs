#![allow(unused_assignments)]
#![allow(dead_code)]

mod another;
mod file_read;
mod test;

use rand::Rng;
use std::{cmp::Ordering, io::Write};

pub fn print_lines(param: Option<&str>) {
    println!("{}", "=".repeat(40) + " " + param.unwrap_or(""));
}

fn get_num() -> i32 {
    return 5;
}

fn main() {
    print_lines(None);
    // let x = 5;
    // let y = 10;

    // println!("x = {x} and y + 2 = {}", y + 2);
    // println!("{:?}", get_num());

    // print_hello();

    // let _basic_closure = |age: i32| {
    //     return age > 18;
    // };

    // print_lines(None);
    // test::some_test_module::another_one::say_hello();
    // read_write_file();

    // let arr = [1, 2, 3, 4];

    // for val in arr.iter() {
    //     println!("{}", val);
    // }
}
