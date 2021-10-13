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
    println!("\n");
    println!("Usage:");
    println!("0 - Shutdown");
    println!("1 - Add link");
    println!("2 - Add station");
    println!("3 - Show link");
    println!("4 - Show station");
    println!("5 - Is station on link");
    println!("6 - Waht is the next station on link");
    println!("7 - Are 2 link connected");
    println!("8 - Save DB");
    println!("9 - Delete DB");
    println!();
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

fn is_station_on_link(links: &mut HashMap<usize, Link>, link: usize, current_station: &String) -> bool {
    let stations = &links.get(&link).unwrap().stations;

    stations.iter().any(|station| station.eq(current_station))
}

fn test_is_station_on_link(links: &mut HashMap<usize, Link>) -> std::io::Result<()> {
    println!("Enter link: ");
    let link: usize = read_line()?.parse().unwrap();
    println!("Enter station: ");
    let current_station = read_line()?;

    match is_station_on_link(links, link, &current_station) {
        true => println!("YES! Station '{}' is on Link {}.", current_station, link),
        false => println!("NO! Station '{}' is not on Link {}.", current_station, link),
    }
    Ok(())
}

fn next_station(links: &mut HashMap<usize, Link>) -> std::io::Result<()> {
    println!("Enter link: ");
    let link: usize = read_line()?.parse().unwrap();
    println!("Enter station: ");
    let current_station = read_line()?;

    if !is_station_on_link(links, link, &current_station) {
        println!("Station '{}' is not on Link {}.", current_station, link);
        return Ok(());
    }

    let stations = &links.get(&link).unwrap().stations;
    let index = stations.iter()
        .position(|station| station.eq(&current_station)).unwrap();
    match stations.get(index + 1) {
        Some(station) => println!("Next station is: {}", station),
        _ => println!("There is no next station for Link {}!", link),
    }

    Ok(())
}

fn are_connected(links: &mut HashMap<usize, Link>) -> std::io::Result<()> {
    println!("Enter first link: ");
    let link: usize = read_line()?.parse().unwrap();
    println!("Enter second link: ");
    let link2: usize = read_line()?.parse().unwrap();

    let stations = &links.get(&link).unwrap().stations;
    let stations2 = &links.get(&link2).unwrap().stations;

    match stations.iter().any(|st1| stations2.contains(st1)) {
        true => println!("Station '{}' is connected to station '{}'", link, link2),
        false => println!("Station '{}' is not connected to station '{}'", link, link2),
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut data = load_data();

    loop {
        print_help();

        match read_line()?.chars().next() {
            Some('0') => exit(0),
            Some('1') => add_link(&mut data.links)?,
            Some('2') => add_station(&mut data.stations)?,
            Some('3') => show_link(&mut data.links)?,
            Some('4') => show_station(&mut data.stations)?,
            Some('5') => test_is_station_on_link(&mut data.links)?,
            Some('6') => next_station(&mut data.links)?,
            Some('7') => are_connected(&mut data.links)?,
            Some('8') => save_data(&mut data),
            Some('9') => delete_db(&mut data),
            _ => ()
        }
    }
}
