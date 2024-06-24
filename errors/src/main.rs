use std::fs;

#[derive(Debug)]
enum ReadNumberError {
    Io(std::io::Error),
    ParseInt(std::num::ParseIntError),
}

impl std::error::Error for ReadNumberError {}

impl std::fmt::Display for ReadNumberError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ReadNumberError::Io(err) => write!(f, "IO error: {}", err),
            ReadNumberError::ParseInt(err) => write!(f, "ParseInt error: {}", err),
        }
    }
}

impl From<std::io::Error> for ReadNumberError {
    fn from(err: std::io::Error) -> Self {
        ReadNumberError::Io(err)
    }
}

impl From<std::num::ParseIntError> for ReadNumberError {
    fn from(err: std::num::ParseIntError) -> Self {
        ReadNumberError::ParseInt(err)
    }
}

fn read_number_from_file(path: &str) -> Result<i32, ReadNumberError> {
    let contents = fs::read_to_string(path)?;
    let number: i32 = contents.trim().parse()?;
    Ok(number)
}

fn main() {
    match read_number_from_file("./errors/number.txt") {
        Ok(num) => println!("Number: {}", num),
        Err(ReadNumberError::Io(err)) => eprintln!("IO error: {}", err),
        Err(ReadNumberError::ParseInt(err)) => eprintln!("ParseInt error: {}", err),
    }
}
