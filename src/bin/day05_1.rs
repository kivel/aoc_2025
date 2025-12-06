use std::collections::HashSet;

#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

fn parse_u64_pair(s: &str, delimiter: char) -> Option<(u64, u64)> {
    // Attempt to split the string once by the given delimiter.
    let (first_str, second_str) = s.split_once(delimiter)?;
    // Attempt to parse the first part into a u64.
    let first_num = first_str.parse::<u64>().ok()?;
    // Attempt to parse the second part into a u64.
    let second_num = second_str.parse::<u64>().ok()?;
    // If both parsing operations succeed, return the tuple.
    Some((first_num, second_num))
}

fn is_fresh(ingredient: &u64, ranges: &Vec<(u64, u64)>) -> bool {
    ranges.iter().any(|(min, max)| ingredient >= min && ingredient <= max)
}

// The puzlle calls for two lists (given as two columns in a ascii file) to be sorted and line by line the absolute differences need to be summed up.
fn puzzle(data: &Vec<String>) -> usize {
    let mut lines = data.into_iter();
    // read ranges until empty line
    let ranges= lines.by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| parse_u64_pair(&line, '-').unwrap())
        .collect::<Vec<(u64, u64)>>();
    // read ingredients from remaining lines
    let ingredients = lines.filter_map(|line| line.parse().ok()).collect::<HashSet<u64>>();

    let fresh_count = ingredients.iter()
        .filter(|&ingredient| is_fresh(ingredient, &ranges))
        .count();
    fresh_count
}


fn main() {
    let data = advent_of_code::Reader::read_file("./input/day05.txt").unwrap();
    let sum = puzzle(&data);
    println!("result: {sum}");
}

#[cfg(test)]
mod tests {
    use crate::{advent_of_code, puzzle};

    #[test]
    fn puzzle_test_data() {
        let d = advent_of_code::Reader::read_file("./input/day05_test.txt").unwrap();
        let result = puzzle(&d);
        println!("result: {result}");
        assert_eq!(result, 3);
    }

    #[test]
    fn puzzle_final_data() {
        let d = advent_of_code::Reader::read_file("./input/day05.txt").unwrap();
        let result = puzzle(&d);
        println!("result: {result}");
        assert_eq!(result, 563);
    }
}
