mod time_error;

use crate::time_error::TimerError;

use std::env;
use std::fs;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::path::Path;

use time::format_description::BorrowedFormatItem;
use time::{macros::format_description, OffsetDateTime};

static FORMAT: &[BorrowedFormatItem<'_>] = format_description!(
    "[year]-[month]-[day] [hour]:[minute]:[second] \
    [offset_hour sign:mandatory]:[offset_minute]:[offset_second]"
);

#[allow(dead_code)]
static TIMES_DIR: &str = "my-timer";
static TIMES_FILE: &str = "times.txt";

// TODO:
// 1. add cli crate
// 2. get dates
fn main() -> Result<(), TimerError> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err(TimerError::WrongNumberOfArguments);
    }

    #[cfg(all(not(debug_assertions), target_os = "linux"))]
    let dir_path = env::var("HOME")? + "/.config/" + TIMES_DIR;

    #[cfg(not(debug_assertions))]
    if !Path::new(dir_path.as_str()).exists() {
        fs::create_dir(&dir_path)?;
    }

    #[cfg(not(debug_assertions))]
    let path = dir_path + "/" + TIMES_FILE;
    #[cfg(debug_assertions)]
    let path = String::from(TIMES_FILE);

    let file = if !Path::new(path.as_str()).exists() {
        fs::File::create_new(path)? // put this as global
    } else {
        fs::File::options()
            .write(true)
            .read(true)
            .append(true)
            .open(path)?
    };

    if &args[1] == "r" {
        // TODO:
        // 1. all other commands should return NoLastRecord
        add_new_time(file)?;
    } else if &args[1] == "l" {
        println!("{}", read_last_from_file(file)?);
    } else if &args[1] == "w" {
        println!("{}", last_time(file)?);
    } else if &args[1] == "a" {
        println!("{}", read_all_from_file(file)?);
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

fn add_new_time(file: fs::File) -> Result<(), TimerError> {
    let my_time = time::OffsetDateTime::now_utc().format(&FORMAT)?;
    println!("{}", my_time);
    write_to_file(file, my_time)?;

    Ok(())
}

fn read_last_from_file(file: fs::File) -> Result<String, TimerError> {
    let lines = BufReader::new(file).lines();
    if let Some(last_el) = lines.last() {
        Ok(last_el?)
    } else {
        Err(TimerError::NoLastRecord)
    }
}

fn read_all_from_file(mut file: fs::File) -> io::Result<String> {
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    buf.pop();

    Ok(buf)
}

fn write_to_file(mut file: fs::File, value: String) -> io::Result<()> {
    file.write((value + "\n").as_bytes())?;

    Ok(())
}

fn last_time(file: fs::File) -> Result<String, TimerError> {
    let last_time = read_last_from_file(file)?;
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
