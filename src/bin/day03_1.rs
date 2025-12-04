#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

fn find_largest_pair(s: &str) -> u32 {
    let digits: Vec<u32> = s.chars().filter_map(|c| c.to_digit(10)).collect();
    
    // Try digits 9 down to 1 as first digit
    for first_digit in (1..=9).rev() {
        if let Some(first_pos) = digits.iter().position(|&d| d == first_digit) {
            // Simply find the max in the remaining slice - no iteration needed!
            // Note: In case the highest number is in the last position(s), this will still work correctly.
            // Because the `Some` will fail because the iter().max() will return None --> empty slice.
            if let Some(&max_second) = digits[(first_pos + 1)..].iter().max() {
                return first_digit * 10 + max_second;
            }
        }
    }
    0
}

// The puzlle calls for two lists (given as two columns in a ascii file) to be sorted and line by line the absolute differences need to be summed up.
fn puzzle(data: &Vec<String>) -> u32 {
    data.iter()
        .map(|line| find_largest_pair(line))
        .sum::<u32>()
}

fn main() {
    let d = advent_of_code::Reader::read_file("./input/day3_1_test.txt").unwrap();
    let sum = puzzle(&d);
    println!("result: {sum}");
}

#[cfg(test)]
mod tests {
    use crate::{advent_of_code, puzzle, find_largest_pair};

    #[test]
    fn test_find_largest_pair() {
        let result = find_largest_pair("234239423423427");
        assert_eq!(result, 97);
    }

    #[test]
    fn puzzle_test_data() {
        let d = advent_of_code::Reader::read_file("./input/day03_test.txt").unwrap();
        let result = puzzle(&d);
        println!("result: {result}");
        assert_eq!(result, 357);
    }

    #[test]
    fn puzzle_final_data() {
        let d = advent_of_code::Reader::read_file("./input/day03.txt").unwrap();
        let result = puzzle(&d);
        println!("result: {result}");
        assert_eq!(result, 16854);
    }
}
