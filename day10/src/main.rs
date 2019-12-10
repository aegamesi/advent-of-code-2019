use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use std::cmp::max;

fn read_lines(filename: &str) -> impl Iterator<Item=String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|line| line.unwrap())
}

struct Asteroid {
    x: i64,
    y: i64,
}

fn count_line_of_sight(asteroids: &Vec<Asteroid>, start: &Asteroid) -> usize {
    let mut angles = Vec::new();
    for a in asteroids {
        let dx = start.x - a.x;
        let dy = start.y - a.y;
        if dx == 0 && dy == 0 {
            continue;
        }
        let angle = (dy as f64).atan2(dx as f64);

        let mut did_see = false;
        for other in &angles {
            if ((angle - *other) as f64).abs() < 0.00001 {
                did_see = true;
                break;
            }
        }
        if !did_see {
            angles.push(angle);
        }
    }
    angles.len()
}

fn main() {
    let lines = read_lines("input.in");

    let mut asteroids = Vec::new();
    for (y, line) in lines.enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                asteroids.push(Asteroid { x: (x as i64), y: (y as i64) });
            }
        }
    }
    println!("Asteroids: {}", asteroids.len());

    let mut best = 0;
    for asteroid in &asteroids {
        best = max(best, count_line_of_sight(&asteroids, asteroid));
    }
    println!("Part 1: {}", best);
}
