extern crate utilities;
use std::collections::HashMap;
use itertools::Itertools;
use itertools::MinMaxResult::{NoElements, OneElement, MinMax};

fn main() {
    let lines = utilities::lines_from_file("input.txt").unwrap();
    
    let rules: HashMap<&str, &str> = lines.iter()
        .skip(2)
        .fold(HashMap::<&str, &str>::new(), |mut map, line| { 
            let mut iter = line.split(" -> ");
            map.insert(iter.next().unwrap(), iter.next().unwrap());
            map});

    let mut map: HashMap<String, usize> = lines[0].chars()
        .zip(lines[0].chars().skip(1))
        .fold(HashMap::<String, usize>::new(), |mut m, (a, b)| {
            *m.entry(format!("{}{}", a, b)).or_default() += 1;
            m            
        });
    
    let mut counts: HashMap<char, usize> = lines[0].chars()
        .fold(HashMap::<char, usize>::new(), |mut m, c| {
            *m.entry(c).or_default() += 1;
            m
        });

    for _ in 0..40 {    
        let temp = map.clone();
        for (k, count) in temp {            
            match rules.get(&k as &str) {
                Some(v) => {
                    *map.entry(k.to_string()).or_default() -= count;
                    *map.entry(format!{"{}{}", k.chars().next().unwrap(), v}).or_default() += count;
                    *map.entry(format!{"{}{}", v, k.chars().skip(1).next().unwrap()}).or_default() += count;
                    *counts.entry(v.chars().next().unwrap()).or_default() += count;
                },
                None => ()
            }            
        }
    }

    let solution: usize = match counts.iter().map(|(_, v)| *v).minmax() {
        NoElements       => 0,
        OneElement(_)    => 0,
        MinMax(min, max) => max - min
    };        
    println!("Solution: {}", solution);    
}
