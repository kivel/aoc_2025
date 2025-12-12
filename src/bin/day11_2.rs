use std::collections::HashMap;

#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

// Recursive function to count paths from current node to "out"
// inspired by a Copilot suggestion
// cache explored nodes to avoid recomputation
fn count_paths(
    devices: &HashMap<String, Vec<String>>,
    current: &str,
    target: &str,
    cache: &mut HashMap<String, usize>,
) -> usize {
    // Base case: reached target
    if current == target {
        return 1;
    }
    // Check cache first
    if let Some(&cached) = cache.get(current) {
        return cached;
    }
    // Get outputs for current node
    let Some(outputs) = devices.get(current) else {
        cache.insert(current.to_string(), 0);
        return 0; // Dead end
    };
    // Sum paths through all outputs
    let total: usize = outputs
        .iter()
        .map(|next| count_paths(devices, next, target, cache))
        .sum();
    cache.insert(current.to_string(), total);
    total
}

fn devices_from_data(data: &Vec<String>) -> HashMap<String, Vec<String>> {
    data.iter()
        .map(|line| {
            match line.split_once(':') {
                Some(parts) => {
                    return (
                        parts.0.trim().to_string(),
                        parts
                            .1
                            .trim()
                            .split_whitespace()
                            .map(|s| s.trim().to_string())
                            .collect::<Vec<String>>(),
                    );
                }
                None => panic!("Invalid line format: {}", line),
            };
        })
        .collect()
}
// never nesters, brace for impact :D
fn puzzle(data: &Vec<String>) -> usize {
    let devices = devices_from_data(data);
    println!("parsed {} devices", devices.len());
    let mut cache = HashMap::new();
    count_paths(&devices, "svr", "out", &mut cache)
}

fn main() {
    let data = advent_of_code::Reader::read_file("./input/day11.txt").unwrap();

    let result = puzzle(&data);
    println!("result: {}", result);
}

#[cfg(test)]
mod tests {
    use crate::{advent_of_code, puzzle};

    #[test]
    fn puzzle_test_data() {
        let d = advent_of_code::Reader::read_file("./input/day11_test.txt").unwrap();
        let result = puzzle(&d);
        println!("result: {result}");
        assert_eq!(result, 5);
    }

    #[test]
    fn puzzle_final_data() {
        let d = advent_of_code::Reader::read_file("./input/day11.txt").unwrap();
        let result = puzzle(&d);
        println!("result: {result}");
        assert_eq!(result, 615);
    }
}
