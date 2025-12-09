use std::{collections::BTreeMap, vec};

#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
struct Circuit {
    boxes: Vec<JunctionBox>,
}

impl Circuit {
    fn new(boxes: Vec<JunctionBox>) -> Self {
        Circuit { boxes }
    }

    fn from_pair((jb1, jb2): (&JunctionBox, &JunctionBox)) -> Self {
        Self::new(vec![*jb1, *jb2])
    }

    fn contains_box(&self, jb: &JunctionBox) -> bool {
        self.boxes.contains(jb)
    }

    fn contains_pair(&self, (jb1, jb2): (&JunctionBox, &JunctionBox)) -> bool {
        self.contains_box(jb1) || self.contains_box(jb2)
    }

    fn insert_pair(&mut self, (jb1, jb2): (&JunctionBox, &JunctionBox)) {
        if !self.contains_box(jb1) {
            self.boxes.push(*jb1);
        }
        if !self.contains_box(jb2) {
            self.boxes.push(*jb2);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
struct JunctionBox {
    x: u32,
    y: u32,
    z: u32,
}

// only compare x for ordering
impl Ord for JunctionBox {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.x.cmp(&other.x)
    }
}

impl JunctionBox {
    fn new(x: u32, y: u32, z: u32) -> Self {
        JunctionBox { x, y, z }
    }

    fn from_str(s: &str) -> Self {
        let parts: Vec<u32> = s
            .split(',')
            .map(|part| part.trim().parse::<u32>().unwrap())
            .collect();
        JunctionBox::new(parts[0], parts[1], parts[2])
    }

    fn distance(&self, other: &JunctionBox) -> u64 {
        (((self.x as f64 - other.x as f64).powi(2)
            + (self.y as f64 - other.y as f64).powi(2)
            + (self.z as f64 - other.z as f64).powi(2))
        .sqrt()
            * 1000000.0) as u64
    }
}

// brute force all possible pairs and scram them into a BTreeMap, which is sorted by distance
fn closest_pair_map(boxes: &Vec<JunctionBox>) -> BTreeMap<u64, (&JunctionBox, &JunctionBox)> {
    let mut box_map: BTreeMap<u64, (&JunctionBox, &JunctionBox)> = BTreeMap::new();

    for i in 0..boxes.len() {
        for j in (i + 1)..boxes.len() {
            let dist = boxes[i].distance(&boxes[j]);
            if box_map.contains_key(&dist) {
                panic!("duplicate distance found: {}", dist);
            }
            box_map.insert(dist, (&boxes[i], &boxes[j]));
        }
    }
    println!("......... created a map of {} pairs", box_map.len());
    box_map
}

fn puzzle(data: &Vec<String>, take_n: usize) -> usize {
    let boxes: Vec<JunctionBox> = data
        .iter()
        .map(|line| JunctionBox::from_str(line))
        .collect();
    // boxes.sort();
    // boxes.iter().for_each(|b| println!("{:?}", b.show()));
    println!("found {} junction boxes", data.len());

    let box_map = closest_pair_map(&boxes);

    let mut circuits: BTreeMap<u64, Circuit> = BTreeMap::new();
    // let (d, first_pair) = box_map.iter().next().unwrap();
    // let c = Circuit::from_pair(*first_pair);
    // circuits.insert(*d, c);

    for b in box_map.iter().take(take_n + 1) {
        let pair = b.1;
        let mut found_match = false;
        // println!("\n ==> Processing pair: {:?} with distance {}", pair, b.0);
        for c in circuits.iter_mut() {
            // println!("checking against circuit with boxes {:?}", c.boxes);
            if c.1.contains_pair(*pair) {
                c.1.insert_pair(*pair);
                found_match = true;
                // println!(
                //     "+++> pair {:?} matches circuit with boxes {:?}",
                //     pair, c.1.boxes
                // );
                break;
            }
        }

        if !found_match {
            let c = Circuit::from_pair(*pair);
            circuits.insert(*b.0, c);
            // println!("created new circuit from pair: {:?}", pair);
        }

        // circuits.extend(new_circuits);
    }

    println!("\n\n----------------- Final Circuits -----------------\n\n");
    // circuits.sort();
    // circuits.reverse();
    let mut circuit_len: Vec<usize> = circuits.iter().map(|c| c.1.boxes.len()).collect();
    circuit_len.sort();
    circuit_len.reverse();
    circuit_len
        .iter()
        .take(3)
        .inspect(|x| println!("took: {}", x))
        .fold(1 as usize, |acc, &x| acc * x)
    // circuits.sort();
    // println!("final circuits: {:?}", circuits);
    // 0
}

fn main() {
    let data = advent_of_code::Reader::read_file("./input/day08_test.txt").unwrap();
    let result = puzzle(&data, 10);
    println!("result: {}", result);
}

#[cfg(test)]
mod tests {
    use crate::{advent_of_code, puzzle};

    #[test]
    fn puzzle_test_data() {
        let d = advent_of_code::Reader::read_file("./input/day08_test.txt").unwrap();
        let result = puzzle(&d, 10);
        println!("result: {result}");
        assert_eq!(result, 40);
    }

    #[test]
    fn puzzle_final_data() {
        let d = advent_of_code::Reader::read_file("./input/day08.txt").unwrap();
        let result = puzzle(&d, 1000);
        println!("result: {result}");
        assert_eq!(result, 0);
    }
}
