use std::{
    env,
    fs::{self, File, OpenOptions},
    io::{self, BufRead, BufReader, Write},
};

fn main() -> io::Result<()> {
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

fn read_last_from_file() -> io::Result<String> {
    let file = File::open("times.txt")?;
    let lines = BufReader::new(file).lines();
    let last_el = lines.last().unwrap();

    last_el
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
