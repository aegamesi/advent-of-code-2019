use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_lines(filename: &str) -> impl Iterator<Item=String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|line| line.unwrap())
}

fn fft(digits: &Vec<i32>) -> Vec<i32> {
    let pattern: [i32; 4] = [0, 1, 0, -1];
    let mut out = Vec::new();
    for out_pos in 0..digits.len() {
        let mut acc: i32 = 0;
        for (in_pos, digit) in digits.iter().enumerate() {
            let p = pattern[((in_pos + 1) / (out_pos + 1)) % 4];

            acc += p * (*digit);
        }
        out.push((acc.abs() % 10) as i32);
    }
    out
}

fn mini_print(digits: &Vec<i32>) {
    for i in 0..8 {
        print!("{}", digits[i]);
    }
    println!();
}

fn main() {
    let digits: Vec<i32> = read_lines("input.in").next().unwrap().bytes().map(|x| (x - 48).into()).collect();

    let mut part1 = digits;
    for _ in 0..100 {
        part1 = fft(&part1);
    }
    mini_print(&part1);
}
