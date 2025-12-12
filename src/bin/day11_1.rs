use std::collections::HashMap;

#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

// Recursive function to count paths from current node to "out"
// inspired by a Copilot suggestion
fn count_paths(devices: &HashMap<String, Vec<String>>, current: &str) -> usize {
    // Base case: reached target
    if current == "out" {
        return 1;
    }
    // Get outputs for current node
    let Some(outputs) = devices.get(current) else {
        return 0; // Dead end
    };
    // Sum paths through all outputs
    outputs.iter().map(|next| count_paths(devices, next)).sum()
}

// never nesters, brace for impact :D
fn puzzle(data: &Vec<String>) -> usize {
    let devices: HashMap<String, Vec<String>> = data
        .iter()
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
        .collect();
    println!("parsed {} devices", devices.len());
    count_paths(&devices, "you")
}

fn main() {
    let data = advent_of_code::Reader::read_file("./input/day11_test.txt").unwrap();

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
