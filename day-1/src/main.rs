extern crate utilities;

fn main() {
    // Read in the data
    let readings: Vec<i32> = utilities::lines_from_file("input.txt")
        .unwrap()
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    println!("Part one: {}", count_depth_increases(&readings, 1));
    println!("Part two: {}", count_depth_increases(&readings, 3));
}

/// Sliding window count of increasings depths
fn count_depth_increases(readings: &[i32], window_size: usize) -> i32 {
    let windowed_readings: Vec<i32> = readings
        .windows(window_size)
        .map(|x| x.iter().sum::<i32>())
        .collect();

    let next = windowed_readings.iter().skip(1);
    windowed_readings
        .iter()
        .zip(next)
        .map(|(current, next)| next - current)
        .fold(0, |acc, x| acc + ((x > 0) as i32))
}

#[test]
fn test_part_one() {
    let readings = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    assert_eq!(count_depth_increases(&readings, 1), 7);
}

#[test]
fn test_part_two() {
    let readings = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    assert_eq!(count_depth_increases(&readings, 3), 5);
}
