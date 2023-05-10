#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(unused_imports)]
// #![allow(rustdoc::all)]

use std::fmt::{Debug, Display};

pub fn print_lines(param: Option<&str>) {
    println!("{}", "=".repeat(40) + " " + param.unwrap_or(""));
}

struct Anything<T> {
    x: T,
    y: T,
}

trait Combine {
    fn combine(&self) -> i32;
}
trait Combine2 {
    fn combine2(&self, x: i32, y: i32) -> i32 {
        return x + y;
    }

    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

impl Anything<i32> {
    fn compare(&self) -> i32 {
        return if self.x > self.y { self.x } else { self.y };
    }
}

impl Combine for Anything<i32> {
    fn combine(&self) -> i32 {
        return self.x + self.y;
    }
}

impl Combine2 for Anything<i32> {}

fn stuff(param: impl Combine2) {
    println!("{}", param.combine2(1, 2));
}

fn same_stuff<T: Combine2>(param: T) {
    println!("{}", param.combine2(1, 2));
}
fn same_stuff_2<T: Combine2 + Combine>(param: T) {
    println!("{}", param.combine2(1, 2));
}
fn same_stuff_3(param: &(impl Combine + Combine2)) {
    println!("{}", param.summarize());
}
fn same_stuff_4<T, U>(x: &T, y: &U) -> ()
where
    T: Combine,
    U: Combine2,
{
    println!("{}, {}", x.combine(), y.summarize());
}

fn print_any<T: Display + Debug>(arg: T) {
    println!("{:?}", arg);
}

fn main() {
    print_lines(None);

    let num_anything = Anything { x: 123, y: 321 };
    let compare = num_anything.compare();

    print_any(compare);
    print_any(num_anything.combine());
    print_any(num_anything.summarize());

    println!("hello");
    println!("hello second");

    // let string_anything = Anything {
    //     x: String::from("x"),
    //     y: String::from("y"),
    // };
    // string_anything.compare(); // panic
    // string_anything.combine(); // panic
}
