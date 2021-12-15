extern crate utilities;
use std::collections::HashSet;

fn main() {
    let map = HeightMap::from_file("input.txt").unwrap();
    let part1: i32 = map.get_low_spots().iter()
        .map(|p| p.value + 1)
        .sum();
    println!("Part one solution: {}", part1);

    let mut basin_sizes = map.count_basin_sizes(&map.get_low_spots());
    basin_sizes.sort_by(|a, b| b.cmp(a));
    let part2 = basin_sizes.iter()        
        .take(3)
        .fold(1, |total, x| total * x);
    println!("Part two solution: {}", part2);
}

struct HeightMap {
    grid: Vec<Vec<i32>>,
    rows: usize,
    columns: usize
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Point {
    x: i32,
    y: i32,
    value: i32
}

impl HeightMap {
    fn from_file(path: &str) -> Option<Self> {
        let map: Vec<Vec<i32>> = utilities::lines_from_file(path).unwrap()
            .into_iter()
            .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as i32).collect::<Vec<i32>>())
            .collect();
        let rows: usize = &map.len() - 1;
        let columns: usize = &map[0].len() - 1;
        Some(HeightMap{ grid: map, rows, columns })
    }

    fn get_low_spots(&self) -> Vec<Point> {                
        self.grid.iter()
            .enumerate()
            .flat_map(|(i, row)| row.iter()
                .enumerate()
                .flat_map(|(j, element)| if self.get_neighbours(i, j)                    
                    .iter()
                    .all(|x| element < &x.value)
                        {
                            Some( Point{ x: i as i32, y: j as i32, value: *element })
                        } else {
                            None
                        })
                    .collect::<Vec<Point>>())
            .collect::<Vec<Point>>()        
    }

    fn get_neighbours(&self, row: usize, column: usize) -> Vec<Point> {        
        let top    = if row == 0               { i32::MAX } else { self.grid[row - 1][column] };
        let bottom = if row == self.rows       { i32::MAX } else { self.grid[row + 1][column] };
        let left   = if column == 0            { i32::MAX } else { self.grid[row][column - 1] };
        let right  = if column == self.columns { i32::MAX } else { self.grid[row][column + 1] };
        vec![Point{ x: row as i32,    y: column as i32 - 1, value: left}, 
            Point{ x: row as i32,     y: column as i32 + 1, value: right},
            Point{ x: row as i32 - 1, y: column as i32,     value: top},
            Point{ x: row as i32 + 1, y: column as i32,     value: bottom}]
    }

    fn count_basin_sizes(&self, low_spots: &[Point]) -> Vec<i32> {
        let mut basin_sizes = Vec::new();
        for point in low_spots {
            let mut basin_set: HashSet<Point> = HashSet::new();
            basin_set.insert(point.clone());
            self.count_increasing_height_neighbours(&point, &mut basin_set);            
            basin_sizes.push(basin_set.len() as i32);
        }
        basin_sizes
    }

    fn count_increasing_height_neighbours(&self, point: &Point, set: &mut HashSet<Point>) -> () {        
        let increasing_neighbours: Vec<Point> = self.get_neighbours(point.x as usize, point.y as usize)
            .into_iter()
            .filter(|n| n.value < 9 && n.value > point.value)
            .collect();        
        for neighbour in increasing_neighbours {
            set.insert(neighbour.clone());
            self.count_increasing_height_neighbours(&neighbour, set);
        }
    }
}
