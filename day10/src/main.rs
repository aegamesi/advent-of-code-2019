use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_lines(filename: &str) -> impl Iterator<Item=String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|line| line.unwrap())
}

#[derive(Clone)]
struct Asteroid {
    x: i64,
    y: i64,
}

impl Asteroid {
    fn angle_to(&self, other: &Asteroid) -> f64 {
        let dx = (other.x - self.x) as f64;
        let dy = (other.y - self.y) as f64;
        (-dy).atan2(dx)
    }

    fn dist2_to(&self, other: &Asteroid) -> f64 {
        let dx = (other.x - self.x) as f64;
        let dy = (other.y - self.y) as f64;
        (dx * dx) + (dy * dy)
    }
}

const FLOAT_DELTA: f64 = 0.00001;

fn float_compare(a: f64, b: f64) -> bool {
    ((a - b) as f64).abs() < FLOAT_DELTA
}

fn angle_delta(x: f64, mut y: f64) -> f64 {
    if y > x {
        y -= (2 as f64) * std::f64::consts::PI;
    }
    x - y
}

fn count_line_of_sight(asteroids: &Vec<Asteroid>, start: &Asteroid) -> usize {
    let mut angles = Vec::new();
    for a in asteroids {
        let dx = start.x - a.x;
        let dy = start.y - a.y;
        if dx == 0 && dy == 0 {
            continue;
        }
        let angle = start.angle_to(a);

        let mut did_see = false;
        for other in &angles {
            if float_compare(angle, *other) {
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

fn get_destroyed(asteroids: &Vec<Asteroid>, laser: &Asteroid, count: usize) -> Asteroid {
    let mut asteroids = asteroids.to_vec();
    // Inefficient but easier than sorting.
    let mut current_angle = (1 as f64).atan2(0 as f64);
    current_angle += (2 as f64) * FLOAT_DELTA;
    let mut num = 0;
    loop {
        num += 1;
        let mut next: i64 = -1;
        let mut lowest_delta: f64 = 99999 as f64;
        let mut lowest_distance: f64 = 99999 as f64;
        for (i, other) in asteroids.iter().enumerate() {
            let dist = laser.dist2_to(other);
            let angle = laser.angle_to(other);
            let delta = angle_delta(current_angle, angle);
            if (delta < lowest_delta - FLOAT_DELTA) || (float_compare(delta, lowest_delta) && dist < lowest_distance) {
                lowest_delta = delta;
                lowest_distance = dist;
                next = i as i64;
            }
        }
        let removed = asteroids.remove(next as usize);
        current_angle = laser.angle_to(&removed) - (2 as f64 * FLOAT_DELTA);
        println!("{} -> {}, {}", num, removed.x, removed.y);
        if num == count {
            return removed;
        }
    }
}

fn main() {
    let lines = read_lines("input.in");

    let mut laser = Asteroid {x: 0, y: 0};
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
        let num = count_line_of_sight(&asteroids, asteroid);
        if num > best {
            best = num;
            laser.x = asteroid.x;
            laser.y = asteroid.y;
        }
    }
    println!("Part 1: {}", best);

    let destroyed = get_destroyed(&asteroids, &laser, 200);
    println!("Part 2: {}", (destroyed.x * 100) + destroyed.y);
}
