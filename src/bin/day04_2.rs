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

// Returns Some(neighbors) if accessible, None if not accessible
fn is_accessible(grid: &advent_of_code::Grid, row: usize, col: usize, remaining_positions: &HashSet<(usize, usize)>) -> Option<Vec<(usize, usize)>> {
    let surrounding_positions = grid.get_surrounding_positions(row, col);
    let at_count = surrounding_positions.iter()
        .filter(|pos| remaining_positions.contains(pos))
        .count();
    
    if at_count < 4 {
        Some(surrounding_positions)
    } else {
        None
    }
}

fn puzzle(data: &Vec<String>) -> u32 {
    let start_time = std::time::Instant::now();
    // lines_to_matrix, aka grid
    let grid = advent_of_code::Grid::from_lines(&data);
    // find all '@' positions
    let mut remaining_positions: HashSet<(usize, usize)> = grid.find_char_positions('@');
    let mut changed_positions: HashSet<(usize, usize)> = remaining_positions.clone();
    println!("Found <{}> '@'", remaining_positions.len());
    let mut accessible_count = 0;
    
    // Keep removing accessible positions until none are left accessible
    loop {
        // Functional approach: filter_map to get accessible positions and their neighbors
        let accessible_with_neighbors: Vec<((usize, usize), Vec<(usize, usize)>)> = changed_positions.iter()
            .filter_map(|&(row, col)| {
                if remaining_positions.contains(&(row, col)) {
                    is_accessible(&grid, row, col, &remaining_positions)
                        .map(|neighbors| ((row, col), neighbors))
                } else {
                    None
                }
            })
            .collect();

        let count = accessible_with_neighbors.len();
        println!("Found <{}> accessible '@'", count);
        
        // If no positions are accessible, we're done
        if count == 0 {
            break;
        }
        
        accessible_count += count;
        
        // Collect next changed positions from all neighbors
        let mut next_changed_positions = HashSet::new();
        
        // Remove accessible positions and collect neighbors for next iteration
        for ((row, col), neighbors) in accessible_with_neighbors {
            remaining_positions.remove(&(row, col));
            
            // Add existing neighbors to next iteration's check list
            for neighbor in neighbors {
                if remaining_positions.contains(&neighbor) {
                    next_changed_positions.insert(neighbor);
                }
            }
        }
        
        changed_positions = next_changed_positions;
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
