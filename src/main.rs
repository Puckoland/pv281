use std::collections::BTreeMap;
use std::io::stdin;
use std::collections::HashMap;
use std::process::exit;
use std::hash::{Hash, Hasher};

extern crate savefile;
use savefile::prelude::*;
#[macro_use]
extern crate savefile_derive;

#[derive(Savefile)]
#[derive(Debug)]
struct Link {
    number: usize,
    stations: Vec<String>,
}

impl PartialEq for Link {
    fn eq(&self, other: &Link) -> bool {
        self.number == other.number
    }
}
impl Hash for Link {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.number.hash(state);
    }
}

#[derive(Savefile)]
#[derive(Debug)]
struct Station {
    name: String,
    harmonogram: BTreeMap<String, usize>,
    to_main_station: bool,
}

#[derive(Savefile)]
struct Data {
    links: HashMap<usize, Link>,
    stations: HashMap<String, Station>,
}

fn save_data(data:&Data) {
    save_file("save.bin", 0, data).unwrap();
}

fn load_data() -> Data {
    let data = match load_file("save.bin", 0) {
        Ok(data) => data,
        _ => {
            println!("Could not find the savefile. Creating new data...");

            Data {
                links: HashMap::new(),
                stations: HashMap::new(),
            }
        }
    };

    data
}

fn delete_db(data: &mut Data) {
    *data = Data {
        links: HashMap::new(),
        stations: HashMap::new(),
    }
}

fn print_help() {
    println!("Usage:");
    println!("0 - Shutdown");
    println!("1 - Add link");
    println!("2 - Add station");
    println!("3 - Show link");
    println!("4 - Show station");
    println!("9 - Delete DB");
}

fn read_line() -> std::io::Result<String> {
    let mut input = String::new();
    stdin().read_line(&mut input)?;

    Ok(String::from(input.trim()))
}

fn add_link(links: &mut HashMap<usize, Link>) -> std::io::Result<()> {
    println!("Adding new link!");

    println!("Number of new link: ");
    let number: usize = read_line()?.parse().unwrap();

    println!("Stations in format ['' '' '' ...]: ");
    let stations: Vec<String> = read_line()?.split(' ')
        .map(|station| station.to_string()).collect();
    
    let link = Link {
        number: number,
        stations: stations,
    };
    links.insert(number, link);
    Ok(())
}

fn show_link(links: &mut HashMap<usize, Link>) -> std::io::Result<()> {
    println!("Links: {:?}", links);
    println!("Enter link number to show: ");
    let number: usize = read_line()?.parse().unwrap();
    match links.get(&number) {
        Some(link) => println!("Link {} stations: {:?}", number, (*link).stations),
        None => println!("There is no link with number '{}'!", number),
    }

    Ok(())
}

fn add_station(stations: &mut HashMap<String, Station>) -> std::io::Result<()> {
    println!("Adding new station!");

    println!("Name of the new station: ");
    let name: String = read_line()?;

    println!("Harmonogram in format [08:30 1,12:10 12,...]: ");
    let mut harmonogram: BTreeMap<String, usize> = BTreeMap::new();
    let entries: Vec<(String, usize)> = read_line()?.split(',')
        .map(|entry| {
            let a: Vec<String> = entry.trim().split(' ')
                .map(|s| String::from(s)).collect();
            (a[0].clone(), a[1].parse().unwrap())
        }).collect();
    for (key, value) in entries {
        harmonogram.insert(key, value);
    }

    println!("Is there a link to Main Station?");
    let to_main_station = read_line()?.eq("yes");

    let station = Station {
        name: name.clone(),
        harmonogram: harmonogram,
        to_main_station: to_main_station,
    };
    stations.insert(name, station);
    Ok(())
}

fn show_station(stations: &mut HashMap<String, Station>) -> std::io::Result<()> {
    println!("{:?}", stations);
    println!("Enter station name to show: ");
    let name = &read_line()?;
    match stations.get(name) {
        Some(station) => println!("{:?}", station),
        None => println!("There is no station with name '{}'!", name),
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut data = load_data();

    let mut input = String::new();
    while input != "0\n" {
        print_help();
        input = String::new();
        stdin().read_line(&mut input)?;

        println!("\n");
        match input.chars().next() {
            Some('0') => exit(0),
            Some('1') => add_link(&mut data.links)?,
            Some('2') => add_station(&mut data.stations)?,
            Some('3') => show_link(&mut data.links)?,
            Some('4') => show_station(&mut data.stations)?,
            Some('9') => delete_db(&mut data),
            _ => ()
        }
        save_data(&data);
        println!("\n");
    }

    Ok(())
}
