#![allow(unused_assignments)]
#![allow(dead_code)]

mod another;
mod file_read;
mod test;

use crate::another::print_hello;
use file_read::read_write_file;

pub fn print_lines(param: Option<&str>) {
    println!("{}", "=".repeat(40) + " " + param.unwrap_or(""));
}

fn main() {
    print_hello();
    let basic_closure = |age: i32| {
        return age > 18;
    };

    print_lines(None);
    test::some_test_module::another_one::say_hello();
    read_write_file();

    let arr = [1, 2, 3, 4];

    for val in arr.iter() {
        println!("{}", val);
    }
}
