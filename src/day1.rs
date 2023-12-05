use crate::time_it;
use crate::utils::read_lines;

fn problem1(f: &str) -> u64 {
    let lines = read_lines(f);
    let mut count: u64 = 0;
    const RADIX: u32 = 10;

    for line in lines {
        let value = 10
            * line
                .chars()
                .find(|&char| char.is_numeric())
                .unwrap()
                .to_digit(RADIX)
                .unwrap() as u64
            + line
                .chars()
                .rev()
                .find(|&char| char.is_numeric())
                .unwrap()
                .to_digit(RADIX)
                .unwrap() as u64;
        count += value;
    }
    count
}

fn problem2(f: &str) -> u64 {
    let lines = read_lines(f);
    let mut count: u64 = 0;
    const RADIX: u32 = 10;

    let string_map = vec![
        ("one", "o1ne"),
        ("two", "t2wo"),
        ("three", "th3ree"),
        ("four", "fo4ur"),
        ("five", "fi5ve"),
        ("six", "s6ix"),
        ("seven", "se7ven"),
        ("eight", "ei8ght"),
        ("nine", "ni9ne"),
    ];

    for line in lines {
        let mut t_line = line.clone();
        for each in string_map.iter() {
            t_line = t_line.replace(each.0, each.1);
        }

        let value = 10
            * t_line
                .chars()
                .find(|&char| char.is_numeric())
                .unwrap()
                .to_digit(RADIX)
                .unwrap() as u64
            + t_line
                .chars()
                .rev()
                .find(|&char| char.is_numeric())
                .unwrap()
                .to_digit(RADIX)
                .unwrap() as u64;
        count += value;
    }

    count
}

pub fn solve() {
    // https://adventofcode.com/2023/day/1
    time_it!("Time", let soln = problem1("files/1.txt"));
    println!("Solution for Day 1 problem 1 is {}", soln);
    time_it!("Time", let soln = problem2("files/1.txt"));
    println!("Solution for Day 1 problem 2 is {}", soln);
}

mod tests {

    #[test]
    fn test_problem1() {
        assert_eq!(crate::day1::problem1("files/1_test_1.txt"), 142);
    }

    #[test]
    fn test_problem2() {
        assert_eq!(crate::day1::problem2("files/1_test_2.txt"), 281);
    }
}
