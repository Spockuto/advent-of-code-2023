use crate::time_it;
use crate::utils::read_lines;
use core::panic;
use std::collections::HashMap;

fn problem1(f: &str) -> u64 {
    let lines = read_lines(f);

    let mut result: u64 = 0;
    for line in lines {
        let split: Vec<_> = line.split_terminator('|').collect();

        if split.len() > 2 {
            panic!("Formatting err with |");
        }
        let winning: Vec<u64> = split[0]
            .split_terminator(' ')
            .filter_map(|num| match num.parse::<u64>() {
                Ok(value) => Some(value),
                Err(_) => None,
            })
            .collect();
        let cards: Vec<u64> = split[1]
            .split_terminator(' ')
            .filter_map(|num| match num.parse::<u64>() {
                Ok(value) => Some(value),
                Err(_) => None,
            })
            .collect();

        let mut round: u64 = 0;
        for win in winning {
            if cards.contains(&win) {
                if round == 0 {
                    round = 1;
                } else {
                    round *= 2;
                }
            }
        }
        result += round;
    }
    result
}

fn problem2(f: &str) -> u64 {
    let lines = read_lines(f);

    let mut result: u64 = 0;
    let mut hash_set: HashMap<usize, u64> = HashMap::new();
    for i in 1..=lines.len() {
        hash_set.insert(i, 1);
    }

    for (i, line) in lines.iter().enumerate() {
        let split: Vec<_> = line.split_terminator('|').collect();

        if split.len() > 2 {
            panic!("Formatting err with |");
        }
        let winning: Vec<u64> = split[0]
            .split_terminator(' ')
            .filter_map(|num| match num.parse::<u64>() {
                Ok(value) => Some(value),
                Err(_) => None,
            })
            .collect();
        let cards: Vec<u64> = split[1]
            .split_terminator(' ')
            .filter_map(|num| match num.parse::<u64>() {
                Ok(value) => Some(value),
                Err(_) => None,
            })
            .collect();

        let mut count: u64 = 0;
        for card in cards {
            if winning.contains(&card) {
                count += 1;
            }
        }
        let current = i + 1;
        let end = (current + count as usize).min(lines.len());
        let copy = *hash_set.get(&current).unwrap_or(&0);
        for slots in current + 1..=end {
            *hash_set.get_mut(&slots).unwrap() += copy;
        }
    }

    for (_, value) in hash_set.into_iter() {
        result += value;
    }
    result
}

pub fn solve() {
    // https://adventofcode.com/2023/day/4
    time_it!(4, 1, problem1("files/4.txt"));
    time_it!(4, 2, problem2("files/4.txt"));
}

mod tests {

    #[test]
    fn test_problem1() {
        assert_eq!(crate::day4::problem1("files/4_test.txt"), 13);
    }

    #[test]
    fn test_problem2() {
        assert_eq!(crate::day4::problem2("files/4_test.txt"), 30);
    }
}
