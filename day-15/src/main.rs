extern crate utilities;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    let risk_map: Vec<Vec<i32>> = utilities::lines_from_file("input.txt").unwrap().into_iter()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as i32).collect::<Vec<i32>>())
        .collect();    
    
    println!("Part one solution: {}", find_min_path((0, 0), ((risk_map.len() - 1) as i32, (risk_map[0].len() - 1) as i32), &risk_map));

    // Some hackery to build part two's map - ugh
    let mut p2_map: Vec<Vec<i32>> = vec![vec![0; 5 * risk_map[0].len()]; 5 *risk_map.len()];
    for r in 0..5 {
        for c in 0..5 {        
            for i in 0..risk_map.len() {
                for j in 0..risk_map[0].len() {
                    let (x, y) = (i + risk_map.len() * r, j + risk_map[0].len() * c);
                    p2_map[x][y] = risk_map[i][j] + c as i32 + r as i32;
                    if p2_map[x][y] > 9 { p2_map[x][y] %= 9 };
                }
            }
        }
    }    
    println!("Part two solution: {}", find_min_path((0, 0), ((p2_map.len() - 1) as i32, (p2_map[0].len() - 1) as i32), &p2_map));
}

fn find_min_path(start: (i32, i32), end: (i32, i32), map: &Vec<Vec<i32>>) -> i32 {
    let mut min_costs: Vec<Vec<i32>> = vec![vec![i32::MAX; map[0].len()]; map.len()];    
    let mut queue = BinaryHeap::new();    
    queue.push(Reverse((0, start.0, start.1)));
    while let Some(Reverse((cost, x, y))) = queue.pop() {
        // Check for end condition
        if (x, y) == end { return cost; }

        // If the current location is worse than the best path already found, bail
        if cost > min_costs[x as usize][y as usize] { continue; }

        // Check cost of going to each neighbour and push new entries onto the heap
        for (r, c) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
            if map.get(r as usize).and_then(|r1| r1.get(c as usize)).is_none() { continue; }            
            let new_cost = cost + map[r as usize][c as usize];
            if new_cost < min_costs[r as usize][c as usize] {                                
                queue.push(Reverse((new_cost, r, c)));
                min_costs[r as usize][c as usize] = new_cost;
            }
        }
    }
    i32::MAX
}