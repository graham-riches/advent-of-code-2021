extern crate utilities;
use std::fmt;

fn main() {
    let mut grid = DotMatrix::from_dotfile("data/dots.txt");
    let folds: Vec<(usize, usize)> = utilities::lines_from_file("data/folds.txt").unwrap()
        .into_iter()
        .map(|line| line.split(",").map(|c| c.parse::<usize>().unwrap()).collect::<Vec<usize>>())
        .map(|mut x| (x.remove(0), x.remove(0)))
        .collect();
        
    for fold in folds {
        grid.fold(fold);
        println!("Dots Visible: {}", grid.count_dots());
    }    
    println!("{:?}", grid);    
}

struct DotMatrix {
    grid: Vec<Vec<bool>>,
    width: usize,
    height: usize
}

// Custom formatting print for grid structure
impl fmt::Debug for DotMatrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("DotMatrix: \r\n").unwrap();
        for y in 0..self.height {
            for x in 0..self.width {
                f.write_fmt(format_args!("{}", if self.grid[y][x] {'█'} else {' '})).unwrap();
            }
            f.write_str("\r\n").unwrap();
        }
        f.write_str("\r\n").unwrap();
        Ok(())
    }
}

impl DotMatrix {
    fn from_dotfile(path: &str) -> Self {
        let dots: Vec<(i32, i32)> = utilities::lines_from_file(path).unwrap().into_iter()
            .map(|x| x.split(",").map(|y| y.parse::<i32>().unwrap()).collect::<Vec<i32>>())
            .map(|mut x| (x.remove(1), x.remove(0)))
            .collect();
        let mut row_max: i32 = dots.iter().map(|(x, _)| *x).max().unwrap() + 1;
        let mut column_max: i32 = dots.iter().map(|(_, y)| *y).max().unwrap() + 1;
        if row_max % 2 == 0 { row_max += 1; };
        if column_max % 2 == 0 { column_max += 1; };        
        let mut grid = vec![vec![false; column_max as usize]; row_max as usize];
        for dot in dots {            
            grid[dot.0 as usize][dot.1 as usize] = true;
        }
        Self{ grid, width: column_max as usize, height: row_max as usize }
    }

    fn fold(&mut self, fold: (usize, usize)) {
        let height_new = if fold.0 != 0 { self.height } else { fold.1 };
        let width_new = if fold.0 != 0 { fold.0 } else { self.width };        
        for y in 0..height_new {
            for x in 0..width_new {
                self.grid[y][x] |= if fold.0 != 0 {
                    let ind = 2 * width_new - x;
                    if ind >= self.width { false } else { self.grid[y][ind] }                    
                } else {
                    let ind = 2 * height_new - y;
                    if ind >= self.height { false } else { self.grid[ind][x] }                    
                };
            }
        }
        self.width = width_new;
        self.height = height_new;
    }

    fn count_dots(&self) -> i32 {
        let mut count = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if self.grid[y][x] {
                    count += 1;
                }
            }
        }
        count
    }
}