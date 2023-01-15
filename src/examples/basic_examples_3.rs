#![allow(unused_assignments)]
#![allow(dead_code)]

use std::{collections::HashMap, ops::Add};

fn print_lines(param: Option<&str>) {
    println!("{}", "=".repeat(40) + " " + param.unwrap_or(""));
}

fn get_sum_generic<T: Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}

#[derive(Debug)]
enum Days {
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
    Sun,
}

impl Days {
    fn is_week_day(&self) -> bool {
        return match self {
            Days::Sat | Self::Sun => true,
            _ => false,
        };
    }
}

fn main() {
    print_lines(Some("Enums"));
    let today: Days = Days::Fri;

    println!("is today weekday ? {:?}", today.is_week_day());
    println!(
        "today is {:?} and where is today weekday ? {:?}",
        today,
        today.is_week_day()
    );

    print_lines(Some("Generics"));

    println!("{:?}", get_sum_generic(1, 2));
    println!("{:?}", get_sum_generic(1.2, 2.5));

    print_lines(Some("Hash maps"));

    let mut heroes_hashmap = HashMap::new();

    heroes_hashmap.insert("superman", "strong");
    heroes_hashmap.insert("batman", "smart");
    println!("{:?}\n", heroes_hashmap);

    for (k, v) in heroes_hashmap.iter() {
        println!("{} = {}", k, v);
    }

    print_lines(Some("Structs and Traits"));

    #[derive(Debug)]
    struct Rectangle<H, W> {
        height: H,
        width: W,
    }

    let mut rec = Rectangle {
        height: 12,
        width: 34,
    };

    rec.height = 22;
    println!("{:?}", rec);

    trait Shape {
        fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32;
    }

    impl Shape for Rectangle<f32, f32> {
        fn new(height: f32, width: f32) -> Rectangle<f32, f32> {
            return Rectangle { height, width };
        }

        fn area(&self) -> f32 {
            return self.height * self.width;
        }
    }

    let rec2: Rectangle<f32, f32> = Shape::new(12.0, 14.0);
    rec2.area();
}
