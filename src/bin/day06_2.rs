#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

#[derive(Debug, Clone)]
enum Operation {
    Add,
    Multiply,
}

impl Operation {
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "+" => Ok(Operation::Add),
            "*" => Ok(Operation::Multiply),
            _ => Err(format!("Unknown operation: {}", s)),
        }
    }
}

// copilot generated those
impl Operation {
    fn apply<T>(&self, operand: &Vec<T>) -> T
    where
        T: std::ops::Add<Output = T> + std::ops::Mul<Output = T> + Copy + Default + From<u8>,
    {
        match self {
            Operation::Add => operand.iter().fold(T::default(), |acc, &x| acc + x),
            Operation::Multiply => operand.iter().fold(T::from(1u8), |acc, &x| acc * x),
        }
    }
}

#[derive(Debug, Clone)]
struct Instruction<T> {
    operation: Operation,
    operand: Vec<T>,
}

impl<T> Instruction<T>
where
    T: std::ops::Add<Output = T> + std::ops::Mul<Output = T> + Copy + Default + From<u8>,
{
    fn apply(&self) -> T {
        self.operation.apply(&self.operand)
    }
}

fn transpose_matrix<T: Copy>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    if matrix.is_empty() {
        return vec![];
    }
    let row_count = matrix.len();
    let col_count = matrix[0].len();
    let mut transposed: Vec<Vec<T>> = vec![vec![matrix[0][0]; row_count]; col_count];

    for r in 0..row_count {
        for c in 0..col_count {
            transposed[c][r] = matrix[r][c];
        }
    }
    transposed
}

fn char_vec_to_int(chars: &Vec<char>) -> Option<u64> {
    if chars.iter().all(|c| *c == ' ') {
        return None;
    }
    let value = chars
        .iter()
        .filter_map(|c| c.to_digit(10).map(|d| d as u64))
        .fold(0, |acc, d| acc * 10 + d);
    Some(value)
}

fn puzzle(data: &Vec<String>) -> u64 {
    let mut data = data.clone(); // we need to modify the data
    // last line contains operations, so lets pop it off and split it by whitespace
    let op = data.pop().unwrap();
    let op = op.split_whitespace().collect::<Vec<&str>>();

    // parse the remaining lines into u32 after splitting by whitespace
    let parsed_data = data
        .iter()
        .map(|part| part.chars().collect::<Vec<char>>())
        .inspect(|chars| println!("chars: {:?}", chars))
        .collect::<Vec<Vec<char>>>();

    // transpose the parsed_data for easier column-wise operations
    let transposed_data = transpose_matrix(&parsed_data);

    // working copy iterator
    let mut iter = transposed_data.clone().into_iter();
    // something to hold the numbers :)
    let mut numbers: Vec<Vec<u64>> = Vec::new();
    // take each column and convert to numbers until no more numbers are found
    // the `char_vec_to_int` will return None, which is the column of spaces only --> column separator
    loop {
        let n = iter
            .by_ref()
            .map_while(|chars| char_vec_to_int(&chars))
            .collect::<Vec<u64>>();
        if n.is_empty() {
            break;
        }
        numbers.push(n);
    }

    // Create instructions functionally, keep fingers crossed that both are of same length
    // if not, something is wrong with shaping of the input data
    let math: Vec<Instruction<u64>> = op
        .iter()
        .enumerate()
        .map(|(idx, op)| Instruction {
            operation: Operation::from_str(op).unwrap(),
            operand: numbers[idx].clone(),
        })
        .collect();
    math.iter().map(|instr| instr.apply()).sum::<u64>()
}

fn main() {
    let data = advent_of_code::Reader::read_file("./input/day06_test.txt").unwrap();
    let result = puzzle(&data);
    println!("result: {}", result);
}

#[cfg(test)]
mod tests {
    use crate::{advent_of_code, puzzle};

    #[test]
    fn char_vec_to_int_test_trailing_spaces() {
        let chars = vec!['1', '2', '3', ' '];
        let result = super::char_vec_to_int(&chars);
        assert_eq!(result, Some(123));
    }

    #[test]
    fn char_vec_to_int_test_leading_spaces() {
        let chars = vec![' ', '1', '2', '3'];
        let result = super::char_vec_to_int(&chars);
        assert_eq!(result, Some(123));
    }

    #[test]
    fn char_vec_to_int_test_all_spaces() {
        let chars = vec![' ', ' ', ' ', ' '];
        let result = super::char_vec_to_int(&chars);
        assert_eq!(result, None);
    }

    #[test]
    fn puzzle_test_data() {
        let d = advent_of_code::Reader::read_file("./input/day06_test.txt").unwrap();
        let result = puzzle(&d);
        println!("result: {result}");
        assert_eq!(result, 3263827);
    }

    #[test]
    fn puzzle_final_data() {
        let d = advent_of_code::Reader::read_file("./input/day06.txt").unwrap();
        let result = puzzle(&d);
        println!("result: {result}");
        assert_eq!(result, 11643736116335);
    }
}
