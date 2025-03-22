mod time_error;

use crate::time_error::TimerError;

use std::{
    env,
    fs::{self, File, OpenOptions},
    io::{self, BufRead, BufReader, Write},
};

use time::format_description::BorrowedFormatItem;
use time::{macros::format_description, OffsetDateTime};

// Can we omit subseconds?
static FORMAT: &[BorrowedFormatItem<'_>] = format_description!(
    "[year]-[month]-[day] [hour]:[minute]:[second] \
    [offset_hour sign:mandatory]:[offset_minute]:[offset_second]"
);

fn main() -> Result<(), TimerError> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(TimerError::WrongNumberOfArguments);
    }

    if &args[1] == "r" {
        add_new_time()?;
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

fn add_new_time() -> Result<(), TimerError> {
    let my_time = time::OffsetDateTime::now_utc().format(&FORMAT)?;
    println!("{}", my_time);
    write_to_file(my_time)?;

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
    // TODO: fix incorrect hour description
    // Because [hour] doesn't recognize one
    // literal, only two
    let new_last_time = OffsetDateTime::parse(last_time.as_str(), &FORMAT).unwrap(); // DELETE UNWRAP
    let result = OffsetDateTime::now_utc() - new_last_time;

    let mut string_result = result.to_string();
    let seconds_position = string_result.chars().position(|c| c == 's').unwrap(); // DELETE ANOTHER UNWRAP
    string_result.truncate(seconds_position + 1);

    Ok(string_result)
}
}
