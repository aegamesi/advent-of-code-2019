use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::MachineStatus::{BadOpcode, Finished, Blocked};
use std::collections::HashMap;
use std::cmp::{min, max};


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

fn run_robot(mut map: HashMap<(i32, i32), i64>, mut machine: Machine) -> HashMap<(i32, i32), i64> {
    let mut x = 0;
    let mut y = 0;
    let mut dx = 0;
    let mut dy = -1;

    loop {
        machine.run();
        match machine.status {
            MachineStatus::Finished => { break; },
            MachineStatus::Runnable => { continue; },
            MachineStatus::Blocked => {
                // Get input:
                let key = (x, y);
                let space = map.get(&key);
                let color = *space.unwrap_or(&0);
                machine.add_input(color);
                machine.run();

                // See if there's output
                let out1 = machine.get_output();
                let out2 = machine.get_output();
                if out1.is_some() {
                    let new_color = out1.unwrap();
                    let rotation = out2.unwrap();
                    map.insert(key, new_color);

                    // Rotate.
                    let old_dx = dx;
                    let old_dy = dy;
                    match rotation {
                        0 => {
                            // Turn left.
                            dx = old_dy;
                            dy = -old_dx;
                        }
                        1 => {
                            // Turn right.
                            dx = -old_dy;
                            dy = old_dx;
                        }
                        _ => { panic!("invalid rotation"); }
                    }

                    x += dx;
                    y += dy;
                }
            },
            _ => { panic!("invalid machine state"); }
        };
    }

    map
}

fn print_map(map: HashMap<(i32, i32), i64>) {
    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;

    for (pos, _) in map.iter() {
        min_x = min(min_x, pos.0);
        min_y = min(min_y, pos.1);
        max_x = max(max_x, pos.0);
        max_y = max(max_y, pos.1);
    }

    for y in min_y..(max_y + 1) {
        for x in min_x..(max_x + 1) {
            let color = *map.get(&(x, y)).unwrap_or(&0);
            if color != 0 {
                print!("\u{2588}");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}


fn main() {
    let line = read_lines("input.in").nth(0).unwrap();
    let mem: Vec<i64> = line.split(",").map(|x| x.parse::<i64>().unwrap()).collect();

    // Part 1.
    let mut machine = Machine::new(&mem);
    let tiles = run_robot(HashMap::new(), machine);
    println!("Num Tiles: {}", tiles.len());

    // Part 2.
    let mut machine = Machine::new(&mem);
    let mut map = HashMap::new();
    map.insert((0, 0), 1);
    let map = run_robot(map, machine);
    print_map(map);
}
