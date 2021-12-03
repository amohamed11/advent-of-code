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
    part2(lines);
}

fn part1(lines: &Vec<&str>) {
    let pairs = lines.into_iter()
        .map(|line| { let s: Vec<_> = line.split(' ').collect(); (s[0], s[1]) });

    let mut horizontal = 0;
    let mut depth = 0;

    for (dir, dist) in pairs {
        match dir {
            "forward" => horizontal += dist.parse::<i32>().unwrap(),
            "down" => depth += dist.parse::<i32>().unwrap(),
            "up" => depth -= dist.parse::<i32>().unwrap(),
            &_ => depth += 0,
        }
    };

    println!("(Part 1) Depth: {:?}, Horizontal: {:?}, Result: {:?}", depth, horizontal, depth*horizontal);
}

fn part2(lines: Vec<&str>) {
    let pairs = lines.into_iter()
        .map(|line| { let s: Vec<_> = line.split(' ').collect(); (s[0], s[1]) });

    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for (dir, dist) in pairs {
        let x = dist.parse::<i32>().unwrap();
        match dir {
            "forward" => {
                horizontal += x;
                depth += x*aim;
            },
            "down" => {
                aim += x;
            },
            "up" => {
                aim -= x;
            },
            &_ => depth += 0,
        }
    };

    println!("(Part 2) Depth: {:?}, Horizontal: {:?}, Result: {:?}", depth, horizontal, depth*horizontal);
}
