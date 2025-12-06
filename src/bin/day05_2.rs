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

// Major EBCAK, I need more memory, or have to grow a brain...
// This takes TB of memory for large ranges...
// fn pair_insert_into_HashSet(pair: (u64, u64), set: &mut HashSet<u64>) {
//     for num in pair.0..=pair.1 {
//         set.insert(num);
//     }
// }

fn merge_ranges_into_vec(pairs: &Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut merged: Vec<(u64, u64)> = Vec::new();
    // initialize with first range
    merged.push(pairs[0]);
    for &(start, end) in pairs[1..].iter() {
        let last = merged.last_mut().unwrap();
        if start <= last.1 + 1 {
            // Ranges overlap or are contiguous, merge them
            last.1 = last.1.max(end); // max to deal containing ranges
        } else {
            // No overlap, add new range
            merged.push((start, end));
        }
    }
    merged
}

// The puzlle calls for two lists (given as two columns in a ascii file) to be sorted and line by line the absolute differences need to be summed up.
fn puzzle(data: &Vec<String>) -> usize {
    // read ranges until empty line
    let mut ranges = data
        .iter()
        .take_while(|line| !line.is_empty())
        .map(|line| parse_u64_pair(&line, '-').unwrap())
        .inspect(|pair| println!("Inspecting pair before insert: {:?}", pair)) // Side effect here
        .collect::<Vec<(u64, u64)>>();
    ranges.sort();
    // println!("Parsed ranges: {:?}", ranges);
    let merged_ranges = merge_ranges_into_vec(&ranges);
    // println!("Merged ranges: {:?}", merged_ranges);
    merged_ranges
        .iter()
        .map(|(start, end)| (end - start + 1) as usize)
        .sum::<usize>()
}

fn main() {
    let data = advent_of_code::Reader::read_file("./input/day05_test.txt").unwrap();
    let sum = puzzle(&data);
    println!("result: {sum}");
}

#[cfg(test)]
mod tests {
    use crate::{advent_of_code, puzzle};

    #[test]
    fn merge_ranges_into_vec_test() {
        let mut input = vec![(5, 10), (1, 3), (2, 6), (15, 20), (18, 22), (19, 20)];
        input.sort();
        let expected = vec![(1, 10), (15, 22)];
        let result = super::merge_ranges_into_vec(&input);
        assert_eq!(result, expected);
    }

    #[test]
    fn puzzle_test_data() {
        let d = advent_of_code::Reader::read_file("./input/day05_test.txt").unwrap();
        let result = puzzle(&d);
        println!("result: {result}");
        assert_eq!(result, 14);
    }

    #[test]
    fn puzzle_final_data() {
        let d = advent_of_code::Reader::read_file("./input/day05.txt").unwrap();
        let result = puzzle(&d);
        println!("result: {result}");
        assert_eq!(result, 338693411431456);
    }
}
