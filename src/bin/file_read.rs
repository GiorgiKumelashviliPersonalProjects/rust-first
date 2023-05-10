use std::fs::File;
use std::io::{BufRead, BufReader, Write};

pub fn read_write_file() {
    let path = "file.txt";
    let output = File::create(path);

    let mut output = match output {
        Ok(file) => file,
        Err(err) => panic!("error creating file {}, err {:?}", path, err),
    };

    write!(output, "Some random text").expect("Failed to write file");

    // read
    let file = File::open(path).unwrap();
    let bufered = BufReader::new(file);

    for line in bufered.lines() {
        print!("Line -> {}", line.unwrap());
    }
}

fn main() {
    read_write_file();
}
