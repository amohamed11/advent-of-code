use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).unwrap();

    println!("Reading puzzle input from: {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Failed to read input file");

    let lines: Vec<i32> = contents
        .trim()
        .split("\n")
        .map(|s| s.parse().unwrap())
        .collect();

    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<i32>) {
    let mut measurements = 0;
    let n = lines.len();

    let mut i = 1;
    while i < n {
        let prev: i32 = lines[i-1];
        let current: i32 = lines[i];

        if  prev < current {
            measurements += 1;
        }

        i += 1;
    }

    println!("(Part 1) Measurements: {:?}", measurements);
}

fn part2(lines: &Vec<i32>) {
    let mut measurements = 0;
    let n = lines.len();

    let mut i = 2;
    while i < n-1 {
        let prev: i32 = lines[i-2..=i].iter().sum();
        let current: i32 = lines[i-1..=i+1].iter().sum();

        if  prev < current {
            measurements += 1;
        }

        i += 1;
    }

    println!("(Part 2) Measurements: {:?}", measurements);
}
