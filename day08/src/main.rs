use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_lines(filename: &str) -> impl Iterator<Item=String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|line| line.unwrap())
}

struct Image {
    w: usize,
    h: usize,
    data: Vec<u32>
}

impl Image {
    fn num_layers(&self) -> usize {
        self.data.len() / (self.w * self.h)
    }

    fn get(&self, layer: usize, x: usize, y: usize) -> u32 {
        let index = (self.w * self.h * layer) + (y * self.w) + x;
        self.data[index]
    }

    fn new(w: usize, h: usize, input: &str) -> Image {
        let data = input.chars().map(|x| x).map(|x| x.to_digit(10).unwrap()).collect();
        Image {
            w,
            h,
            data
        }
    }

    fn count_digits(&self, layer: usize, kind: u32) -> u64 {
        let mut count = 0;
        for y in 0..self.h {
            for x in 0..self.w {
                if self.get(layer, x, y) == kind {
                    count += 1;
                }
            }
        }
        count
    }
}

fn main() {
    let line = read_lines("input.in").nth(0).unwrap();
    let image = Image::new(25, 6, &line);

    let mut best = 9999999;
    let mut best_val = 0;
    for i in 0..image.num_layers() {
        let num0 = image.count_digits(i, 0);
        if num0 < best {
            best = num0;
            let num1 = image.count_digits(i, 1);
            let num2 = image.count_digits(i, 2);
            best_val = num1 * num2;
        }
    }
    println!("part 1: layer {} with {}", best, best_val);
}
