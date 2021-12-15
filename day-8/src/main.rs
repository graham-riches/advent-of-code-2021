use std::collections::HashSet;
use std::collections::HashMap;
extern crate utilities;
use utilities::set;

fn main() {
    let combinations: Vec<Combination> = utilities::lines_from_file("input.txt").unwrap()
        .into_iter()
        .map(|line| Combination::from_string(&line) )
        .collect();
 
    let part1_lengths: HashSet<usize> = set![2, 4, 3, 7];
    let part1: i32 = combinations.iter()
        .map(|c| c.output.iter().fold(0, |sum, item| sum + part1_lengths.contains(&item.len()) as i32))
        .sum();
    println!("Part one solution: {}", part1);

    let part2 = combinations.iter()
        .fold(0, |sum, c| sum + c.get_output_value(&Numbers::from_combination(&c)));
    println!("Part two solution: {}", part2);
}

#[derive(Debug)]
struct Combination {
    input: Vec<HashSet<char>>,
    output: Vec<HashSet<char>>
}

impl Combination {
    fn from_string(input: &str) -> Self {        
        let mut inputs: Vec<Vec<HashSet<char>>> = input.split('|')
            .map(|i| i.split(' ')
                .map(|s| s.chars().fold(HashSet::<char>::new(), |mut set, c| {set.insert(c); set}))
                .filter(|set| set.len() > 0 )
                .collect::<Vec<HashSet<char>>>())
            .collect();            
        Combination{ input: inputs.remove(0), output: inputs.remove(0) }
    }

    fn get_by<F>(&self, func: F) -> HashSet<char>
    where F: Fn(&HashSet<char>) -> bool 
    {
        self.input.iter()
            .filter(|x| func(x))
            .next()            
            .unwrap()
            .clone()
    }

    fn get_output_value(&self, numbers: &Numbers) -> i32 {
        let mut digit = self.output.len() as u32;
        self.output.iter()
            .fold(0, |sum, o| {
                let num = numbers.get_value_by_set(o) * 10i32.pow(digit - 1);
                digit -= 1;
                sum + num })
    }
}

#[derive(Debug)]
struct Numbers {
    numbers: HashMap<i32, HashSet<char>>
}

impl Numbers {
    fn from_combination(combination: &Combination) -> Self {
        let mut map = HashMap::new();        
        map.insert(1, combination.get_by(|x| x.len() == 2));
        map.insert(4, combination.get_by(|x| x.len() == 4));
        map.insert(7, combination.get_by(|x| x.len() == 3));
        map.insert(8, combination.get_by(|x| x.len() == 7));
        map.insert(3, combination.get_by(|x| x.len() == 5 && map[&1].is_subset(x)));
        map.insert(2, combination.get_by(|x| x.len() == 5 && x.intersection(&map[&4]).count() == 2));
        map.insert(5, combination.get_by(|x| x.len() == 5 && x.intersection(&map[&4]).count() == 3 && x.intersection(&map[&1]).count() == 1));
        map.insert(0, combination.get_by(|x| x.len() == 6 && !map[&5].is_subset(x)));
        map.insert(6, combination.get_by(|x| x.len() == 6 && map[&5].is_subset(x) && !map[&1].is_subset(x)));
        map.insert(9, combination.get_by(|x| x.len() == 6 && map[&5].is_subset(x) && map[&7].is_subset(x)));
        Numbers{ numbers: map }
    }

    fn get_value_by_set(&self, set: &HashSet<char>) -> i32 {
        self.numbers.iter()
            .filter(|&(_, v)| set.intersection(v).count() == set.len() && v.len() == set.len())
            .map(|(k, _)| *k)
            .next()
            .unwrap()
    }
}
