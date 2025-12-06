#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

fn day01_1(data: &Vec<String>) -> u32 {
    let mut dial_position: i32 = 50;
    let mut counter: u32 = 0;
    data.iter().for_each(|l| {
        let distance: u32 = l[1..].parse::<u32>().unwrap();
        if l.starts_with("L") {
            dial_position -= distance as i32;
        } else {
            dial_position += distance as i32;
        }
        if (dial_position % 100) == 0 {
            counter += 1;
        }
    });
    counter
}

fn main() {
    let d = advent_of_code::Reader::read_file("./input/day01_test.txt").unwrap();
    let sum = day01_1(&d);
    println!("result: {sum}");
}

#[cfg(test)]
mod tests {
    use crate::{advent_of_code, day01_1};

    #[test]
    fn day01_res() {
        let d = advent_of_code::Reader::read_file("./input/day01_test.txt").unwrap();
        let result = day01_1(&d);
        println!("result: {result}");
        assert_eq!(result, 3);
    }

    #[test]
    fn day01_final() {
        let d = advent_of_code::Reader::read_file("./input/day01.txt").unwrap();
        let result = day01_1(&d);
        println!("result: {result}");
        assert_eq!(result, 1180);
    }
}
