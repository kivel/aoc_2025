use std::collections::BTreeSet;

#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn new(x: u32, y: u32) -> Self {
        Point { x, y }
    }

    fn from_str(s: &str) -> Self {
        let parts: Vec<u32> = s
            .split(',')
            .map(|part| part.trim().parse::<u32>().unwrap())
            .collect();
        Point::new(parts[0], parts[1])
    }

    // area of rectangle defined by self and other point
    fn area(&self, other: &Point) -> u64 {
        let width = (other.x as i32 - self.x as i32).abs() as u64 + 1;
        let height = (other.y as i32 - self.y as i32).abs() as u64 + 1;
        width * height
    }
}

// brute force all possible pairs and scram them into a BTreeMap, which is sorted by distance
fn area_map(points: &Vec<Point>) -> BTreeSet<u64> {
    let mut rect_map: BTreeSet<u64> = BTreeSet::new();

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let area = points[i].area(&points[j]);
            rect_map.insert(area);
        }
    }
    println!("......... created a map of {} pairs", rect_map.len());
    rect_map
}

fn puzzle(data: &Vec<String>) -> usize {
    let vertices: Vec<Point> = data.iter().map(|line| Point::from_str(line)).collect();
    println!("found {} points", data.len());

    let rect_map = area_map(&vertices);

    rect_map.last().unwrap().clone() as usize
}

fn main() {
    let data = advent_of_code::Reader::read_file("./input/day09_test.txt").unwrap();
    let result = puzzle(&data);
    println!("result: {}", result);
}

#[cfg(test)]
mod tests {
    use crate::{advent_of_code, puzzle};

    #[test]
    fn puzzle_test_data() {
        let d = advent_of_code::Reader::read_file("./input/day09_test.txt").unwrap();
        let result = puzzle(&d);
        println!("result: {result}");
        assert_eq!(result, 50);
    }

    #[test]
    fn puzzle_final_data() {
        let d = advent_of_code::Reader::read_file("./input/day09.txt").unwrap();
        let result = puzzle(&d);
        println!("result: {result}");
        assert_eq!(result, 4760959496);
    }
}
