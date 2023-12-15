use crate::time_it;
use crate::utils::read_lines;
use colored::Colorize;
use indexmap::IndexMap;

#[derive(Debug, Default)]
struct Label<'a> {
    label: &'a str,
    hash: usize,
    focal_length: Option<u64>,
    op: Operation,
}
#[derive(Debug, Default)]
enum Operation {
    #[default]
    Equal,
    Sub,
}

impl<'a> From<&'a str> for Label<'a> {
    fn from(s: &'a str) -> Self {
        for (i, c) in s.chars().enumerate() {
            if c == '=' {
                return Self {
                    label: &s[..i],
                    hash: calculate_hash(&s[..i]) as usize,
                    focal_length: Some(s[(i + 1)..].parse::<u64>().unwrap()),
                    op: Operation::Equal,
                };
            }

            if c == '-' {
                return Self {
                    label: &s[..i],
                    hash: calculate_hash(&s[..i]) as usize,
                    focal_length: None,
                    op: Operation::Sub,
                };
            }
        }
        Self {
            ..Default::default()
        }
    }
}

fn calculate_hash(s: &str) -> u64 {
    let mut start: u64 = 0;

    for c in s.chars() {
        start = ((start + c as u64) * 17) % 256;
    }
    start
}

fn problem1(f: &str) -> u64 {
    let lines = read_lines(f);

    lines[0].split(',').map(calculate_hash).sum()
}

fn problem2(f: &str) -> usize {
    let lines = read_lines(f);
    let mut boxes: Vec<IndexMap<&str, u64>> = vec![IndexMap::new(); 256];
    let mut result = 0;

    for l in lines[0].split(',') {
        let label = Label::from(l);
        match label.op {
            Operation::Equal => {
                boxes[label.hash].insert(label.label, label.focal_length.unwrap());
            }
            Operation::Sub => {
                boxes[label.hash].shift_remove(&label.label);
            }
        }
    }

    for (i, b) in boxes.iter().enumerate() {
        for (j, (_, f)) in b.iter().enumerate() {
            result += (i + 1) * (j + 1) * *f as usize;
        }
    }
    result
}

pub fn solve() {
    // https://adventofcode.com/2023/day/15
    time_it!(15, 1, problem1("files/15.txt"));
    time_it!(15, 2, problem2("files/15.txt"));
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
