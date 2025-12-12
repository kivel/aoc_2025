use std::collections::{HashMap, HashSet};

#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

// Recursive function to count paths from current node to "out"
// inspired by a Copilot suggestion
// cache explored nodes to avoid recomputation
fn count_paths(
    devices: &HashMap<String, Vec<String>>,
    current: &str,
    target: &str,
    visited: &HashSet<String>, // Nodes visited on this path
    cache: &mut HashMap<(String, bool, bool), usize>, // (node, has_dac, has_fft) -> count
) -> usize {
    let has_dac = visited.contains("dac");
    let has_fft = visited.contains("fft");

    // Base case: reached target and has both dac and fft
    if current == target {
        return if has_dac && has_fft { 1 } else { 0 };
    }
    // Check cache with state
    let cache_key = (current.to_string(), has_dac, has_fft);
    if let Some(&cached) = cache.get(&cache_key) {
        return cached;
    }
    // Get outputs for current node
    let Some(outputs) = devices.get(current) else {
        cache.insert(cache_key, 0);
        return 0; // Dead end
    };

    // Add current to visited set
    let mut new_visited = visited.clone();
    new_visited.insert(current.to_string());

    // Sum paths through all outputs
    let total: usize = outputs
        .iter()
        .map(|next| count_paths(devices, next, target, &new_visited, cache))
        .sum();
    cache.insert(cache_key, total);
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
    let visited = HashSet::new();
    count_paths(&devices, "svr", "out", &visited, &mut cache)
}

fn main() {
    let data = advent_of_code::Reader::read_file("./input/day11_test2.txt").unwrap();

    let result = puzzle(&data);
    println!("result: {}", result);
}

#[cfg(test)]
mod tests {
    use crate::{advent_of_code, puzzle};

    #[test]
    fn puzzle_test_data() {
        let d = advent_of_code::Reader::read_file("./input/day11_test2.txt").unwrap();
        let result = puzzle(&d);
        println!("result: {result}");
        assert_eq!(result, 2);
    }

    #[test]
    fn puzzle_final_data() {
        let d = advent_of_code::Reader::read_file("./input/day11.txt").unwrap();
        let result = puzzle(&d);
        println!("result: {result}");
        assert_eq!(result, 615);
    }
}
