use std::fs::File;
use std::io::{BufRead, BufReader};
extern crate num_integer;

fn read_lines(filename: &str) -> impl Iterator<Item=String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|line| line.unwrap())
}

#[derive(Clone, Eq)]
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
    fn apply_gravity(&mut self, other: &Body) {
        self.vx += calculate_gravity(self.x, other.x);
        self.vy += calculate_gravity(self.y, other.y);
        self.vz += calculate_gravity(self.z, other.z);
    }

    fn apply_velocity(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
        self.z += self.vz;
    }

    fn energy(&self) -> i64 {
        let potential = self.x.abs() + self.y.abs() + self.z.abs();
        let kinetic = self.vx.abs() + self.vy.abs() + self.vz.abs();
        potential * kinetic
    }

    fn eq_x(&self, other: &Self) -> bool {
        self.x == other.x && self.vx == other.vy
    }

    fn eq_y(&self, other: &Self) -> bool {
        self.y == other.y && self.vy == other.vy
    }

    fn eq_z(&self, other: &Self) -> bool {
        self.z == other.z && self.vz == other.vz
    }
}

impl PartialEq for Body {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x &&
            self.y == other.y &&
            self.z == other.z &&
            self.vx == other.vx &&
            self.vy == other.vy &&
            self.vz == other.vz
    }
}

#[derive(Clone, Eq)]
struct World {
    bodies: Vec<Body>,
}

impl World {
    fn new(input: impl Iterator<Item=String>) -> World {
        let bodies: Vec<Body> =
            input
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

        World {
            bodies,
        }
    }

    fn step(&mut self) {
        let mut new_bodies = self.bodies.clone();
        for body in &mut new_bodies {
            for other in &self.bodies {
                body.apply_gravity(other);
            }
        }
        for body in &mut new_bodies {
            body.apply_velocity();
        }
        self.bodies = new_bodies;
    }

    fn energy(&self) -> i64 {
        self.bodies.iter().map(|b| b.energy()).sum()
    }

    fn eq_x(&self, other: &Self) -> bool {
        self.bodies.iter().zip(other.bodies.iter()).all(|(b1, b2)| b1.eq_x(b2))
    }

    fn eq_y(&self, other: &Self) -> bool {
        self.bodies.iter().zip(other.bodies.iter()).all(|(b1, b2)| b1.eq_y(b2))
    }

    fn eq_z(&self, other: &Self) -> bool {
        self.bodies.iter().zip(other.bodies.iter()).all(|(b1, b2)| b1.eq_z(b2))
    }
}

impl PartialEq for World {
    fn eq(&self, other: &Self) -> bool {
        self.bodies == other.bodies
    }
}

fn step_equality<F>(source: &World, compare: F) -> u64
    where F : Fn(&World, &World) -> bool {
    let mut period = 0;
    let mut world = source.clone();
    loop {
        period += 1;
        world.step();

        if compare(&world, source) {
            break;
        }
    }
    period
}

fn main() {
    let source = World::new(read_lines("input.in"));

    let mut world = source.clone();
    let steps = 1000;
    for _ in 0..steps {
        world.step();
    }
    println!("Part 1 total energy: {}", world.energy());

    // Part 2. ugh
    let x_period = step_equality(&source, |a, b| a.eq_x(b));
    let y_period = step_equality(&source, |a, b| a.eq_y(b));
    let z_period = step_equality(&source, |a, b| a.eq_z(b));
    let mut lcm: u64 = num_integer::lcm(x_period, y_period);
    lcm = num_integer::lcm(lcm, z_period);
    println!("Part 2: {}", lcm);
}
