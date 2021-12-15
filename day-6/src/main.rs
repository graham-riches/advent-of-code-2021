extern crate utilities;
use std::collections::HashMap;

fn main() {
    let fish: Vec<i32> = utilities::lines_from_file("input.txt")
        .unwrap()
        .into_iter()
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut population = Population::new(&fish);
    println!(
        "Part one solution: {}",
        population.get_population_after_days(80)
    );

    let mut population = Population::new(&fish);
    println!(
        "Part two solution: {}",
        population.get_population_after_days(256)
    );
}

struct Population {
    timers: HashMap<i32, usize>,
}

impl Population {
    fn new(fish: &[i32]) -> Self {
        let mut timers: HashMap<i32, usize> = HashMap::new();
        for i in 0..=8 {
            *timers.entry(i).or_default() += fish.iter().filter(|x| **x == i).count();
        }
        Population { timers }
    }

    fn update(&mut self) {
        let temp = self.timers.clone();
        for day in (0..=8).rev() {
            *self.timers.get_mut(&day).unwrap() = *temp.get(&((day + 1) % 9)).unwrap();
        }
        *self.timers.get_mut(&6).unwrap() += *self.timers.get(&8).unwrap();
    }

    fn count_population(&self) -> usize {
        self.timers.iter().fold(0, |count, (_, v)| count + v)
    }

    fn get_population_after_days(&mut self, days: usize) -> usize {
        for _ in 1..=days {
            self.update();
        }
        self.count_population()
    }
}
