extern crate utilities;
use std::collections::HashMap;
pub mod lines;
use crate::lines::{Point, Line};

fn main() {
    let input = utilities::lines_from_file("input.txt").unwrap();

    // Part one solution
    let intersections = input
        .iter()
        .map(|x| Line::from_string(x))
        .filter(|x| x.is_horizontal() || x.is_vertical())
        .map(|x| x.get_contained_points())
        .flatten()
        .fold(HashMap::<Point, usize>::new(), |mut m, p| {
            *m.entry(p).or_default() += 1;
            m
        })
        .iter()
        .filter(|&(_, v)| *v > 1)
        .count();
    println!("Part one solution: {:?}", intersections);

    // Part two solution
    let intersections = input
        .iter()
        .map(|x| Line::from_string(x))    
        .map(|x| x.get_contained_points())
        .flatten()
        .fold(HashMap::<Point, usize>::new(), |mut m, p| {
            *m.entry(p).or_default() += 1;
            m
        })
        .iter()
        .filter(|&(_, v)| *v > 1)
        .count();
    println!("Part two solution: {:?}", intersections);
}
