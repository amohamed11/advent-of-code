use std::env;
use std::fs;

struct Cell {
    content: i32,
    marked: bool
}

struct Board {
    cells: Vec<Vec<Cell>>,
    win: bool
}


fn row_check(board: Board) -> bool {
    let mut is_win = false;

    if  (board.cells[0][0].marked &&
            board.cells[0][1].marked &&
            board.cells[0][2].marked &&
            board.cells[0][3].marked &&
            board.cells[0][4].marked) || 
        (board.cells[1][0].marked &&
            board.cells[1][1].marked &&
            board.cells[1][2].marked &&
            board.cells[1][3].marked &&
            board.cells[1][4].marked) || 
        (board.cells[2][0].marked &&
            board.cells[2][1].marked &&
            board.cells[2][2].marked &&
            board.cells[2][3].marked &&
            board.cells[2][4].marked) || 
        (board.cells[3][0].marked &&
            board.cells[3][1].marked &&
            board.cells[3][2].marked &&
            board.cells[3][3].marked &&
            board.cells[3][4].marked) || 
        (board.cells[4][0].marked &&
            board.cells[4][1].marked &&
            board.cells[4][2].marked &&
            board.cells[4][3].marked &&
            board.cells[4][4].marked) {
        is_win = true;
    }

    is_win
}

fn col_check(board: Board) -> bool {
    let mut is_win = false;

    if (board.cells[0][0].marked &&
            board.cells[1][0].marked &&
            board.cells[2][0].marked &&
            board.cells[3][0].marked &&
            board.cells[4][0].marked) ||
        (board.cells[0][1].marked &&
            board.cells[1][1].marked &&
            board.cells[2][1].marked &&
            board.cells[3][1].marked &&
            board.cells[4][1].marked) ||
        (board.cells[0][2].marked &&
            board.cells[1][2].marked &&
            board.cells[2][2].marked &&
            board.cells[3][2].marked &&
            board.cells[4][2].marked) ||
        (board.cells[0][3].marked &&
            board.cells[1][3].marked &&
            board.cells[2][3].marked &&
            board.cells[3][3].marked &&
            board.cells[4][3].marked) ||
        (board.cells[0][4].marked &&
            board.cells[1][4].marked &&
            board.cells[2][4].marked &&
            board.cells[3][4].marked &&
            board.cells[4][4].marked) {
        is_win = true;
    }

    is_win
}

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
    let mut board = Board{
        cells: Vec::new(),
        win: false
    };

    for line in lines {
    }
}

fn part2(lines: &Vec<&str>) {

}
