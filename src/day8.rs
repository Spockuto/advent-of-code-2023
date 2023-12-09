use std::collections::HashMap;

use crate::time_it;
use crate::utils::read_lines;
use num_integer::Integer;

fn problem1(f: &str) -> u64 {
    let lines = read_lines(f);
    let mut result: u64 = 0;

    let instructions = lines.get(0).unwrap();

    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut key: &str = "AAA";

    for line in lines.iter().skip(2) {
        let split: Vec<&str> = line
            .split_terminator(['=', ' ', '(', ')', ','])
            .filter(|x| !x.is_empty())
            .collect();
        map.insert(split[0], (split[1], split[2]));
    }

    while key != "ZZZ" {
        let inst = instructions
            .chars()
            .nth(result as usize % instructions.len())
            .unwrap();

        key = match inst {
            'L' => map.get(key).unwrap().0,
            'R' => map.get(key).unwrap().1,
            _ => todo!(),
        };
        result += 1;
    }

    result
}

fn problem2(f: &str) -> u64 {
    let lines = read_lines(f);
    let instructions = lines.get(0).unwrap();

    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut keys: Vec<&str> = vec![];

    for line in lines.iter().skip(2) {
        let split: Vec<&str> = line
            .split_terminator(['=', ' ', '(', ')', ','])
            .filter(|x| !x.is_empty())
            .collect();
        map.insert(split[0], (split[1], split[2]));

        if split[0].ends_with('A') {
            keys.push(split[0]);
        }
    }

    let mut counts: Vec<u64> = vec![];

    for key in keys.iter_mut() {
        let mut count: u64 = 0;
        while !key.ends_with('Z') {
            let inst = instructions
                .chars()
                .nth(count as usize % instructions.len())
                .unwrap();
            *key = match inst {
                'L' => map.get(key).unwrap().0,
                'R' => map.get(key).unwrap().1,
                _ => todo!(),
            };
            count += 1;
        }
        counts.push(count);
    }
    counts.iter().fold(1, |acc, x| acc.lcm(x))
}

pub fn solve() {
    // https://adventofcode.com/2023/day/8
    time_it!("Time", let soln = problem1("files/8.txt"));
    println!("Solution for Day 8 problem 1 is {}", soln);
    time_it!("Time", let soln = problem2("files/8.txt"));
    println!("Solution for Day 8 problem 2 is {}", soln);
}

mod tests {

    #[test]
    fn test_problem1() {
        assert_eq!(crate::day8::problem1("files/8_test_1.txt"), 6);
    }

    #[test]
    fn test_problem2() {
        assert_eq!(crate::day8::problem2("files/8_test_2.txt"), 6);
    }
}
