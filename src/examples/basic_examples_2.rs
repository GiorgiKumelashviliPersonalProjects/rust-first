#![allow(unused_assignments)]

use std::cmp::Ordering;

use rand::Rng;

fn print_lines(param: Option<&str>) {
    println!("{}", "=".repeat(40) + " " + param.unwrap_or(""));
}

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn loger() {
    print_lines(None);

    let random_int = rand::thread_rng().gen_range(1..1001);
    let age = 34;

    let can_vote = if age >= 18 { true } else { false };

    if random_int < 500 && random_int > 100 {
        println!("Between 100 and 500")
    }

    if (random_int < 500) && (random_int > 100) {
        println!("Between 100 and 500")
    }

    println!("random_int {}", random_int);
    println!("can_vote {}", can_vote);
    println!("{:?}", 1..101);

    match age.cmp(&18) {
        Ordering::Equal => println!("Can vote at 18"),
        Ordering::Greater => println!("Definetly can vote"),
        Ordering::Less => println!("Can not vote"),
    }

    let arr = [15, 20, 35, 40, 55];
    let mut loop_index = 0;

    print_lines(None);

    println!("last {}", arr.last().unwrap());

    loop {
        let item = arr[loop_index];
        println!("number {}", item);

        if item == 55 {
            break;
        };

        if item % 2 == 0 {
            loop_index += 1;
            continue;
        }

        println!("odd number {}", item);
        loop_index += 1;
    }

    print_lines(None);

    let my_tuple: (u8, String, f64) = (47, "something".to_string(), 50_000_000.0);
    let (x, ..) = my_tuple;

    println!("tuple {:?}", my_tuple);
    println!("first element {:?}", my_tuple.0);
    println!("first element {:?}", x);

    print_lines(None);

    let mut modifiable_string = String::new();
    modifiable_string.push('A');
    modifiable_string.push_str(" modifiable_string");
    modifiable_string = modifiable_string.replace("_", " ");

    println!("{}", modifiable_string);
    println!("{:?}", modifiable_string);
    println!();

    for word in modifiable_string.split_whitespace() {
        println!("{}", word);
    }

    print_lines(None);

    let mut st3 = String::from("a c d d d g b h 2");
    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();
    v1.dedup();

    let byte_arr = st3.as_bytes();
    let st4 = &st3[0..3];

    println!("{:?}", byte_arr);
    println!("{:?}", st4);

    st3.clear();
    println!("{:?}", st3);

    print_lines(Some("Casting"));

    let u1: u8 = 12;
    let u2: u8 = 22;

    let final_u: u32 = (u1 as u32) + (u2 as u32);
    println!("{:?}", final_u);

    print_lines(Some("Enums"));
}
