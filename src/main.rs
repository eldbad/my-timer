use std::{
    env, error, fmt,
    fs::{self, File, OpenOptions},
    io::{self, BufRead, BufReader, Write},
};

// why pub tho?
#[derive(Debug)]
pub struct NoLastRecordError;

impl fmt::Display for NoLastRecordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "no last record found in file")
    }
}

impl error::Error for NoLastRecordError {}

#[derive(Debug)]
pub enum OopsError {
    NoLastRecordError,
    IoError(io::Error),
}

impl From<io::Error> for OopsError {
    fn from(error: io::Error) -> Self {
        OopsError::IoError(error)
    }
}

fn main() -> Result<(), OopsError> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("oops");
    }
    if &args[1] == "r" {
        let my_time = time::OffsetDateTime::now_utc().to_string();
        println!("{}", my_time);
        write_to_file(my_time)?;
    } else if &args[1] == "l" {
        println!("{}", read_last_from_file()?);
    } else if &args[1] == "a" {
        println!("{}", read_all_from_file()?);
    } else {
        panic!("wrong arguments");
    }

    Ok(())
}

fn read_last_from_file() -> Result<String, OopsError> {
    let file = File::open("times.txt")?;
    let lines = BufReader::new(file).lines();
    if let Some(last_el) = lines.last() {
        Ok(last_el?)
    } else {
        Err(OopsError::NoLastRecordError)
    }
}

fn read_all_from_file() -> io::Result<String> {
    fs::read_to_string("times.txt")
}

fn write_to_file(date: String) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("times.txt")?;
    file.write((date + "\n").as_bytes())?;

    Ok(())
}
