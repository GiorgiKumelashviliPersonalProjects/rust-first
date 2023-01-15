pub mod some_test_module {
    fn wassup_something() {
        println!("hello");
    }

    pub mod another_one {
        pub fn say_hello() {
            super::wassup_something();
        }
    }
}

pub mod something {
    pub mod inside1 {
        use crate::print_lines;

        pub fn print_hello() {
            print_lines(Some("Hello from inside 1"));
        }
    }
    pub mod inside2 {
        use crate::print_lines;

        pub fn print_hello() {
            print_lines(Some("Hello from inside 2"));
        }
    }
}
