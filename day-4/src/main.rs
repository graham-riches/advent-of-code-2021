extern crate utilities;
pub mod bingo;
use crate::bingo::BingoBoard;
use std::collections::HashSet;

fn main() {
    let lines = utilities::lines_from_file("input.txt").unwrap();
    let numbers: Vec<i32> = lines.iter()
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    
    let mut boards: Vec<BingoBoard>= lines.into_iter()
        .skip(2)
        .collect::<Vec<String>>()
        .chunks(6)
        .map(|x| BingoBoard::from_strings(x, 5).unwrap())
        .collect();

    let mut winners: HashSet<usize> = HashSet::new();
    let total_boards = boards.len();
    for number in numbers {
        for i in 0..total_boards {
            let board = &mut boards[i];
            if board.apply_new_number(number) {
                if winners.insert(i) {
                    let unused: i32 = board.get_all_unset_values().iter().sum();
                    if winners.len() == 1 {
                        println!("Part one solution: {}", unused * number);
                    } else if winners.len() == total_boards {
                        println!("Part two solution: {}", unused * number);
                    }
                }
            }
        }
    }
}

