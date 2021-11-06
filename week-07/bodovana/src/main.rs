use rand::Rng;
use std::sync::{ Arc, Mutex };
use std::thread;
use std::time::Duration;

struct Table {
    forks: Vec<Mutex<()>>,
}

#[derive(Clone)]
struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}

impl Philosopher {
    fn new(name: String, left: usize, right: usize) -> Philosopher {
        Philosopher { name, left: left, right: right }
    }

    fn eat(&self, table: &Table) {
        let sleep_time = rand::thread_rng().gen_range(500..3000);
        thread::sleep(Duration::from_millis(sleep_time));

        println!("{} wants to eat!", self.name);
        let _left = table.forks[self.left].lock().unwrap();
        // Deadlock test
        // thread::sleep(Duration::from_millis(2000));
        println!("{} has left fork!", self.name);
        let _right = table.forks[self.right].lock().unwrap();

        println!("{} started eating!", self.name);
        thread::sleep(Duration::from_millis(sleep_time));
        println!("{} stoped eating!", self.name);
    }
}

fn main() {
    let n = 3;
    let mut philosophers: Vec<Philosopher> = Vec::new();
    let mut forks: Vec<Mutex<()>> = Vec::new();
    for i in 0..n {
        forks.push(Mutex::new(()));

        let name = format!("philosopher {}", i);
        let new_phil = match i {
            x if x == (n - 1) => Philosopher::new(name, 0, n-1),
            _ => Philosopher::new(name, i, i+1)
        };
        philosophers.push(new_phil);
    }
    let table = Arc::new(
        Table { forks: forks }
    );

    let handles: Vec<_> = philosophers.clone().into_iter().map(|p| {
        let table = table.clone();

        thread::spawn(move || {
            loop {
                p.eat(&table);
            }
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }
}
