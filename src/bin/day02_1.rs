#[path = "../advent_of_code/mod.rs"]
mod advent_of_code;

// The puzlle calls for two lists (given as two columns in a ascii file) to be sorted and line by line the absolute differences need to be summed up.
fn day2_1(data: &Vec<String>) -> u32 {
    42
}

fn main1() {
    let d = advent_of_code::Reader::read_file("./input/day2_1_test.txt").unwrap();
    let sum = day2_1(&d);
    println!("result: {sum}");
}

// check if number is made from two identical chunks, e.g., 1212, 3333, 4545, but not 1234 or 123123 or 12341234
fn is_invalid(s: &str) -> bool {
    let len = s.len();
    if len % 2 != 0 {
        return false; // Only even-length strings can be split into identical chunks
    }
    let half = len / 2;
    let first_half = &s[..half];
    let second_half = &s[half..];
    first_half == second_half
}

// fn has_identical_chunks(s: &str) -> bool {
//     let len = s.len();
//     if len % 2 != 0 {
//         return false; // Only even-length strings can be split into identical chunks
//     }

//     // Try all possible chunk sizes (divisors of length)
//     (1..=len/2).any(|chunk_size| {
//         if len % chunk_size == 0 {
//             let first_chunk = &s[..chunk_size];
//             s.as_bytes()
//                 .chunks(chunk_size)
//                 .all(|chunk| chunk == first_chunk.as_bytes())
//         } else {
//             false
//         }
//     })
// }

fn find_numbers_with_identical_chunks(lower: u64, upper: u64) -> Vec<u64> {
    (lower..=upper)
        .filter(|&n| is_invalid(&n.to_string()))
        .collect()
}

// thanks coPilot for the parsing function
fn parse_intervals(line: &str) -> Vec<(u64, u64)> {
    line.split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .filter_map(|interval| {
            let parts: Vec<&str> = interval.split('-').collect();
            if parts.len() == 2 {
                let lower = parts[0].parse::<u64>().ok()?;
                let upper = parts[1].parse::<u64>().ok()?;
                Some((lower, upper))
            } else {
                None
            }
        })
        .collect()
}

fn main() {
    // let line = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    let line = "67562556-67743658,62064792-62301480,4394592-4512674,3308-4582,69552998-69828126,9123-12332,1095-1358,23-48,294-400,3511416-3689352,1007333-1150296,2929221721-2929361280,309711-443410,2131524-2335082,81867-97148,9574291560-9574498524,648635477-648670391,1-18,5735-8423,58-72,538-812,698652479-698760276,727833-843820,15609927-15646018,1491-1766,53435-76187,196475-300384,852101-903928,73-97,1894-2622,58406664-58466933,6767640219-6767697605,523453-569572,7979723815-7979848548,149-216";

    let intervals = parse_intervals(line);
    let mut total_count = 0;
    let mut sum = 0u64;

    for (lower, upper) in intervals {
        let results = find_numbers_with_identical_chunks(lower, upper);
        println!("Interval {}-{}: found {:?} numbers", lower, upper, results);
        total_count += results.len();
        sum += results.iter().sum::<u64>();
    }

    println!("\nTotal numbers across all intervals: {}", total_count);
    println!("Sum of all numbers across all intervals: {}", sum);
}

#[cfg(test)]
mod tests {
    use crate::{advent_of_code, day2_1};

    fn is_valid(n: &u32) -> bool {
        true
    }

    #[test]
    fn invalid() {
        let n: u32 = 11;
    }

    #[test]
    fn day1_res() {
        let d = advent_of_code::Reader::read_file("./input/day2_1_test.txt").unwrap();
        let result = day2_1(&d);
        println!("result: {result}");
        assert_eq!(result, 2);
    }

    #[test]
    fn day1_final() {
        let d = advent_of_code::Reader::read_file("./input/day2_1.txt").unwrap();
        let result = day2_1(&d);
        println!("result: {result}");
        assert_eq!(result, 631);
    }
}
