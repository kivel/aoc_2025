use std::collections::{HashMap, HashSet};

#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

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

    // each timeline is a unique path from start to end, that's the HashSet inside the Vec
    // The vec contains all timelines with matching tail positions, since timelines can share positions
    // the Mashmap key is the timeline tail position, so we can quickly check if a splitter affects any timelines
    let mut timelines: HashMap<usize, usize> = HashMap::new();
    let start_pos = *&data[0].find('S').unwrap();
    // initialize with starting position
    timelines.insert(start_pos, 1); // we will put the positions in the HashSet when a splitter is encountered
    println!("start timelines: {:?}", timelines);

    // when a splitter is encountered, each timeline splits into two new timelines
    // the last position will be delted, the new timelines will have positions -1 and +1 from the splitter

    for (i, splitter) in splitters.iter().enumerate() {
        println!("processing splitter set {}: {:?}", i + 1, splitter);
        for s in splitter.iter() {
            // println!("processing splitter at position: {}", s);
            if let Some(timelines_at_s) = timelines.remove(s) {
                timelines
                    .entry(s + 1)
                    .and_modify(|counter| *counter += timelines_at_s)
                    .or_insert(timelines_at_s);
                timelines
                    .entry(s - 1)
                    .and_modify(|counter| *counter += timelines_at_s)
                    .or_insert(timelines_at_s);
            }
            // println!("timelines: {:?}", timelines);
        }
    }
    println!("final timelines: {:?}", timelines);
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
        assert_eq!(result, 1555);
    }
}
