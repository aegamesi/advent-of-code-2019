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

fn main() {
    let lines = read_lines("input.in");
    let mut mem = Vec::new();

    for line in lines {
        let opcode = line.parse::<i64>().unwrap();
        mem.push(opcode);
    }

    mem[1] = 12;
    mem[2] = 2;

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

    println!("Position 0: {}", mem[0]);
}
