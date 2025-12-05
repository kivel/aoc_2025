use std::collections::HashSet;

#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

/// Checks if a position containing '@' is accessible for removal.
/// 
/// A position is considered accessible if it has fewer than 4 surrounding '@' characters.

// slower version, because of HashSet operations
// fn is_accessible_intersection(grid: &advent_of_code::Grid, row: usize, col: usize, remaining_positions: &HashSet<(usize, usize)>) -> bool {
//     let surrounding_positions: HashSet<(usize, usize)> = grid.get_surrounding_positions(row, col).into_iter().collect();
//     let count = remaining_positions.intersection(&surrounding_positions).count();
//     count < 4
// }

// faster version, using Vec and direct contains checks
fn is_accessible(grid: &advent_of_code::Grid, row: usize, col: usize, remaining_positions: &HashSet<(usize, usize)>) -> bool {
    let surrounding_positions = grid.get_surrounding_positions(row, col);
    let at_count = surrounding_positions.iter()
        .filter(|pos| remaining_positions.contains(pos))
        .count();
    at_count < 4
}

fn puzzle(data: &Vec<String>) -> u32 {
    let start_time = std::time::Instant::now();
    // lines_to_matrix, aka grid
    let grid = advent_of_code::Grid::from_lines(&data);
    // find all '@' positions
    let mut remaining_positions: HashSet<(usize, usize)> = grid.find_char_positions('@');
    println!("Found <{}> '@'", remaining_positions.len());
    let mut accessible_count = 0;
    
    // Keep removing accessible positions until none are left accessible
    loop {
        // Find and remove accessible positions in one functional step
        let accessible_positions: Vec<(usize, usize)> = remaining_positions.iter()
            .filter(|&&(row, col)| is_accessible(&grid, row, col, &remaining_positions))
            .cloned()
            .collect();
        
        let count = accessible_positions.len();
        println!("Found <{}> accessible '@'", count);
        
        // If no positions are accessible, we're done
        if count == 0 {
            break;
        }
        
        accessible_count += count;
        
        // Remove all accessible positions from the set
        for pos in accessible_positions {
            remaining_positions.remove(&pos);
        }
    }
    
    let final_count = accessible_count;
    println!("Remaining '@' count: {}", final_count);
    let stop_time = std::time::Instant::now();
    let duration = stop_time.duration_since(start_time);
    println!("Duration: {:?}", duration);
    final_count as u32
}

fn main() {
    let d = advent_of_code::Reader::read_file("./input/day04.txt").unwrap();
    let sum = puzzle(&d);
    println!("result: {sum}");
}

#[cfg(test)]
mod tests {
    use crate::{advent_of_code, puzzle};

    #[test]
    fn puzzle_test_data() {
        let d = advent_of_code::Reader::read_file("./input/day04_test.txt").unwrap();
        let result = puzzle(&d);
        println!("result: {result}");
        assert_eq!(result, 43);
    }

    #[test]
    fn puzzle_final_data() {
        let d = advent_of_code::Reader::read_file("./input/day04.txt").unwrap();
        let result = puzzle(&d);
        println!("result: {result}");
        assert_eq!(result, 8690);
    }
}
