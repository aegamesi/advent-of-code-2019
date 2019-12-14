use std::fs::File;
use std::io::{BufRead, BufReader};
use permutohedron::heap_recursive;
use crate::MachineStatus::{BadOpcode, Finished, Blocked};

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

    fn get_output(&mut self) -> i64 {
        let val = self.outputs[self.output_pos];
        self.output_pos += 1;
        val
    }

    fn get_status(&self) -> MachineStatus {
        self.status
    }
}


fn main() {
    let line = read_lines("input.in").nth(0).unwrap();
    let mem: Vec<i64> = line.split(",").map(|x| x.parse::<i64>().unwrap()).collect();

    // Part 1.
    let mut machine = Machine::new(&mem);
    machine.add_input(1);
    machine.run();
    println!("Part 1: {}", machine.get_output());
}
