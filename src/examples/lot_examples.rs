#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(unused_imports)]
// #![allow(rustdoc::all)]

// mod another;
// mod file_read;
// mod test;

// use examples::basic_examples_2::clear_screen;
use rand::Rng;
use std::{
    cmp::{max, Ordering},
    collections::HashMap,
    env,
    fs::File,
    io::{self, ErrorKind, Read, Write},
    ops::Add,
};
// mod examples;

// use crate::{
//     editor::{
//         tab::LapceEditorTab,
//         view::{editor_tab_child_widget, LapceEditorView},
//     },
//     terminal::LapceTerminalView,
// };

// use crate::examples::basic_examples_2::clear_screen;

pub fn print_lines(param: Option<&str>) {
    println!("{}", "=".repeat(40) + " " + param.unwrap_or(""));
}

fn get_num() -> i32 {
    return 5;
}

fn print_num(num: &mut i32) {
    *num += 1;

    println!("{:?}", num);
}

fn print_string(str: &mut String) {
    str.push_str("modified");

    println!("{:?}", str);
}

#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}

#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        return self.width * self.height;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }
}

impl Person {
    fn log(&self) {
        println!("name is {}, age is {}", self.name, self.age);
    }
}

#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

#[derive(strum_macros::Display)]
enum MyEnum {
    A,
    B,
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("helxlo.txt")?.read_to_string(&mut username)?;

    return Ok(username);
}

fn main() {
    // load all env in .env file
    dotenv::dotenv().ok();

    print_lines(None);
    println!("{:?}", read_username_from_file());

    // let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error);
    //         }
    //     },
    // };

    // print_lines(None);
    // println!("{:?}", greeting_file);
    // println!("{}", env::var("SOMETHING").unwrap());
    print_lines(None);
    // for (x, y) in env::vars_os() {
    //     println!("{}, {}", x.to_str().unwrap(), y.to_str().unwrap());
    // }
    // print_lines(None);

    let mut hash_map: HashMap<String, i32> = HashMap::new();

    hash_map.insert(String::from("key 1"), 32);
    hash_map.insert(String::from("key 2"), 323);

    println!("{:?}", hash_map);
    println!("{:?}", hash_map.get(&String::from("key 1")).unwrap());

    for (k, v) in &hash_map {
        println!("{}, {}", k, v);
    }

    // hash_map.entry(String::from("key 1")).or_insert(12);
    hash_map.entry(String::from("key 4")).or_insert(12);
    println!("{:?}", hash_map);
    panic!("hello");
    // println!("{}", field_name);s

    // let v1: Vec<i32> = Vec::new();
    // let mut v = vec![1, 2, 3];

    // v.push(23);

    // let third: &i32 = &v[2];
    // println!("The third element is {third}");

    // let third: Option<&i32> = v.get(2);
    // match third {
    //     Some(third) => println!("The third element is {third}"),
    //     None => println!("There is no third element."),
    // }

    // println!("{:?}", v);

    // print_lines(None);

    // let mut xxx = vec![1, 2, 3, 4, 5];
    // let first = xxx[0];
    // // let first = &xxx[0]; // panics
    // xxx.push(6);

    // println!("The first element is: {first}");

    // #[derive(Debug)]
    // enum SpreadsheetCell {
    //     Int(i32),
    //     Float(f64),
    //     Text(String),
    // }

    // let row = vec![
    //     SpreadsheetCell::Int(3),
    //     SpreadsheetCell::Text(String::from("blue")),
    //     SpreadsheetCell::Float(10.12),
    // ];

    // println!("{:?}", row);

    // let s1 = String::from("hello");
    // let len = calculate_length(&s1);
    // println!("The length of '{}' is {}.", s1, len);

    // let black = Color(0, 0, 0);
    // let origin = Point(0, 0, 0);

    // let mut person = Person {
    //     age: 12,
    //     name: String::from("asdas"),
    // };

    // let another_person = Person { age: 23, ..person };

    // person.age = 90;

    // another_person.log();
    // // println!("{:#?}", another_person);
    // println!("{:#?}", origin);
    // println!("{:#?}", black);
    // dbg!("{:#?}", black);

    // print_lines(None);

    // let rect1 = Rectangle {
    //     width: 30,
    //     height: 50,
    // };
    // let rect2 = Rectangle {
    //     width: 10,
    //     height: 40,
    // };
    // let rect3 = Rectangle {
    //     width: 60,
    //     height: 45,
    // };

    // println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    // println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    // // println!("{:?}", another_person);

    // #[derive(Debug)]
    // enum IpAddrKind {
    //     V4,
    //     V6,
    // }

    // #[derive(Debug)]
    // enum IpAddrKindSecond {
    //     V4(String),
    //     V6,
    // }

    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // print_lines(None);
    // println!("{:#?}", four);
    // println!("{:?}", four);
    // println!("{:?}", MyEnum::A.to_string());

    // print_type_of(&four);

    // let num = &mut 123;
    // print_num(num);
    // println!("{:?}", num);

    // let mut str = String::from("hello ");
    // print_string(&mut str);
    // println!("{:?}", str);

    // let len = first_word(&str);

    // println!("{len}");

    // #[derive(Debug)]
    // enum IpAddr {
    //     V4(String),
    //     V6(String),
    // }

    // let home = IpAddr::V4(String::from("127.0.0.1"));

    // print_lines(None);
    // println!("{:?}", home);

    // let config_max = Some(300);
    // // let config_max = Some(300);
    // // if let max = config_max.unwrap() {
    // //     println!("The maximum is configured to be {}", max)
    // // }

    // clear_screen();
}

// fn calculate_length(s: &String) -> usize {
//     return s.len();
// }

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    return s.len();
}
