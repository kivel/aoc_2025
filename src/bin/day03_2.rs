#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

/// Find the largest digit that appears earliest in the slice
fn find_largest_digit_greedy(digits: &[u32]) -> Option<(u32, usize)> {
    // Try digits 9 down to 1 as first digit
    for first_digit in (1..=9).rev() {
        if let Some(first_pos) = digits.iter().position(|&d| d == first_digit) {
            return Some((first_digit, first_pos));
        }
    }
    None
}
// largest possible joltage, Will be EXCACTLY 12 digits long
fn find_largest_joltage(s: &str) -> u64 {
    let digits: Vec<u32> = s.chars().filter_map(|c| c.to_digit(10)).collect();

    let digits = digits;
    let mut left: usize = 0;
    let mut right: usize = digits.len() - 11; // start with all but the last 11 digits
    let mut joltage: u64 = 0;
    loop {
        let d1 = find_largest_digit_greedy(&digits[left..right]).unwrap();
        left += d1.1 + 1;
        right += 1;
        joltage *= 10;
        joltage += d1.0 as u64;
        if right > digits.len() {
            break;
        }
    }
    joltage
}

// the copilot solution, I didn't know about `scan` at that time. It's elegant, but maybe less readable?
fn find_largest_joltage_elegant(s: &str) -> u64 {
    let digits: Vec<u32> = s.chars().filter_map(|c| c.to_digit(10)).collect();

    // Use iterator-based approach with scan to maintain state
    let window_size = digits.len() - 11;
    (0..12)
        .scan((0, window_size), |(left, right), _| {
            if *right > digits.len() {
                return None;
            }

            let digit_info = find_largest_digit_greedy(&digits[*left..*right])?;
            *left += digit_info.1 + 1;
            *right += 1;

            Some(digit_info.0 as u64)
        })
        .fold(0, |acc, digit| acc * 10 + digit)
}

fn puzzle(data: &Vec<String>) -> u64 {
    data.iter()
        .map(|line| find_largest_joltage(line))
        .sum::<u64>()
}

fn main() {
    let n = find_largest_joltage("234234234234278");
    println!("largest joltage: {}", n);
    let d = advent_of_code::Reader::read_file("./input/day03_test.txt").unwrap();
    let result = puzzle(&d);
    println!("result: {result}");
}

#[cfg(test)]
mod tests {
    use crate::{advent_of_code, puzzle};

    #[test]
    fn test_find_largest_joltage() {
        let result = super::find_largest_joltage_elegant("234234234234278");
        assert_eq!(result, 434234234278);
        let result = super::find_largest_joltage_elegant("818181911112111");
        assert_eq!(result, 888911112111);
        let result = super::find_largest_joltage_elegant("811111111111119");
        assert_eq!(result, 811111111119);
        let result = super::find_largest_joltage_elegant("818181911112111");
        assert_eq!(result, 888911112111);
    }

    #[test]
    fn test_find_largest_digit_greedy() {
        let digits = vec![2, 3, 4, 2];
        let result = super::find_largest_digit_greedy(&digits);
        assert_eq!(result, Some((4, 2)));
    }

    #[test]
    fn puzzle_test_data() {
        let d = advent_of_code::Reader::read_file("./input/day03_test.txt").unwrap();
        let result = puzzle(&d);
        println!("result: {result}");
        assert_eq!(result, 3121910778619);
    }

    #[test]
    fn puzzle_final_data() {
        let d = advent_of_code::Reader::read_file("./input/day03.txt").unwrap();
        let result = puzzle(&d);
        println!("result: {result}");
        assert_eq!(result, 167526011932478);
    }
}
