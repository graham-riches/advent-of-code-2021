extern crate utilities;

fn main() {
    let lines = utilities::lines_from_file("input.txt").unwrap();
    let instructions: Vec<(&str, i32)> = lines.iter()
        .map(|x| x.split(' ').collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>() 
        .iter()
        .map(|x| (x[0], x[1].parse::<i32>().unwrap()))
        .collect();

    let part1 = instructions.iter().fold((0, 0), |position, instruction| apply_part_one(&position, &instruction));        
    println!("Part one solution: {}", part1.0 * part1.1 );

    let part2 = instructions.iter().fold((0, 0, 0), |position, instruction| apply_part_two(&position, &instruction));
    println!("Part one solution: {}", part2.0 * part2.1 );
        
}

fn apply_part_one(current_position: &(i32, i32), instruction: &(&str, i32)) -> (i32, i32) {    
    match instruction.0 {
        "forward" => (current_position.0 + instruction.1, current_position.1),
        "down"    => (current_position.0, current_position.1 + instruction.1),
        "up"      => (current_position.0, current_position.1 - instruction.1),
        _         => *current_position
    }    
}

fn apply_part_two(current_position: &(i32, i32, i32), instruction: &(&str, i32)) -> (i32, i32, i32) {
    match instruction.0 {
        "forward" => (current_position.0 + instruction.1, current_position.1 + current_position.2 * instruction.1, current_position.2),
        "down"    => (current_position.0, current_position.1, current_position.2 + instruction.1),
        "up"      => (current_position.0, current_position.1, current_position.2 - instruction.1),
        _         => *current_position
    }
}