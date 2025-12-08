use std::collections::HashSet;

#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

fn puzzle(data: &Vec<String>) -> u32 {
    let mut positions: HashSet<usize> = HashSet::new();
    positions.insert(*&data[0].find('S').unwrap());
    println!("start positions: {:?}", positions);

    let splitters: Vec<HashSet<usize>> = data
        .iter()
        .filter(|line| line.contains('^'))
        .map(|line| {
            line.char_indices()
                .filter(|(_, c)| *c == '^')
                .map(|(i, _)| i)
                .collect::<HashSet<usize>>()
        })
        .collect();
    println!("all splitter positions: {:?}", splitters);

    let mut total_splits: u32 = 0;
    for splitter in splitters {
        for s in splitter.iter() {
            if positions.remove(s) {
                total_splits += 1;
                positions.insert(s - 1);
                positions.insert(s + 1);
            }
        }
    }

    total_splits
}

fn main() {
    let data = advent_of_code::Reader::read_file("./input/day07_test.txt").unwrap();
    let result = puzzle(&data);
    println!("result: {}", result);
}

#[cfg(test)]
mod tests {
    use crate::{advent_of_code, puzzle};

    #[test]
    fn puzzle_test_data() {
        let d = advent_of_code::Reader::read_file("./input/day07_test.txt").unwrap();
        let result = puzzle(&d);
        println!("result: {result}");
        assert_eq!(result, 21);
    }

    #[test]
    fn puzzle_final_data() {
        let d = advent_of_code::Reader::read_file("./input/day07.txt").unwrap();
        let result = puzzle(&d);
        println!("result: {result}");
        assert_eq!(result, 1555);
    }
}
