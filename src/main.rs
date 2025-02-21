use std::{
    env,
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Read, Write},
};

fn main() {
    // Commands
    // 1. Parse and print last time. If not found - print "not found"
    // 2. Restart time
    // 3. View all times
    // TODO: change unwrap to something more pleasing
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("oops");
    }
    if &args[1] == "r" {
        let my_time = time::OffsetDateTime::now_utc().to_string();
        println!("{}", my_time);
        write_to_file(my_time);
    } else if &args[1] == "l" {
        println!("{}", read_last_from_file());
    } else if &args[1] == "a" {
        println!("{}", read_all_from_file());
    } else {
        panic!("wrong arguments");
    }
}

fn read_last_from_file() -> String {
    let file = File::open("times.txt").unwrap();
    let lines = BufReader::new(file).lines();
    let last_el = lines.last().unwrap().unwrap();

    return last_el;
}

fn read_all_from_file() -> String {
    fs::read_to_string("times.txt").unwrap()
}

fn write_to_file(date: String) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("times.txt")
        .unwrap();
    file.write((date + "\n").as_bytes()).unwrap();
}
