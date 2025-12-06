#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

fn is_accessible(grid: &advent_of_code::Grid, row: usize, col: usize) -> bool {
    let surrounding_chars = grid.get_surrounding_chars(row, col);
    let at_count = surrounding_chars.iter().filter(|&&c| c == '@').count();
    at_count < 4
}
// The puzlle calls for two lists (given as two columns in a ascii file) to be sorted and line by line the absolute differences need to be summed up.
fn puzzle(data: &Vec<String>) -> u32 {
    // lines_to_matrix, aka grid
    let grid = advent_of_code::Grid::from_lines(&data);
    // find all '@' positions
    let at_positions = grid.find_char_positions('@');
    println!("Found <{}> '@'", at_positions.len());
    // filter positions directly
    let accessible_count = at_positions
        .iter()
        .filter(|&&(row, col)| is_accessible(&grid, row, col))
        .count();

    println!("Accessible '@' count: {}", accessible_count);
    accessible_count as u32
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
        assert_eq!(result, 13);
    }

    #[test]
    fn puzzle_final_data() {
        let d = advent_of_code::Reader::read_file("./input/day04.txt").unwrap();
        let result = puzzle(&d);
        println!("result: {result}");
        assert_eq!(result, 1527);
    }
}
