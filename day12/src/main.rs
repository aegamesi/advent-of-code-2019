use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_lines(filename: &str) -> impl Iterator<Item=String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|line| line.unwrap())
}

#[derive(Clone)]
struct Body {
    x: i64,
    y: i64,
    z: i64,

    vx: i64,
    vy: i64,
    vz: i64,
}

fn calculate_gravity(a: i64, b: i64) -> i64 {
    match a.cmp(&b) {
        std::cmp::Ordering::Less => 1,
        std::cmp::Ordering::Greater => -1,
        std::cmp::Ordering::Equal => 0,
    }
}

impl Body {
    fn apply_gravity(&self, bodies: &Vec<Body>) -> Body {
        let mut new = self.clone();
        for other in bodies {
            new.vx += calculate_gravity(new.x, other.x);
            new.vy += calculate_gravity(new.y, other.y);
            new.vz += calculate_gravity(new.z, other.z);
        }
        new
    }

    fn apply_velocity(&self) -> Body {
        let mut new = self.clone();
        new.x += self.vx;
        new.y += self.vy;
        new.z += self.vz;
        new
    }

    fn energy(&self) -> i64 {
        let potential = self.x.abs() + self.y.abs() + self.z.abs();
        let kinetic = self.vx.abs() + self.vy.abs() + self.vz.abs();
        potential * kinetic
    }
}

fn main() {
    let mut bodies: Vec<Body> =
        read_lines("input.in")
        .map(|x| x
            .split(",")
            .map(|y| y.parse::<i64>().unwrap())
            .collect()
        )
        .map(|coords: Vec<i64>| Body {
            x: *coords.get(0).unwrap(),
            y: *coords.get(1).unwrap(),
            z: *coords.get(2).unwrap(),
            vx: 0,
            vy: 0,
            vz: 0,
        }).collect();

    let steps = 1000;
    for _ in 0..steps {
        bodies = bodies
            .iter()
            .map(|b| b.apply_gravity(&bodies))
            .map(|b| b.apply_velocity())
            .collect();
    }

    let energy: i64 = bodies.iter().map(|b| b.energy()).sum();
    println!("Part 1 total energy: {}", energy);
}
