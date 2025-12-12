use std::collections::BTreeMap;

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
}

enum PointClass {
    Inside,
    Edge,
    Outside,
    Vertex,
}

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
struct Rectangle {
    p1: Point,
    p2: Point,
    vertices: Vec<Point>,
}

impl Rectangle {
    fn new(p1: Point, p2: Point) -> Self {
        let x_min = p1.x.min(p2.x);
        let x_max = p1.x.max(p2.x);
        let y_min = p1.y.min(p2.y);
        let y_max = p1.y.max(p2.y);
        let vertices = vec![
            Point::new(x_min, y_min),
            Point::new(x_min, y_max),
            Point::new(x_max, y_min),
            Point::new(x_max, y_max),
        ];
        Rectangle { p1, p2, vertices }
    }

    // area of rectangle defined by self.p1 and self.p2
    fn area(&self) -> u64 {
        let width = (self.p1.x as i64 - self.p2.x as i64).abs() as u64 + 1;
        let height = (self.p1.y as i64 - self.p2.y as i64).abs() as u64 + 1;
        width * height
    }

    fn point_in_rect(&self, p: &Point) -> PointClass {
        let x_min = self.p1.x.min(self.p2.x);
        let x_max = self.p1.x.max(self.p2.x);
        let y_min = self.p1.y.min(self.p2.y);
        let y_max = self.p1.y.max(self.p2.y);
        // has to be strictly inside, not on the edge
        match p {
            _ if self.vertices.contains(p) => PointClass::Vertex,
            _ if (p.x == x_min || p.x == x_max) && (p.y > y_min && p.y < y_max) => PointClass::Edge,
            _ if (p.y == y_min || p.y == y_max) && (p.x > x_min && p.x < x_max) => PointClass::Edge,
            _ if p.x > x_min && p.x < x_max && p.y > y_min && p.y < y_max => PointClass::Inside,
            _ => PointClass::Outside,
        }
    }
}

// brute force all possible pairs and scram them into a BTreeMap, which is sorted by distance
fn area_map(points: &Vec<Point>) -> BTreeMap<u64, Rectangle> {
    let mut rect_map: BTreeMap<u64, Rectangle> = BTreeMap::new();

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let rect = Rectangle::new(points[i].clone(), points[j].clone());
            let mut area = rect.area() * 1000;
            while let Some(_) = rect_map.get(&area) {
                // collision, increment area slightly
                // NOTE: this is a hack to avoid dealing with duplicate areas
                area += 1;
            }
            match rect_map.insert(area, rect) {
                Some(_) => panic!("duplicate area found: {}", area),
                None => {}
            }
        }
    }
    println!("......... created a map of {} pairs", rect_map.len());
    rect_map
}

fn has_dividing_line(points: &Vec<Point>) -> bool {
    for p1 in points.iter() {
        for p2 in points.iter() {
            if p1 == p2 {
                continue;
            }
            if (p1.x == p2.x) || (p1.y == p2.y) {
                return true;
            }
        }
    }
    false
}

fn is_valid_rectangle(r: &Rectangle, points: &Vec<Point>) -> bool {
    let mut on_border_points: Vec<Point> = Vec::new();
    for p in points.iter() {
        let classification = r.point_in_rect(p);
        match classification {
            PointClass::Inside => {
                // println!("  contains a point {:?}, skipping", p);
                return false;
            }
            PointClass::Edge => {
                on_border_points.push(p.clone());
            }
            PointClass::Vertex => {}
            PointClass::Outside => {}
        }
    }
    if on_border_points.len() >= 2 {
        return !has_dividing_line(&on_border_points);
    }
    true
}

fn puzzle(data: &Vec<String>) -> usize {
    let vertices: Vec<Point> = data.iter().map(|line| Point::from_str(line)).collect();
    println!("found {} points", data.len());

    let rect_map = area_map(&vertices);

    let ok_rect = rect_map
        .iter()
        .rev()
        .skip_while(|(_, r)| !is_valid_rectangle(r, &vertices))
        .next();
    if let Some((area, r)) = ok_rect {
        println!(
            "✓  found rectangle with area {:?} and points {:?} and {:?} that contains no other points",
            area, r.p1, r.p2
        );
        return r.area() as usize;
    }
    0
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
        assert_eq!(result, 24);
    }

    #[test]
    fn puzzle_final_data() {
        let d = advent_of_code::Reader::read_file("./input/day09.txt").unwrap();
        let result = puzzle(&d);
        println!("result: {result}");
        assert_eq!(result, 4760959496);
    }
}
