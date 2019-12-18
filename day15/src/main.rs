use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::MachineStatus::{BadOpcode, Finished, Blocked};
use std::collections::HashMap;

fn read_lines(filename: &str) -> impl Iterator<Item=String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|line| line.unwrap())
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum MachineStatus {
    Runnable,
    Blocked,
    Finished,
    BadOpcode(i64),
}

struct Machine {
    mem: Vec<i64>,
    pos: usize,
    inputs: Vec<i64>,
    outputs: Vec<i64>,
    input_pos: usize,
    output_pos: usize,
    status: MachineStatus,
    relative_base: i64,
}

impl Machine {
    fn new(mem: &Vec<i64>) -> Machine {
        let mut new_mem = Vec::new();
        new_mem.extend(mem);
        for _ in 0..1000 {
            new_mem.push(0);
        }

        Machine {
            mem: new_mem,
            pos: 0,
            inputs: Vec::new(),
            outputs: Vec::new(),
            input_pos: 0,
            output_pos: 0,
            status: MachineStatus::Runnable,
            relative_base: 0,
        }
    }

    fn arg(&mut self, arg: usize) -> &mut i64 {
        let addressing: i64 = self.mem[self.pos] / 100;
        let mode = (addressing / 10_i64.pow(arg as u32)) % 10;
        match mode {
            0 => {
                let addr = self.mem[self.pos + 1 + arg];
                &mut self.mem[addr as usize]
            },
            1 => &mut self.mem[self.pos + 1 + arg],
            2 => {
                let val = self.mem[self.pos + 1 + arg];
                &mut self.mem[(self.relative_base + val) as usize]
            }
            _ => { panic!(); }
        }
    }

    fn run(&mut self) {
        match self.status {
            BadOpcode(_) => { return; },
            Finished => { return; },
            _ => {}
        }

        loop {
            let opcode = self.mem[self.pos] % 100;
            // println!("raw: {}, pos: {}, opcode: {}, addressing: {}", mem[pos], pos, opcode, addressing);

            match opcode {
                1 => {
                    let a = *self.arg(0);
                    let b = *self.arg(1);
                    *self.arg(2) =  a + b;
                    self.pos += 4;
                }
                2 => {
                    let a = *self.arg(0);
                    let b = *self.arg(1);
                    *self.arg(2) = a * b;
                    self.pos += 4;
                }
                3 => {
                    if self.input_pos < self.inputs.len() {
                        let val = self.inputs[self.input_pos];
                        self.input_pos += 1;
                        *self.arg(0) = val;
                        self.pos += 2;
                    } else {
                        self.status = Blocked;
                        return;
                    }
                }
                4 => {
                    let val = *self.arg(0);
                    self.outputs.push(val);
                    self.pos += 2;
                }
                5 => {
                    let cond = *self.arg(0);
                    let target = *self.arg(1);
                    if cond != 0 {
                        self.pos = target as usize;
                    } else {
                        self.pos += 3;
                    }
                }
                6 => {
                    let cond = *self.arg(0);
                    let target = *self.arg(1);
                    if cond == 0 {
                        self.pos = target as usize;
                    } else {
                        self.pos += 3;
                    }
                }
                7 => {
                    let a = *self.arg(0);
                    let b = *self.arg(1);
                    let val = (a < b) as i64;
                    *self.arg(2) = val;
                    self.pos += 4;
                }
                8 => {
                    let a = *self.arg(0);
                    let b = *self.arg(1);
                    let val = (a == b) as i64;
                    *self.arg(2) = val;
                    self.pos += 4;
                }
                9 => {
                    let val = *self.arg(0);
                    self.relative_base += val;
                    self.pos += 2;
                }
                99 => {
                    self.status = Finished;
                    return;
                }
                _ => {
                    self.status = BadOpcode(opcode);
                    return;
                }
            }
        }
    }

    fn easy_run(&mut self, inputs: &Vec<i64>) -> &Vec<i64> {
        self.add_inputs(inputs);
        self.run();
        &self.outputs
    }

    fn add_input(&mut self, input: i64) {
        self.inputs.push(input);
    }

    fn add_inputs(&mut self, inputs: &Vec<i64>) {
        self.inputs.extend(inputs);
    }

    fn get_output(&mut self) -> Option<i64> {
        if self.output_pos < self.outputs.len() {
            let val = self.outputs[self.output_pos];
            self.output_pos += 1;
            Some(val)
        } else {
            None
        }
    }

    fn get_status(&self) -> MachineStatus {
        self.status
    }
}

#[derive(Copy, Clone)]
enum Direction {
    None,
    North,
    East,
    South,
    West
}

impl Direction {
    fn number(&self) -> i64 {
        match self {
            Direction::None => 0,
            Direction::North => 1,
            Direction::South => 2,
            Direction::West => 3,
            Direction::East => 4,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum TileKind {
    Empty,
    Wall,
    Oxygen,
    Unknown,
}

impl TileKind {
    fn passable(&self) -> bool {
        self == &TileKind::Empty || self == &TileKind::Oxygen
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Position(i64, i64);

impl Position {
    fn neighbor(&self, direction: Direction) -> Position {
        match direction {
            Direction::None => Position(self.0, self.1),
            Direction::North => Position(self.0, self.1 - 1),
            Direction::East => Position(self.0 + 1, self.1),
            Direction::South => Position(self.0, self.1 + 1),
            Direction::West => Position(self.0 - 1, self.1),
        }
    }

    fn neighbors(&self) -> Vec<Position> {
        vec![
            self.neighbor(Direction::North),
            self.neighbor(Direction::East),
            self.neighbor(Direction::South),
            self.neighbor(Direction::West),
        ]
    }

    fn direction(&self, other: &Position) -> Direction {
        if self.0 < other.0 {
            Direction::East
        } else if self.0 > other.0 {
            Direction::West
        } else if self.1 > other.1 {
            Direction::North
        } else if self.1 < other.1 {
            Direction::South
        } else {
            Direction::None
        }
    }
}

struct Tile {
    kind: TileKind,
    prev: Position,
}

struct Robot {
    machine: Machine,
    pos: Position,
    tile: TileKind
}

impl Robot {
    fn new(machine: Machine) -> Robot {
        Robot {
            machine,
            pos: Position(0, 0),
            tile: TileKind::Empty,
        }
    }

    fn step(&mut self, dir: &Direction) -> TileKind {
        self.machine.add_input(dir.number());
        self.machine.run();
        let status = self.machine.get_output();
        match status {
            Some(0) => TileKind::Wall,
            Some(1) => TileKind::Empty,
            Some(2) => TileKind::Oxygen,
            _ => TileKind::Unknown,
        }
    }
}

struct World {
    mem: Vec<i64>,
    tiles: HashMap<Position, TileKind>,
    backpointers: HashMap<Position, Position>,
}

impl World {
    fn new(mem: &Vec<i64>) -> World {
        let mut tiles = HashMap::new();

        World {
            mem: mem.to_vec(),
            tiles,
            backpointers: HashMap::new(),
        }
    }

    fn get_path(&self, target: &Position) -> Vec<Direction> {
        let mut backwards = Vec::new();
        let mut curr = target;
        loop {
            let prev = self.backpointers.get(curr);
            match prev {
                None => { break; }
                Some(prev) => {
                    if prev == curr {
                        break;
                    }
                    backwards.push(prev.direction(curr));
                    curr = prev;
                }
            };
        }
        backwards.reverse();
        backwards
    }

    fn get_type(&self, pos: &Position) -> TileKind {
        match self.tiles.get(pos) {
            None => TileKind::Unknown,
            Some(tile) => *tile,
        }
    }

    fn explore(&mut self) {
        let mut fringe: Vec<(Position, Position)> = Vec::new();
        fringe.push((Position(0, 0), Position(0, 0)));

        while fringe.len() > 0 {
            let (prev, curr) = fringe.remove(0);
            if self.tiles.contains_key(&curr) {
                continue;
            }

            let mut robot = Robot::new(Machine::new(&self.mem));
            let state = self.get_path(&curr).iter().map(|dir| robot.step(dir)).last();
            let tile = state.unwrap_or(TileKind::Empty);

            self.tiles.insert(curr, tile);
            self.backpointers.insert(curr, prev);

            if tile != TileKind::Wall {
                for neighbor in curr.neighbors() {
                    if !self.tiles.contains_key(&neighbor) {
                        fringe.push((curr, neighbor));
                        self.backpointers.insert(neighbor, curr);
                    }
                }
            }
        }
    }
}


fn main() {
    let line = read_lines("input.in").nth(0).unwrap();
    let mem: Vec<i64> = line.split(",").map(|x| x.parse::<i64>().unwrap()).collect();

    // Part 1.
    let mut world = World::new(&mem);
    world.explore();
    let (oxygen_pos, _) = world.tiles.iter().filter(|(k, v)| **v == TileKind::Oxygen).next().unwrap();
    println!("pos: {}, {}", oxygen_pos.0, oxygen_pos.1);
    let path = world.get_path(oxygen_pos);
    println!("path len: {}", path.len());
}
