use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn read_lines(filename: &str) -> impl Iterator<Item=String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|line| line.unwrap())
}

fn get_total_orbits(orbits: &HashMap<String, String>, start: &str, end: &str) -> u64 {
    let mut total = 0;
    let mut current = start;
    while current != end {
        current = orbits.get(current).unwrap();
        total += 1;
    }
    total
}

fn get_parents(orbits: &HashMap<String, String>, start: &str, end: &str) -> Vec<String> {
    let mut path = Vec::new();
    let mut current = start;
    while current != end {
        current = orbits.get(current).unwrap();
        path.push(current.to_string());
    }
    path
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
        total_orbits += get_total_orbits(&orbits, body, "COM");
    }
    println!("Total orbits: {}", total_orbits);

    // Find latest common parent
    let you_path = get_parents(&orbits, "YOU", "COM");
    let san_path = get_parents(&orbits, "SAN", "COM");
    let mut common: &str = "COM";
    let mut i = 1;
    loop {
        if you_path[you_path.len() - i] != san_path[san_path.len() - i] {
            break;
        } else {
            common = &you_path[you_path.len() - i];
            i += 1;
        }
    }

    println!("Common parent: {}", common);

    let transfers = get_total_orbits(&orbits, "YOU", common) +  get_total_orbits(&orbits, "SAN", common) - 2;
    println!("Transfers: {}", transfers);
}
