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
    let mut timelines: HashMap<usize, HashSet<Vec<usize>>> = HashMap::new();
    let start_pos = *&data[0].find('S').unwrap();
    // initialize with starting position
    timelines.insert(start_pos, HashSet::from([vec![]])); // we will put the positions in the HashSet when a splitter is encountered
    println!("start timelines: {:?}", timelines);

    // when a splitter is encountered, each timeline splits into two new timelines
    // the last position will be delted, the new timelines will have positions -1 and +1 from the splitter

    for (i, splitter) in splitters.iter().enumerate() {
        println!("processing splitter set {}: {:?}", i + 1, splitter);
        for s in splitter.iter() {
            // println!("processing splitter at position: {}", s);
            if let Some(timelines_at_s) = timelines.remove(s) {
                let t: HashSet<Vec<usize>> = timelines_at_s
                    .into_iter()
                    .map(|mut positions| {
                        positions.push(*s);
                        positions
                    })
                    .collect();
                // println!("timelines at s: {:?}", t);
                if let Some(left) = timelines.get_mut(&(s - 1)) {
                    left.extend(t.clone());
                    // left.insert(t.clone());
                } else {
                    timelines.insert(s - 1, t.clone());
                }
                if let Some(right) = timelines.get_mut(&(s + 1)) {
                    right.extend(t.clone());
                } else {
                    timelines.insert(s + 1, t.clone());
                }
            }
            // println!("timelines: {:?}", timelines);
        }
    }
    println!("final timelines: {:?}", timelines);
    timelines.iter().map(|(_, v)| v.len()).sum()
    // 0
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
