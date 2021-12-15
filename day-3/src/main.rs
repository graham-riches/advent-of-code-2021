extern crate utilities;
pub mod binary_functions;

fn main() {
    let input = utilities::lines_from_file("input.txt").unwrap();
    let numbers: Vec<i32> = input.iter()
        .map(|x| i32::from_str_radix(x, 2).unwrap())
        .collect();
    let bit_size = input[0].len();
    let gamma = calculate_gamma(&numbers, bit_size);
    let epsilon = calculate_epsilon(gamma, bit_size);
    println!("Part one solution: {}", gamma * epsilon);

    let oxygen_rating = get_oxygen_generator_rating(&numbers, bit_size);
    let c02_rating = get_co2_scrubber_rating(&numbers, bit_size);
    println!("Part two solution: {}", oxygen_rating * c02_rating);
}

fn calculate_gamma(numbers: &[i32], total_bits: usize) -> i32 {
    let common_bits: Vec<i32> = binary_functions::get_most_common_bits(numbers, total_bits)
        .iter()
        .map(|x| (*x > 0) as i32)
        .collect();
    let mut gamma = 0;
    for i in 0..total_bits {
        gamma |= common_bits[i] << i;
    }
    gamma
}

fn calculate_epsilon(gamma: i32, total_bits: usize) -> i32 {
    let mask = i32::from_str_radix(&"1".repeat(total_bits), 2).unwrap();
    !gamma & mask
}

fn get_oxygen_generator_rating(numbers: &[i32], total_bits: usize) -> i32 {
    binary_functions::get_value_by_most_common_bits(numbers, total_bits, true)
}

fn get_co2_scrubber_rating(numbers: &[i32], total_bits: usize) -> i32 {
    binary_functions::get_value_by_most_common_bits(numbers, total_bits, false)
}

// Tests with sample input
#[test]
fn test_calculate_gamma_rate() {
    let input: Vec<&str> = vec![
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ];
    let numbers: Vec<i32> = input
        .iter()
        .map(|x| i32::from_str_radix(x, 2).unwrap())
        .collect();
    assert_eq!(
        calculate_gamma(&numbers, input[0].len().try_into().unwrap()),
        22
    );
}

#[test]
fn test_calculate_epsilon_rate() {
    assert_eq!(calculate_epsilon(22, 5), 9);
}

#[test]
fn test_calculate_oxygen_generator_rating() {
    let input: Vec<&str> = vec![
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ];
    let numbers: Vec<i32> = input
        .iter()
        .map(|x| i32::from_str_radix(x, 2).unwrap())
        .collect();
    assert_eq!(get_oxygen_generator_rating(&numbers, 5), 23);
}

#[test]
fn test_calculate_co2_scrubber_rating() {
    let input: Vec<&str> = vec![
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ];
    let numbers: Vec<i32> = input
        .iter()
        .map(|x| i32::from_str_radix(x, 2).unwrap())
        .collect();
    assert_eq!(get_co2_scrubber_rating(&numbers, 5), 10);
}
