use std::cmp;
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
    cmp::max((mass / 3) - 2, 0)
}

fn compute_fuel_part2(mass: i64) -> i64 {
    let mut new_fuel: i64 = compute_fuel(mass);
    let mut total_fuel = new_fuel;

    while new_fuel > 0 {
        new_fuel = compute_fuel(new_fuel);
        total_fuel += new_fuel;
    }
    total_fuel
}

fn main() {
    let lines = read_lines("input.in");

    let mut part1_fuel: i64 = 0;
    let mut part2_fuel: i64 = 0;
    for line in lines {
        let mass = line.parse::<i64>().unwrap();
        part1_fuel += compute_fuel(mass);
        part2_fuel += compute_fuel_part2(mass);
    }

    println!("Part 1 Fuel: {}", part1_fuel);
    println!("Part 2 Fuel: {}", part2_fuel);
}
