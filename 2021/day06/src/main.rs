use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).unwrap();

    println!("Reading puzzle input from: {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Failed to read input file");

    let lines: Vec<&str> = contents.trim().split("\n").collect();

    part1(&lines);
}

fn part1(lines: &Vec<&str>) {
    let cycles = 80;
    let mut school: Vec<i32> = Vec::new(); 

    for line in lines {
        for fish in line.split_terminator(",") {
            school.push(fish.parse().unwrap());
        }
    }

    for _ in 0..cycles {
        for i in 0..school.len() {
            if school[i] == 0 {
                school[i] = 6;
                school.push(8);
            } else {
                school[i] -= 1;
            }
        }
    }

    println!("School size: {:?}", school.len());
}
