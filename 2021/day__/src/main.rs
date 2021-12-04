use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).unwrap();

    println!("Reading puzzle input from: {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Failed to read input file");

    let lines: Vec<&str> = contents.trim().split("\n").collect();

    for line in &lines {
        println!("Line: {}", line);
    }

    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<&str>) {

}

fn part2(lines: &Vec<&str>) {

}
