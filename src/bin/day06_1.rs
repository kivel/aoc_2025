use std::{collections::HashSet, iter::Enumerate};

#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

#[derive(Debug, Clone)]
enum Operation {
    Add,
    Multiply,
}

#[derive(Debug, Clone)]
struct Instruction<T> {
    operation: Operation,
    operand: Vec<T>,
}

impl<T> Instruction<T>
where
    T: std::ops::Add<Output = T> + std::ops::Mul<Output = T> + Copy + Default,
{
    fn apply(&self) -> T {
        match self.operation {
            Operation::Add => self.operand.iter().fold(T::default(), |acc, &x| acc + x),
            Operation::Multiply => self
                .operand
                .iter()
                .skip(1)
                .fold(self.operand[0], |acc, &x| acc * x),
        }
    }
}

// The puzlle calls for two lists (given as two columns in a ascii file) to be sorted and line by line the absolute differences need to be summed up.
fn puzzle(data: &Vec<String>) -> usize {
    0
}

fn main() {
    let mut data = advent_of_code::Reader::read_file("./input/day06.txt").unwrap();
    println!("data lines: {}", data.len());
    let op = data.pop().unwrap();
    let op = op.split_whitespace().collect::<Vec<&str>>();
    println!("Last line: {:?}", &op);
    println!("data lines: {}", data.len());
    // parse the remaining lines into u32 after splitting by whitespace
    let parsed_data = data
        .iter()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|part| part.parse::<u64>().ok())
                .collect::<Vec<u64>>() // Collect each line's numbers into Vec<u64>
        })
        .collect::<Vec<Vec<u64>>>();

    // transpose the parsed_data
    let transposed_data: Vec<Vec<u64>> = {
        let row_count = parsed_data.len();
        let col_count = parsed_data[0].len();
        (0..col_count)
            .map(|col_idx| {
                (0..row_count)
                    .map(|row_idx| parsed_data[row_idx][col_idx])
                    .collect::<Vec<u64>>()
            })
            .collect()
    };

    println!("Parsed data: {:?}", parsed_data);
    println!("Transposed data: {:?}", transposed_data);
    let mut math: Vec<Instruction<u64>> = vec![];
    for (idx, op) in op.iter().enumerate() {
        match *op {
            "+" => {
                math.push(Instruction {
                    operation: Operation::Add,
                    operand: transposed_data[idx].clone(),
                });
            }
            "*" => {
                math.push(Instruction {
                    operation: Operation::Multiply,
                    operand: transposed_data[idx].clone(),
                });
            }
            _ => panic!("Unknown operation: {}", op),
        }
    }
    println!("Instructions: {:?}", math);
    // let result: u32 = math.iter()
    //     .map(|instr| instr.apply())
    //     .collect::<Vec<u32>>()
    //     .iter()
    //     .sum::<u32>();
    let result: u64 = math.iter().map(|instr| instr.apply()).sum();
    println!("result: {}", result);
}

#[cfg(test)]
mod tests {
    use crate::{advent_of_code, puzzle};

    #[test]
    fn puzzle_test_data() {
        let d = advent_of_code::Reader::read_file("./input/day05_test.txt").unwrap();
        let result = puzzle(&d);
        println!("result: {result}");
        assert_eq!(result, 3);
    }

    #[test]
    fn puzzle_final_data() {
        let d = advent_of_code::Reader::read_file("./input/day05.txt").unwrap();
        let result = puzzle(&d);
        println!("result: {result}");
        assert_eq!(result, 563);
    }
}
