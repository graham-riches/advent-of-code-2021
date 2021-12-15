use std::fmt;

pub struct BingoBoard {
    data: Vec<Vec<i32>>,
    filled: Vec<Vec<bool>>,
    size: usize
}

impl BingoBoard {
    pub fn new(size: usize) -> Self {
        BingoBoard {
            data: vec![vec![0; size]; size],
            filled: vec![vec![false; size]; size],
            size: size
        }
    }

    pub fn from_strings<'a, I>(lines: I, size: usize) -> Option<Self>
    where
        I: IntoIterator<Item = &'a String>,
    {
        let mut board = Self::new(size);
        let mut iter = lines.into_iter();
        for i in 0..board.size {
            let entries: Vec<i32> = iter.next().unwrap()
                .split(' ')                 
                .map(|x| x.trim().parse::<i32>())
                .flatten()
                .collect();
            for j in 0..board.size {
                board.data[i][j] = entries[j];
            }
        }
        Some(board)
    }

    /// Checks for a bingo
    fn check_for_bingo(&self) -> bool {
        let row_bingo = self.filled.iter()
            .map(|x| x.iter().all(|v| *v))
            .any(|x| x);
        let column_bingo = (0..self.size)
            .map(|x| self.filled.iter()
                .map(|d| d[x])
                .all(|v| v))
            .any(|x| x);
        row_bingo || column_bingo
    }

    /// Applies a new number and returns true for "Bingo"
    pub fn apply_new_number(&mut self, number: i32) -> bool {
        for i in 0..self.size {
            for j in 0..self.size {
                if self.data[i][j] == number {
                    self.filled[i][j] = true;
                }
            }
        }
        self.check_for_bingo()
    }

    pub fn get_all_unset_values(&self) -> Vec<i32> {
        let mut numbers: Vec<i32> = Vec::new();
        for i in 0..self.size {
            for j in 0..self.size {
                if !self.filled[i][j] {
                    numbers.push(self.data[i][j]);
                }
            }
        }
        numbers
    }
}

impl fmt::Debug for BingoBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in &self.data {
            f.write_fmt(format_args!("{:?}\r\n", line)).unwrap();
        }
        Ok(())
    }
}


#[test]
fn test_from_strings() {
    let strings = vec!["22 13 17 11  0", "8  2 23  4 24", "21  9 14 16  7", "6 10  3 18  5", "1 12 20 15 19"];
    let strings: Vec<String> = strings.iter().map(|x| x.to_string()).collect();
    let b = BingoBoard::from_strings(&strings, 5);
    assert_eq!(b.is_none() , false);
    let board = b.unwrap();
    assert_eq!(board.data[0], vec![22, 13, 17, 11, 0]);
    assert_eq!(board.data[1], vec![8, 2, 23, 4, 24]);
    assert_eq!(board.data[2], vec![21, 9, 14, 16, 7]);
    assert_eq!(board.data[3], vec![6, 10, 3, 18, 5]);
    assert_eq!(board.data[4], vec![1, 12, 20, 15, 19]);
}

#[test]
fn test_row_bingo() {
    let strings = vec!["22 13 17 11  0", "8  2 23  4 24", "21  9 14 16  7", "6 10  3 18  5", "1 12 20 15 19"];
    let strings: Vec<String> = strings.iter().map(|x| x.to_string()).collect();
    let mut b = BingoBoard::from_strings(&strings, 5).unwrap();
    let numbers = vec![22, 13, 17, 11, 0];
    let mut result = false;
    for number in numbers {
        result = b.apply_new_number(number);
    }
    assert_eq!(result, true);
}

#[test]
fn test_column_bingo() {
    let strings = vec!["22 13 17 11  0", "8  2 23  4 24", "21  9 14 16  7", "6 10  3 18  5", "1 12 20 15 19"];
    let strings: Vec<String> = strings.iter().map(|x| x.to_string()).collect();
    let mut b = BingoBoard::from_strings(&strings, 5).unwrap();
    let numbers = vec![22, 8, 21, 6, 1];
    let mut result = false;
    for number in numbers {
        result = b.apply_new_number(number);
    }
    assert_eq!(result, true);
}

#[test]
fn test_middle_column_bingo() {
    let strings = vec!["22 13 17 11  0", "8  2 23  4 24", "21  9 14 16  7", "6 10  3 18  5", "1 12 20 15 19"];
    let strings: Vec<String> = strings.iter().map(|x| x.to_string()).collect();
    let mut b = BingoBoard::from_strings(&strings, 5).unwrap();
    let numbers = vec![13, 2, 9, 10, 12];
    let mut result = false;
    for number in numbers {
        result = b.apply_new_number(number);
    }
    assert_eq!(result, true);
}