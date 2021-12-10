use std::cmp;
use std::env;
use std::fs;

#[derive(Debug)]
struct Vent {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).unwrap();

    println!("Reading puzzle input from: {}", filename);
    let contents = fs::read_to_string(filename).expect("Failed to read input file");

    let lines: Vec<&str> = contents.trim().split("\n").collect();

    part1(&lines);
}

fn part1(lines: &Vec<&str>) {
    let mut vents: Vec<Vent> = Vec::new();
    let mut overlap_vents = 0;

    for line in lines {
        let pairs = line.split(" -> ");
        let mut nums: Vec<i32> = Vec::new();

        for pair in pairs {
            pair.split(",").for_each(|n| nums.push(n.parse().unwrap()));
        }

        if nums[0] == nums[2] || nums[1] == nums[3] {
            let vent = Vent {
                x1: nums[0],
                y1: nums[1],
                x2: nums[2],
                y2: nums[3],
            };

            println!("{:?}", vent);
            vents.push(vent);
        }

        for (i, a) in vents.iter().enumerate() {
            let a_minX = cmp::min(a.x1, a.x2);
            let a_maxX = cmp::max(a.x1, a.x2);
            let a_minY = cmp::min(a.y1, a.y2);
            let a_maxY = cmp::max(a.y1, a.y2);

            for b in vents.iter().skip(i + 1) {
                let b_minX = cmp::min(b.x1, b.x2);
                let b_maxX = cmp::max(b.x1, b.x2);
                let b_minY = cmp::min(b.y1, b.y2);
                let b_maxY = cmp::max(b.y1, b.y2);

                // Coincidence check
                // Horizontal
                if (a.y1 == b.y1) || (a.y2 == b.y2) {
                    if a_minX >= b_maxX {
                        overlap_vents += 1;
                        continue;
                    }
                }
            }
        }
    }

    println!("Overlapping vents: {:?}", overlap_vents);
}
