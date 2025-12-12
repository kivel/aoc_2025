use std::fmt;

use nalgebra::{DMatrix, DVector};
#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Machine {
    led: u32,
    buttons: DMatrix<usize>,
    joltage: DVector<u32>,
}

impl fmt::Display for Machine {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{:?} | {:?}", self.buttons, self.joltage)
    }
}

fn fill_zeros(vec: &Vec<usize>, size: usize) -> Vec<usize> {
    let mut ret_vec = vec![0; size];
    vec.iter().for_each(|i| ret_vec[*i] = 1);
    ret_vec
}

impl Machine {
    fn new(led: u32, buttons: DMatrix<usize>, joltage: DVector<u32>) -> Self {
        Machine {
            led,
            buttons,
            joltage,
        }
    }

    // example str: "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"
    fn from_str(s: &str) -> Self {
        let mut parts: Vec<&str> = s.split(' ').collect();
        let led = 0u32;
        // see part-1 for led parsing

        let joltage_str = parts.pop().unwrap().trim_matches(&['{', '}'][..]);
        let j_vec: Vec<u32> = joltage_str
            .split(',')
            .filter_map(|part| part.trim().parse::<u32>().ok())
            .collect();
        let joltage = DVector::from_vec(j_vec.clone());

        let b_vec: Vec<Vec<usize>> = parts
            .iter()
            .skip(1)
            .map(|s| s.trim_matches(&['(', ')'][..]))
            .map(|bs| {
                bs.split(',')
                    .filter_map(|part| part.trim().parse::<usize>().ok())
                    .collect::<Vec<usize>>()
            })
            .map(|v| fill_zeros(&v, joltage.len()))
            .collect();
        let buttons: DMatrix<usize> = DMatrix::from_vec(joltage.len(), b_vec.len(), b_vec.concat());
        Machine::new(led, buttons, joltage)
    }
}

// part to is a linear algebra problem, where each button press represents a "1" in a matrix row,
// the joltage represents the target vector (RHS), we need to solve for the vector of button presses (LHS)
fn solve(machine: &Machine) -> Option<usize> {
    // Convert usize matrix to f64 matrix for linear algebra operations
    let buttons_f64 = machine.buttons.map(|x| x as f64);
    let joltage_f64 = machine.joltage.map(|x| x as f64);

    // let svd = buttons_f64.svd(true, true);
    // let r = svd.solve(&joltage_f64, 1e-10);

    // println!("SVD solution for machine with led {}: {:?}", machine.led, r);

    let qr = buttons_f64.qr();
    let r_qr = qr.solve(&joltage_f64);
    println!(
        "QR solution for machine with led {}: {:?}",
        machine.led, r_qr
    );

    Some(0)

    // let lu = buttons_f64.lu();
    // lu.solve(&joltage_f64).map(|_| 0) // Return Some(0) as placeholder
}

// fn solve_system(matrix: DMatrix<f64>, target: DVector<f64>) -> Option<DVector<f64>> {
//     // Use LU decomposition to solve
//     let lu = matrix.lu();
//     lu.solve(&target)
// }

fn puzzle(data: &Vec<String>) -> usize {
    let machines: Vec<Machine> = data.iter().map(|line| Machine::from_str(line)).collect();
    machines.iter().map(|m| solve(m).unwrap_or_default()).sum()
}

fn main() {
    let data = advent_of_code::Reader::read_file("./input/day10_test.txt").unwrap();

    println!("parsed {} machines", data.len());
    print!("machines:\n");
    let machines: Vec<Machine> = data.iter().map(|line| Machine::from_str(line)).collect();
    machines.iter().for_each(|m| println!("  {}", m));

    let m = &machines[0];
    println!("solving machine:\n  {}", m);
    let result = solve(m).unwrap_or_default();
    println!("result: {}", result);
    // let result = puzzle(&data);
    // println!("result: {}", result);
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
