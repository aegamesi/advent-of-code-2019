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
}

impl Machine {
    fn new(mem: &Vec<i64>) -> Machine {
        Machine {
            mem: mem.to_vec(),
            pos: 0,
            inputs: Vec::new(),
            outputs: Vec::new(),
            input_pos: 0,
            output_pos: 0,
            status: MachineStatus::Runnable,
        }
    }

    fn get_arg(&self, arg: usize) -> i64 {
        let addressing: i64 = self.mem[self.pos] / 100;
        let raw = self.mem[self.pos + 1 + arg];
        let mode = (addressing / 10_i64.pow(arg as u32)) % 10;
        // println!("get. raw: {}, mode{}", raw, mode);
        match mode {
            0 => self.mem[raw as usize],
            1 => raw,
            _ => { panic!(); }
        }
    }

    fn write_arg(&mut self, arg: usize, val: i64) {
        let addressing: i64 = self.mem[self.pos] / 100;
        let raw = self.mem[self.pos + 1 + arg];
        let mode = (addressing / 10_i64.pow(arg as u32)) % 10;
        // println!("set. raw: {}, mode{}", raw, mode);
        match mode {
            0 => self.mem[raw as usize] = val,
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
                    let a = self.get_arg(0);
                    let b = self.get_arg(1);
                    self.write_arg(2, a + b);
                    self.pos += 4;
                }
                2 => {
                    let a = self.get_arg(0);
                    let b = self.get_arg(1);
                    self.write_arg(2, a * b);
                    self.pos += 4;
                }
                3 => {
                    if self.input_pos < self.inputs.len() {
                        let val = self.inputs[self.input_pos];
                        self.input_pos += 1;
                        self.write_arg(0, val);
                        self.pos += 2;
                    } else {
                        self.status = Blocked;
                        return;
                    }
                }
                4 => {
                    let val = self.get_arg(0);
                    self.outputs.push(val);
                    self.pos += 2;
                }
                5 => {
                    let cond = self.get_arg(0);
                    let target = self.get_arg(1);
                    if cond != 0 {
                        self.pos = target as usize;
                    } else {
                        self.pos += 3;
                    }
                }
                6 => {
                    let cond = self.get_arg(0);
                    let target = self.get_arg(1);
                    if cond == 0 {
                        self.pos = target as usize;
                    } else {
                        self.pos += 3;
                    }
                }
                7 => {
                    let a = self.get_arg(0);
                    let b = self.get_arg(1);
                    let val = (a < b) as i64;
                    self.write_arg(2, val);
                    self.pos += 4;
                }
                8 => {
                    let a = self.get_arg(0);
                    let b = self.get_arg(1);
                    let val = (a == b) as i64;
                    self.write_arg(2, val);
                    self.pos += 4;
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

fn run_amplifiers(mem: &Vec<i64>, phases: Vec<i64>) -> i64 {
    let mut val = 0;
    for phase in phases {
        let mut machine = Machine::new(mem);
        let inputs = vec![phase, val];
        let output = machine.easy_run(&inputs);
        val = output[0];
    }
    val
}

fn run_amplifiers_loop(mem: &Vec<i64>, phases: Vec<i64>) -> i64 {
    let mut val = 0;
    let mut index = 0;
    let mut machines: Vec<Machine> = phases.into_iter().map(|x| {
        let mut machine = Machine::new(mem);
        machine.add_input(x);
        machine
    }).collect();

    loop {
        machines[index].add_input(val);
        machines[index].run();
        val = machines[index].get_output();

        let status = machines[index].get_status();
        match status {
            Finished => {
                if index == machines.len() - 1 {
                    return val;
                }
            },
            Blocked => {}
            _ => { panic!("bad at {}: {:?}", index, status); }
        }

        index = (index + 1) % machines.len();
    }
}

fn main() {
    let line = read_lines("input.in").nth(0).unwrap();
    let mem: Vec<i64> = line.split(",").map(|x| x.parse::<i64>().unwrap()).collect();

    let mut highest = 0;
    let mut phases = vec![0, 1, 2, 3, 4];
    heap_recursive(&mut phases, |permutation| {
        let output = run_amplifiers(&mem, permutation.to_vec());
        if output > highest {
            highest = output;
        }
    });
    println!("part 1 output: {}", highest);

    let mut highest = 0;
    let mut phases = vec![5, 6, 7, 8, 9];
    heap_recursive(&mut phases, |permutation| {
        let output = run_amplifiers_loop(&mem, permutation.to_vec());
        if output > highest {
            highest = output;
        }
    });
    println!("part 2 output: {}", highest);
}
