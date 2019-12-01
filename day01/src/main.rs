use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line.unwrap());
    }

    lines
}

fn compute_fuel(mass: i64) -> i64 {
	(mass / 3) - 2
}

fn main() {
    let lines = read_lines("input.in");

    let mut total_fuel: i64 = 0;
    for line in lines {
    	let mass = line.parse::<i64>().unwrap();
    	total_fuel += compute_fuel(mass);
    }

    println!("Total Fuel: {}", total_fuel);
}
