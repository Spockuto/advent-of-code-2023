use indexmap::IndexMap;

use crate::time_it;
use crate::utils::read_lines;

#[derive(Debug, Clone, Default)]
struct Label {
    label: String,
    hash: usize,
    focal_length: Option<u64>,
    op: Operation,
}
#[derive(Debug, Clone, Default)]
enum Operation {
    #[default]
    Equal,
    Sub,
}

impl From<&str> for Label {
    fn from(s: &str) -> Self {
        let mut ret: Label = Default::default();
        for (i, c) in s.chars().enumerate() {
            if c == '=' {
                ret = Label {
                    label: String::from(&s[..i]),
                    hash: calculate_hash(&s[..i]) as usize,
                    focal_length: Some(s[(i + 1)..].parse::<u64>().unwrap()),
                    op: Operation::Equal,
                };
                break;
            }

            if c == '-' {
                ret = Label {
                    label: String::from(&s[..i]),
                    hash: calculate_hash(&s[..i]) as usize,
                    focal_length: None,
                    op: Operation::Sub,
                };
                break;
            }
        }
        ret
    }
}

fn calculate_hash(s: &str) -> u64 {
    let mut start: u64 = 0;

    for c in s.chars() {
        start = (((start + c as u64) << 4) + (start + c as u64)) & 255;
    }
    start
}

fn problem1(f: &str) -> u64 {
    let lines = read_lines(f);

    lines[0].split(',').map(calculate_hash).sum()
}

fn problem2(f: &str) -> usize {
    let lines = read_lines(f);
    let mut boxes: Vec<IndexMap<String, u64>> = vec![IndexMap::new(); 256];
    let mut result = 0;

    for x in lines[0].split(',') {
        let label = Label::from(x);
        match label.op {
            Operation::Equal => {
                if boxes[label.hash].contains_key(&label.label) {
                    boxes[label.hash][&label.label] = label.focal_length.unwrap();
                } else {
                    boxes[label.hash].insert(label.label, label.focal_length.unwrap());
                }
            }
            Operation::Sub => {
                if boxes[label.hash].contains_key(&label.label) {
                    boxes[label.hash].shift_remove(&label.label);
                }
            }
        }
    }

    for (i, b) in boxes.iter().enumerate() {
        if !b.is_empty() {
            for (j, (_, f)) in b.iter().enumerate() {
                result += (i + 1) * (j + 1) * *f as usize;
            }
        }
    }
    result
}

pub fn solve() {
    // https://adventofcode.com/2023/day/15
    time_it!("Time", let soln = problem1("files/15.txt"));
    println!("Solution for Day 15 problem 1 is {}", soln);
    time_it!("Time", let soln = problem2("files/15.txt"));
    println!("Solution for Day 15 problem 2 is {}", soln);
}

mod tests {

    #[test]
    fn test_problem1() {
        assert_eq!(crate::day15::problem1("files/15_test.txt"), 1320);
    }

    #[test]
    fn test_problem2() {
        assert_eq!(crate::day15::problem2("files/15_test.txt"), 145);
    }
}
