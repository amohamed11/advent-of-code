use std::cmp;
use std::env;
use std::fs;

#[derive(Debug)]
struct Vent {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    points: Vec<(i32, i32)>,
}

fn generate_points(x1: i32, y1: i32, x2: i32, y2: i32) -> Vec<(i32, i32)> {
    let min_x = cmp::min(x1, x2);
    let max_x = cmp::max(x1, x2);
    let min_y = cmp::min(y1, y2);
    let max_y = cmp::max(y1, y2);

    let mut points: Vec<(i32, i32)> = Vec::new();

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            points.push((x, y));
        }
    }

    points
}

// fn angle_is_45(nums: &Vec<i32>) -> bool {
//     // tan(45) = 1, tan(x) = opposite/adjacent
//     // I knew trig would be useful one day
//     let adjacent = i32::abs(nums[0] - nums[2]);
//     let opposite = i32::abs(nums[1] - nums[3]);

//     if adjacent == 0 {
//         return false;
//     }

//     let tan = opposite/adjacent;

//     return i32::abs(tan) == 1;
// }

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
    let mut map_max_x = 0;
    let mut map_max_y = 0;

    for line in lines {
        let pairs = line.split(" -> ");
        let mut nums: Vec<i32> = Vec::new();

        for pair in pairs {
            pair.split(",").for_each(|n| nums.push(n.parse().unwrap()));
        }

        if nums[0] == nums[2]
            || nums[1] == nums[3]
        {
            let points = generate_points(nums[0], nums[1], nums[2], nums[3]);

            let vent = Vent {
                x1: nums[0],
                y1: nums[1],
                x2: nums[2],
                y2: nums[3],
                points: points,
            };

            vents.push(vent);

            map_max_x = cmp::max(map_max_x, cmp::max(nums[0], nums[2]));
            map_max_y = cmp::max(map_max_y, cmp::max(nums[1], nums[3]));
        }
    }

    println!("Processed input & generated vents.");
    println!("Plane (max_x, max_y): ({:?}, {:?})", map_max_x, map_max_y);

    // this looks weird but it works, i guess
    let mut map = vec![vec![0; (map_max_y + 1) as usize]; (map_max_x + 1) as usize];

    for v in &vents {
        for point in &v.points {
            map[point.0 as usize][point.1 as usize] += 1;
        }
    }

    for x in 0..map.len() {
        for y in 0..map[x].len() {
            if map[x][y] >= 2 {
                overlap_vents += 1;
            }
        }
    }

    println!("Overlapping vents: {:?}", overlap_vents);
}
