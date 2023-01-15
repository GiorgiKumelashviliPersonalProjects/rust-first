#![allow(unused_assignments)]

use rand::Rng;
use std::cmp::Ordering;



fn print_lines(param: Option<&str>) {
    println!("{}", "=".repeat(40) + " " + param.unwrap_or(""));
}

fn loger() {
    print_lines(None);
}
