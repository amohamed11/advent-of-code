use std::env;
use std::fs;

trait UpdateState {
    fn update_state(&mut self, state: bool);
}

struct Cell {
    number: i32,
    marked: bool,
}

impl UpdateState for Cell {
    fn update_state(&mut self, state: bool) {
        self.marked = state;
    }
}

struct Board {
    cells: Vec<Vec<Cell>>,
}

fn row_check(board: &Board) -> bool {
    let mut is_win = false;

    if (board.cells[0][0].marked
        && board.cells[0][1].marked
        && board.cells[0][2].marked
        && board.cells[0][3].marked
        && board.cells[0][4].marked)
        || (board.cells[1][0].marked
            && board.cells[1][1].marked
            && board.cells[1][2].marked
            && board.cells[1][3].marked
            && board.cells[1][4].marked)
        || (board.cells[2][0].marked
            && board.cells[2][1].marked
            && board.cells[2][2].marked
            && board.cells[2][3].marked
            && board.cells[2][4].marked)
        || (board.cells[3][0].marked
            && board.cells[3][1].marked
            && board.cells[3][2].marked
            && board.cells[3][3].marked
            && board.cells[3][4].marked)
        || (board.cells[4][0].marked
            && board.cells[4][1].marked
            && board.cells[4][2].marked
            && board.cells[4][3].marked
            && board.cells[4][4].marked)
    {
        is_win = true;
    }

    is_win
}

fn col_check(board: &Board) -> bool {
    let mut is_win = false;

    if (board.cells[0][0].marked
        && board.cells[1][0].marked
        && board.cells[2][0].marked
        && board.cells[3][0].marked
        && board.cells[4][0].marked)
        || (board.cells[0][1].marked
            && board.cells[1][1].marked
            && board.cells[2][1].marked
            && board.cells[3][1].marked
            && board.cells[4][1].marked)
        || (board.cells[0][2].marked
            && board.cells[1][2].marked
            && board.cells[2][2].marked
            && board.cells[3][2].marked
            && board.cells[4][2].marked)
        || (board.cells[0][3].marked
            && board.cells[1][3].marked
            && board.cells[2][3].marked
            && board.cells[3][3].marked
            && board.cells[4][3].marked)
        || (board.cells[0][4].marked
            && board.cells[1][4].marked
            && board.cells[2][4].marked
            && board.cells[3][4].marked
            && board.cells[4][4].marked)
    {
        is_win = true;
    }

    is_win
}

fn generate_board(block: &str) -> Board {
    let lines = block.split_terminator("\n");
    let mut board = Board {
        cells: Vec::new(),
    };


    for line in lines {
        let mut row: Vec<Cell> = Vec::new();

        line.split_whitespace().for_each(|n| {
            if !n.is_empty() {
                row.push(Cell {
                    number: n.parse().unwrap(),
                    marked: false,
                })
            }
        });

        board.cells.push(row);
    }

    return board;
}

fn mark_cell(board: &mut Board, num: i32) {
    let mut done = false;

    for row in board.cells.iter_mut() {
        if done {
            break;
        }
        for cell in row.iter_mut() {
            if cell.number == num {
                cell.update_state(true);
                done = true;
                break;
            }
        }
    }
}

fn calculate_score(board: &Board, last_num: i32) -> i32 {
    let mut unmarked_sum = 0;
    
    for row in &board.cells {
        row.iter().for_each(|c| {
            if !c.marked {
                unmarked_sum += c.number;
            }
        })
    }

    unmarked_sum * last_num
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).unwrap();

    println!("Reading puzzle input from: {}", filename);
    let contents = fs::read_to_string(filename).expect("Failed to read input file");

    let blocks: Vec<&str> = contents.trim().split("\n\n").collect();

    part1(&blocks);
}

fn part1(blocks: &Vec<&str>) {
    let mut nums: Vec<i32> = Vec::new();
    let mut boards: Vec<Board> = Vec::new();
    let mut game_over = false;

    blocks[0]
        .split_terminator(",")
        .for_each(|n| { nums.push(n.parse().unwrap())});

    for block in blocks.iter().skip(1) {
        if block.is_empty() {
            continue;
        } else {
            let board = generate_board(block);
            boards.push(board);
        }
    }

    for num in nums {
        if game_over {
            break;
        }
        for board in &mut boards {
            mark_cell(board, num);
            if row_check(&board) || col_check(&board) {
                let score = calculate_score(&board, num);
                println!("Score: {:?}", score);
                game_over = true;
                break;
            }
        }
    }


}