use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).unwrap();

    println!("Reading puzzle input from: {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Failed to read input file");

    let lines: Vec<&str> = contents
        .trim()
        .split("\n")
        .collect();

    for line in &lines {
        println!("Line: {}", line);
    }

    part1(&lines);
    // part2(&lines);
}

fn part1(lines: &Vec<&str>) {
    let mut matrix = Vec::new();

   for line in lines {
        let v: Vec<i32> = line.split("").map(|x| x.parse::<i32>().unwrap()).collect();
        matrix.push(v);
    }
    // for line in lines {
    //     let mut mcb = HashMap::from([
    //         (1, 0),
    //         (0, 0)
    //     ]);
    // }
}

// fn part2(lines: &Vec<&str>) {

// }
