use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn read_lines(filename: &str) -> impl Iterator<Item=String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|line| line.unwrap())
}

fn get_total_orbits(orbits: &HashMap<String, String>, start: &str) -> u64 {
    let mut total = 0;
    let mut current = start;
    while current != "COM" {
        current = orbits.get(current).unwrap();
        total += 1;
    }
    total
}

fn main() {
    let mut orbits = HashMap::new();
    for line in read_lines("input.in") {
        let split: Vec<&str> = line.split(")").collect();
        let body = split[0];
        let sat = split[1];

        orbits.insert(sat.to_string(), body.to_string());
    }

    let mut total_orbits = 0;
    for (body, _) in &orbits {
        total_orbits += get_total_orbits(&orbits, body);
    }
    println!("Total orbits: {}", total_orbits);
}
