extern crate utilities;

fn main() {
    let positions: Vec<i32> = utilities::lines_from_file("input.txt")
        .unwrap()
        .into_iter()
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let min: i32 = *positions.iter().min().unwrap();
    let max: i32 = *positions.iter().max().unwrap();    

    let part1 = (min..=max)        
        .map(|x| positions.iter().fold(0, |sum, p| sum + (p - x).abs()))
        .min()
        .unwrap();
    println!("Part one solution: {}", part1);

    let part2 = (min..=max)        
        .map(|x| positions.iter().fold(0, |sum, p| sum + (0..=((p - x).abs())).sum::<i32>()))
        .min()
        .unwrap();
    println!("Part two solution: {}", part2);
}
