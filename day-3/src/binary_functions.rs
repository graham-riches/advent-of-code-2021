/// Get the most common set bits in a sequence of numbers
/// For each x:
///   x < 0: 0 is the most common bit
///   x > 0: 1 is the most common bit
///   x == 0: 0 and 1 are equally common
pub fn get_most_common_bits(numbers: &[i32], total_bits: usize) -> Vec<i32> {
    let count: Vec<i32> = vec![0; total_bits];
    numbers.iter()
        .fold(count, |count, value| sum_bits(count, *value))
}

/// Helper to sum bits into a vector that maintains a running total of set bits
fn sum_bits(mut count: Vec<i32>, number: i32) -> Vec<i32> {
    for j in 0..count.len() {
        count[j] += if (number & (0x01 << j)) > 0 { 1 } else { -1 }
    }
    count
}

/// Returns a value out of a collection by iteratively filtering based on the most or least common bits set
pub fn get_value_by_most_common_bits(numbers: &[i32], total_bits: usize, most_common: bool) -> i32 {
    let mut values: Vec<i32> = numbers.to_vec();
    for i in 0..total_bits {
        let common_bits: Vec<i32> = get_most_common_bits(&values, total_bits)
            .into_iter()
            .map(|x| match x {
                x if x > 0 => most_common as i32,
                x if x < 0 => !most_common as i32,
                x if x == 0 => most_common as i32,
                _ => 0,
            })
            .rev()
            .collect();

        values = values
            .into_iter()
            .filter(|x| (*x >> (total_bits - i - 1) & (0x01)) == common_bits[i])
            .collect();
        if values.len() == 1 {
            break;
        }
    }
    values[0]
}
