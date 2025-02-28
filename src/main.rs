use std::{
    env, fmt,
    fs::{self, File, OpenOptions},
    io::{self, BufRead, BufReader, Write},
};

use time::{macros::format_description, OffsetDateTime};

#[derive(Debug)]
pub enum TimerError {
    NoLastRecordError,
    WrongNumberOfArguments,
    WrongArgument,
    IoError(io::Error),
}

impl fmt::Display for TimerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimerError::NoLastRecordError => write!(f, "no last record found in file"),
            TimerError::IoError(err) => write!(f, "{}", err),
            TimerError::WrongNumberOfArguments => write!(f, "wrong number of arguments given"),
            TimerError::WrongArgument => write!(f, "wrong argument"),
        }
    }
}

impl From<io::Error> for TimerError {
    fn from(error: io::Error) -> Self {
        TimerError::IoError(error)
    }
}

fn main() -> Result<(), TimerError> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(TimerError::WrongNumberOfArguments);
    }

    // change to match
    // and remove Ok(())
    if &args[1] == "r" {
        let my_time = time::OffsetDateTime::now_utc().to_string();
        println!("{}", my_time);
        write_to_file(my_time)?;
    } else if &args[1] == "l" {
        println!("{}", read_last_from_file()?);
    } else if &args[1] == "w" {
        println!("{}", last_time()?);
    } else if &args[1] == "a" {
        println!("{}", read_all_from_file()?);
    } else {
        return Err(TimerError::WrongArgument);
    }

    Ok(())
}

fn read_last_from_file() -> Result<String, TimerError> {
    let file = File::open("times.txt")?;
    let lines = BufReader::new(file).lines();
    if let Some(last_el) = lines.last() {
        Ok(last_el?)
    } else {
        Err(TimerError::NoLastRecordError)
    }
}

fn read_all_from_file() -> io::Result<String> {
    fs::read_to_string("times.txt")
}

fn write_to_file(value: String) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("times.txt")?;
    file.write((value + "\n").as_bytes())?;

    Ok(())
}

fn last_time() -> Result<String, TimerError> {
    let last_time = read_last_from_file()?;
    // Can we omit subseconds?
    let format = format_description!(
        "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond] \
        [offset_hour sign:mandatory]:[offset_minute]:[offset_second]"
    );
    let new_last_time = OffsetDateTime::parse(last_time.as_str(), &format).unwrap(); // DELETE UNWRAP
    let result = OffsetDateTime::now_utc() - new_last_time;

    Ok(result.to_string())
}
