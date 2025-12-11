use std::{
    collections::{HashSet, VecDeque},
    fmt,
};
#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
struct Machine {
    led: u32,
    buttons: Vec<u32>,
    joltage: Vec<u32>,
}

impl fmt::Display for Machine {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{:#08b} | {:?}", self.led, self.buttons)
    }
}

impl Machine {
    fn new(led: u32, buttons: Vec<u32>, joltage: Vec<u32>) -> Self {
        Machine {
            led,
            buttons,
            joltage,
        }
    }

    // Helper to convert tuple indices to bitmask
    fn tuple_to_mask(indices: &Vec<u32>) -> u32 {
        let mut mask = 0u32;
        indices.iter().for_each(|i| mask |= 1 << i);
        mask
    }

    // example str: "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"
    fn from_str(s: &str) -> Self {
        let mut parts: Vec<&str> = s.split(' ').collect();
        let mut led = 0u32;
        let led_str = parts[0];
        println!("parsing led string: {}", led_str);
        led_str
            .trim_matches(&['[', ']'][..])
            .char_indices()
            .inspect(|c| println!(" --> {:?}", c))
            .filter_map(|(i, c)| match c {
                '#' => Some(i as u32),
                _ => None,
            })
            .for_each(|i| led |= 1 << i);
        println!("parsed led mask: {:#b}", led);

        let joltage_str = parts.pop().unwrap().trim_matches(&['{', '}'][..]);
        let joltage: Vec<u32> = joltage_str
            .split(',')
            .filter_map(|part| part.trim().parse::<u32>().ok())
            .collect();

        let buttons: Vec<u32> = parts
            .iter()
            .skip(1)
            .map(|s| s.trim_matches(&['(', ')'][..]))
            .map(|bs| {
                bs.split(',')
                    .filter_map(|part| part.trim().parse::<u32>().ok())
                    .collect::<Vec<u32>>()
            })
            .map(|bv| Machine::tuple_to_mask(&bv))
            .collect();
        Machine::new(led, buttons, joltage)
    }
}

fn solve(machine: &Machine) -> Option<usize> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    // Start from state 0 with 0 operations
    queue.push_back((0u32, 0usize));
    visited.insert(0u32);

    while let Some((state, ops)) = queue.pop_front() {
        // Check if we reached target
        if state == machine.led {
            return Some(ops);
        }

        // Try applying each tuple
        for &button in machine.buttons.iter() {
            let new_state = state ^ button; // XOR operation

            if !visited.contains(&new_state) {
                visited.insert(new_state);
                queue.push_back((new_state, ops + 1));
            }
        }
    }
    None // No solution found
}

fn puzzle(data: &Vec<String>) -> usize {
    let machines: Vec<Machine> = data.iter().map(|line| Machine::from_str(line)).collect();
    machines.iter().map(|m| solve(m).unwrap_or_default()).sum()
}

fn main() {
    let data = advent_of_code::Reader::read_file("./input/day10_test.txt").unwrap();
    let result = puzzle(&data);
    println!("result: {}", result);
}

#[cfg(test)]
mod tests {
    use crate::{advent_of_code, puzzle};

    #[test]
    fn puzzle_test_data() {
        let d = advent_of_code::Reader::read_file("./input/day10_test.txt").unwrap();
        let result = puzzle(&d);
        println!("result: {result}");
        assert_eq!(result, 7);
    }

    #[test]
    fn puzzle_final_data() {
        let d = advent_of_code::Reader::read_file("./input/day10.txt").unwrap();
        let result = puzzle(&d);
        println!("result: {result}");
        assert_eq!(result, 417);
    }
}
