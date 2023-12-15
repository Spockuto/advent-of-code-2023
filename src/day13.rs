use std::iter::zip;
use std::ops::AddAssign;

use crate::time_it;
use crate::utils::read_lines;

struct Reflection(usize, usize);

impl AddAssign for Reflection {
    fn add_assign(&mut self, other: Self) {
        *self = Self(self.0 + other.0, self.1 + other.1);
    }
}

fn reflection(nums: Vec<u64>, smudge: bool) -> usize {
    let mut mirror = 0;
    for i in 0..nums.len() {
        let mut temp = 0;
        let mut smudge_count = 1;
        let (v1, v2) = nums.split_at(i);
        for (a, b) in zip(v1.iter().rev(), v2) {
            temp += a ^ b;
            if (a ^ b).count_ones() == 1 {
                smudge_count -= 1
            }
        }
        if smudge {
            if temp.count_ones() == 1 && smudge_count == 0 {
                mirror = i;
            }
        } else if temp == 0 {
            mirror = i;
        }
    }
    mirror
}

fn find_mirror(mirror: Vec<String>, smudge: bool) -> Reflection {
    let mut rows: Vec<u64> = vec![];
    let mut columns: Vec<u64> = vec![0; mirror[0].len()];

    for (i, line) in mirror.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                if j == 0 {
                    rows.push(1);
                } else {
                    rows[i] = (rows[i] << 1) + 1;
                }
                columns[j] = (columns[j] << 1) + 1;
            } else {
                if j == 0 {
                    rows.push(0);
                } else {
                    rows[i] <<= 1;
                }
                columns[j] <<= 1;
            }
        }
    }

    Reflection(reflection(rows, smudge), reflection(columns, smudge))
}

fn problem1(f: &str, smudge: bool) -> usize {
    let lines = read_lines(f);
    let mut result = Reflection(0, 0);

    let mut start: usize = 0;
    for (i, line) in lines.iter().cloned().enumerate() {
        if line.is_empty() {
            let (data, _) = lines.split_at(i);
            result += find_mirror(data[start..].to_vec(), smudge);
            start = i + 1;
        }

        if i == lines.len() - 1 {
            result += find_mirror(lines[start..].to_vec(), smudge);
        }
    }
    100 * result.0 + result.1
}

pub fn solve() {
    // https://adventofcode.com/2023/day/13
    time_it!("Time", let soln = problem1("files/13.txt", false));
    println!("Solution for Day 13 problem 1 is {}", soln);
    time_it!("Time", let soln = problem1("files/13.txt", true));
    println!("Solution for Day 13 problem 2 is {}", soln);
}

mod tests {

    #[test]
    fn test_problem1() {
        assert_eq!(crate::day13::problem1("files/13_test.txt", false), 405);
    }

    #[test]
    fn test_problem2() {
        assert_eq!(crate::day13::problem1("files/13_test.txt", true), 400);
    }
}
