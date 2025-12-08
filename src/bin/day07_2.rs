use std::collections::{HashMap, HashSet};

#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

// this puzzle is like those pin pyramids where a ball falls down and splits at each fork
fn puzzle(data: &Vec<String>) -> usize {
    // splitter positions, represented by '^'
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

    // each timeline is a unique path from start to end, but it's enough to count how many timelines reach each position
    let mut timelines: HashMap<usize, usize> = HashMap::new();
    let start_pos = *&data[0].find('S').unwrap();
    // initialize with starting position
    timelines.insert(start_pos, 1); // we will put the positions in the HashSet when a splitter is encountered
    println!("start timelines: {:?}", timelines);

    // when a splitter is encountered, each timeline splits into two new timelines
    for splitter in splitters {
        // all splitters in that line
        for s in splitter.iter() {
            // each splitter position
            if let Some(timelines_at_s) = timelines.remove(s) {
                // remove the tachion at that position
                // create two new timelines at s+1 and s-1 (increase counter if already exists)
                timelines
                    .entry(s + 1)
                    .and_modify(|counter| *counter += timelines_at_s)
                    .or_insert(timelines_at_s);
                timelines
                    .entry(s - 1)
                    .and_modify(|counter| *counter += timelines_at_s)
                    .or_insert(timelines_at_s);
            }
        }
    }
    timelines.iter().map(|(_, v)| *v).sum()
    // 0usize
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
        assert_eq!(result, 40);
    }

    #[test]
    fn puzzle_final_data() {
        let d = advent_of_code::Reader::read_file("./input/day07.txt").unwrap();
        let result = puzzle(&d);
        println!("result: {result}");
        assert_eq!(result, 12895232295789);
    }
}
