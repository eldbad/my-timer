mod time_error;

use crate::time_error::TimerError;

use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

use time::format_description::BorrowedFormatItem;
use time::{macros::format_description, OffsetDateTime};

static FORMAT: &[BorrowedFormatItem<'_>] = format_description!(
    "[year]-[month]-[day] [hour]:[minute]:[second] \
    [offset_hour sign:mandatory]:[offset_minute]:[offset_second]"
);

// TODO:
// 1. add cli crate
// 2. create times.txt if not found
// 3. get dates
// 4. times.txt to $HOME/.config/my-timer/
fn main() -> Result<(), TimerError> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(TimerError::WrongNumberOfArguments);
    }

    if &args[1] == "r" {
        // TODO:
        // 1. all other commands should return NoLastRecord
        // 2. should create a directory my-timer and file times.txt corresponding to operating system
        add_new_time()?;
    } else if &args[1] == "l" {
        println!("{}", read_last_from_file()?);
    } else if &args[1] == "w" {
        println!("{}", last_time()?);
    } else if &args[1] == "a" {
        println!("{}", read_all_from_file()?);
    } else if &args[1] == "h" {
        println!("my-timer <command>");
        println!("r - restart timer (add a new time)");
        println!("l - show last time");
        println!("w - show time passed from last time");
        println!("a - show all times");
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
        Err(TimerError::NoLastRecord)
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
    let new_last_time = OffsetDateTime::parse(last_time.as_str(), &FORMAT)?;
    let result = OffsetDateTime::now_utc() - new_last_time;

    let mut string_result = result.to_string();
    let seconds_position = string_result
        .chars()
        .position(|c| c == 's')
        .ok_or(TimerError::ParseDuration)?;
    string_result.truncate(seconds_position + 1);

    Ok(string_result)
}
}
