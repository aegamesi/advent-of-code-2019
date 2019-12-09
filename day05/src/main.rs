use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_lines(filename: &str) -> impl Iterator<Item=String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|line| line.unwrap())
}

fn simulate(mut mem: Vec<i64>, input: Vec<i64>) {
    let mut pos: usize = 0;
    let mut input_pos = 0;

    loop {
        let opcode = mem[pos] % 100;
        let addressing: i64 = mem[pos] / 100;
        // println!("raw: {}, pos: {}, opcode: {}, addressing: {}", mem[pos], pos, opcode, addressing);

        let get_arg = |mem: &Vec<i64>, arg: usize| {
            let raw = mem[pos + 1 + arg];
            let mode = (addressing / 10_i64.pow(arg as u32)) % 10;
            // println!("get. raw: {}, mode{}", raw, mode);
            match mode {
                0 => mem[raw as usize],
                1 => raw,
                _ => { panic!(); }
            }
        };

        let write_arg = |mem: &mut Vec<i64>, arg: usize, val: i64| {
            let raw = mem[pos + 1 + arg];
            let mode = (addressing / 10_i64.pow(arg as u32)) % 10;
            // println!("set. raw: {}, mode{}", raw, mode);
            match mode {
                0 => mem[raw as usize] = val,
                _ => { panic!(); }
            }
        };

        match opcode {
            1 => {
                let a = get_arg(&mem, 0);
                let b = get_arg(&mem, 1);
                write_arg(&mut mem, 2, a + b);
                pos += 4;
            }
            2 => {
                let a = get_arg(&mem, 0);
                let b = get_arg(&mem, 1);
                write_arg(&mut mem, 2, a * b);
                pos += 4;
            }
            3 => {
                let val = input[input_pos];
                input_pos += 1;
                write_arg(&mut mem, 0, val);
                pos += 2;
            }
            4 => {
                let val = get_arg(&mem, 0);
                println!("vm: {}", val);
                pos += 2;
            }
            5 => {
                let cond = get_arg(&mem, 0);
                let target = get_arg(&mem, 1);
                if cond != 0 {
                    pos = target as usize;
                } else {
                    pos += 3;
                }
            }
            6 => {
                let cond = get_arg(&mem, 0);
                let target = get_arg(&mem, 1);
                if cond == 0 {
                    pos = target as usize;
                } else {
                    pos += 3;
                }
            }
            7 => {
                let a = get_arg(&mem, 0);
                let b = get_arg(&mem, 1);
                let val = (a < b) as i64;
                write_arg(&mut mem, 2, val);
                pos += 4;
            }
            8 => {
                let a = get_arg(&mem, 0);
                let b = get_arg(&mem, 1);
                let val = (a == b) as i64;
                write_arg(&mut mem, 2, val);
                pos += 4;
            }
            99 => { break; }
            _ => {
                panic!("bad opcode: {}", opcode);
            }
        }
    }
}

fn main() {
    let line = read_lines("input.in").nth(0).unwrap();
    let mem: Vec<i64> = line.split(",").map(|x| x.parse::<i64>().unwrap()).collect();
    println!("len: {}", mem.len());

    println!("part 1:");
    let input = [1].to_vec();
    simulate(mem.to_vec(), input);

    println!("part 2:");
    let input = [5].to_vec();
    simulate(mem.to_vec(), input);
}
