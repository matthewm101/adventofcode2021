use std::fs;
use std::collections::hash_set::HashSet;

const CHECKS: [[usize;5];10] = [
    [0,1,2,3,4], [5,6,7,8,9], [10,11,12,13,14], [15,16,17,18,19], [20,21,22,23,24],
    [0,5,10,15,20], [1,6,11,16,21], [2,7,12,17,22], [3,8,13,18,23], [4,9,14,19,24]
];

fn check_board(board: &[u64], marks: &HashSet<u64>) -> Option<u64> {
    let modified_board: Vec<Option<u64>> = board.iter().map(|x| if marks.contains(x) {None} else {Some(*x)}).collect();
    for check in CHECKS {
        if check.iter().map(|x| modified_board[*x].is_none()).all(|x|x) {
            // println!("{:?}",board);
            // println!("{:?}",marks.into_iter().collect::<Vec<&u64>>());
            // println!("{:?}",check);
            return Some(modified_board.iter().map(|x| x.unwrap_or(0)).sum());
        }
    }
    return None;
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let mut lines_iter = file.lines().into_iter();
    let number_list: Vec<u64> = lines_iter.next().unwrap().split(",").map(|s| s.parse().unwrap()).collect();
    let mut boards: Vec<[u64;25]> = Vec::new();
    while lines_iter.next().is_some() {
        let mut board = [0u64;25];
        for i in 0..5 {
            let line = lines_iter.next().unwrap();
            // println!("{}", line);
            let row: Vec<u64> = line.split(" ").filter(|s| !s.is_empty()).map(|s| s.parse().unwrap()).collect();
            for j in 0..5 {
                board[i*5+j] = row[j];
            }
        }
        boards.push(board);
    }
    let mut read_numbers: HashSet<u64> = HashSet::new();
    let mut won_boards: HashSet<usize> = HashSet::new();
    let mut winning_order: Vec<usize> = Vec::new();
    'outer: for n in &number_list {
        read_numbers.insert(*n);
        for board in &boards {
            if let Some(x) = check_board(board, &read_numbers) {
                println!("Remaining value of the first board to win: {}", x);
                println!("Remaining value multipled by winning number: {}", x*n);
                break 'outer;
            }
        }
    }
    'outer: for n in &number_list {
        read_numbers.insert(*n);
        for board_number in 0..boards.len() {
            if !won_boards.contains(&board_number) {
                if let Some(x) = check_board(&boards[board_number], &read_numbers) {
                    if won_boards.len() == boards.len() - 1 {
                        println!("Value of last board to win: {}", x);
                        println!("Last board's value times winning number: {}", x*n);
                        break 'outer;
                    }
                    won_boards.insert(board_number);
                    winning_order.push(board_number);
                }
            }
        }
    }
}
