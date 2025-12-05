use std::collections::HashSet;

#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

/// Checks if a position containing '@' is accessible for removal.
/// 
/// A position is considered accessible if it has fewer than 4 surrounding '@' characters.
/// This function examines all 8 surrounding positions (including diagonals) and counts
/// how many of them contain '@' characters that are still remaining in the grid.
/// 
/// # Arguments
/// 
/// * `grid` - The grid containing the characters
/// * `row` - The row index of the position to check
/// * `col` - The column index of the position to check
/// * `remaining_positions` - Vector of positions that still contain '@' characters
/// 
/// # Returns
/// 
/// Returns `true` if the position has fewer than 4 surrounding '@' characters,
/// making it accessible for removal. Returns `false` otherwise.
fn is_accessible(grid: &advent_of_code::Grid, row: usize, col: usize, remaining_positions: &HashSet<(usize, usize)>) -> bool {
    let surrounding_positions = grid.get_surrounding_positions(row, col);
    let at_count = surrounding_positions.iter()
        .filter(|pos| remaining_positions.contains(pos))
        .count();
    at_count < 4
}

// The puzlle calls for two lists (given as two columns in a ascii file) to be sorted and line by line the absolute differences need to be summed up.
fn puzzle(data: &Vec<String>) -> u32 {
    // lines_to_matrix, aka grid
    let grid = advent_of_code::Grid::from_lines(&data);
    // find all '@' positions
    let mut remaining_positions: HashSet<(usize, usize)> = grid.find_char_positions('@');
    println!("Found <{}> '@'", remaining_positions.len());
    let mut accessible_count = 0;
    
    // Keep removing accessible positions until none are left accessible
    loop {
        // Find positions that are accessible (less than 4 surrounding '@')
        let accessible_positions: Vec<(usize, usize)> = remaining_positions.iter()
            .filter(|&&(row, col)| is_accessible(&grid, row, col, &remaining_positions))
            .cloned()
            .collect();
        
        let count = accessible_positions.len();
        println!("Found <{}> accessible '@'", accessible_count);
        
        // If no positions are accessible, we're done
        if count == 0 {
            break;
        }
        accessible_count += count;
        // Remove accessible positions from the remaining list
        remaining_positions.retain(|pos| !accessible_positions.contains(pos));
    }
    
    let final_count = accessible_count;
    println!("Remaining '@' count: {}", final_count);
    final_count as u32
}

fn main() {
    let d = advent_of_code::Reader::read_file("./input/day04_test.txt").unwrap();
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
