use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let f = File::open("stairway.md").unwrap();
    let mut lines = BufReader::new(f).lines();

    loop {
        if let Some(line) = lines.next() {
            let line = line.unwrap();
            println!("{} ({} bytes long)", line, line.len());
        } else {
            break;
        }
    }
}
