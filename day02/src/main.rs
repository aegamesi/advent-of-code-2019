use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_lines(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line.unwrap());
    }

    lines
}

fn simulate(mut mem: Vec<i64>) -> i64 {
    let mut pos = 0;
    loop {
        match mem[pos] {
            1 => {
                let a = mem[pos + 1] as usize;
                let b = mem[pos + 2] as usize;
                let c = mem[pos + 3] as usize;
                mem[c] = mem[a] + mem[b];
                pos += 4;
            }
            2 => {
                let a = mem[pos + 1] as usize;
                let b = mem[pos + 2] as usize;
                let c = mem[pos + 3] as usize;
                mem[c] = mem[a] * mem[b];
                pos += 4;
            }
            99 => { break; }
            _ => { panic!(); }
        }
    }
    mem[0]
}

fn main() {
    let lines = read_lines("input.in");
    let mut mem = Vec::new();

    for line in lines {
        let opcode = line.parse::<i64>().unwrap();
        mem.push(opcode);
    }

    mem[1] = 12;
    mem[2] = 2;
    let output = simulate(mem.to_vec());

    println!("Output (12, 2): {}", output);

    let target = 19690720;
    for a in 0..99 {
        for b in 0..99 {
            let mut mem2 = mem.to_vec();
            mem2[1] = a;
            mem2[2] = b;
            let output = simulate(mem2);
            println!("({}, {}) => {}", a, b, output);

            if output == target {
                println!("============= ANSWER: {}", a * 100 + b);
                return;
            }
        }
    }
}
