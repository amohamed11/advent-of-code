use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).unwrap();

    println!("Reading puzzle input from: {}", filename);
    let contents = fs::read_to_string(filename).expect("Failed to read input file");

    let lines: Vec<&str> = contents.trim().split("\n").collect();

    part1(&lines);
    // part2(&lines);
}

/*
00100
11110  =>  21320
10110

if col_sum > ceil(row_count/2) => 1 else 0
*/

fn part1(lines: &Vec<&str>) {
    let mut sums = HashMap::new();
    let mut gamma_bits = Vec::new();
    let mut epsilon_bits = Vec::new();
    let half = (lines.len()/2) as i32;

    for line in lines {
        // Split into individual bits
        let bits = line
            .split_terminator("")
            .skip(1)
            .map(|b| b.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        // Column sums update
        bits.iter()
            .enumerate()
            .for_each(|(i, b)| *sums.entry(i).or_insert(0) += b);
    }

    for i in 0..sums.len() {
        if sums[&i] as i32 > half {
            gamma_bits.push("1");
            epsilon_bits.push("0");
        } else {
            gamma_bits.push("0");
            epsilon_bits.push("1");
        }
    }

    let gamma_b =  gamma_bits.join("");
    let epsilon_b = epsilon_bits.join("");
    let gamma =  isize::from_str_radix(&gamma_b, 2).unwrap();
    let epsilon =  isize::from_str_radix(&epsilon_b, 2).unwrap();

    println!("-----------------");
    println!("Part 1");
    println!("Bytes => gamma: {}, epsilon: {}", gamma_b, epsilon_b);
    println!("Decimal => gamma: {:?}, epsilon: {:?}", gamma, epsilon);
    println!("Answer: {:?}", gamma*epsilon);
    println!("-----------------");
}

// fn part2(lines: &Vec<&str>) {

// }
