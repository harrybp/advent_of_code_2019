use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut total_fuel: i32 = 0;
    for line in reader.lines() {
        let line: String = line.unwrap(); // Ignore errors.
        let mass: i32 = line.parse::<i32>().unwrap();
        let fuel: i32 = calculate_fuel(mass);
        total_fuel += fuel;
    }

    println!("{}", total_fuel);
}

fn calculate_fuel(mass: i32) -> i32 {
    return (mass/3) -2;
}
