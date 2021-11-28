use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::stdin;
use std::str::FromStr;
use clap::{App, load_yaml};

pub fn main() {
    let (n, path) = parse_args();

    match more(n, path) {
        Ok(_) => println!("done"),
        Err(error) => println!("error {}", error),
    }
}

fn parse_args() -> (i32, String) {
    let yaml = load_yaml!("more.yaml");
    let matches = App::from(yaml).get_matches();
    
    let mut n = 5;
    if let Some(i) = matches.value_of("lines") {
        n = FromStr::from_str(i).unwrap();
    }
    let mut path = String::from("input.txt");
    if let Some(f) = matches.value_of("file") {
        path = String::from(f);
    }

    (n, path)
}

fn more(n: i32, file_path: String) -> std::io::Result<()> {
    let file = File::open(file_path).expect("File not found");

    let mut reader = BufReader::new(file);
    let mut len: usize = 1;
    while len != 0 {
        read_n_lines(n, &mut len, &mut reader)?;

        wait_for_enter()?;
    }

    Ok(())
}

fn read_n_lines(n: i32, len: &mut usize, reader: &mut BufReader<File>) -> std::io::Result<()> {
    let mut counter = 0;
    while *len != 0 && counter < n {
        let mut line_buf = vec![];
        *len = reader.read_until(b'\n', &mut line_buf)?;
        print!("{}", String::from_utf8(line_buf).unwrap());
        // let mut line_buf = String::new();
        // *len = reader.read_line(&mut line_buf)?;
        // print!("{}", line_buf);
        counter += 1;
    }

    Ok(())
}

fn wait_for_enter() -> std::io::Result<()> {
    stdin().read_line(&mut String::new())?;

    Ok(())
}
