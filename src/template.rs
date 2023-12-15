use crate::utils::read_lines;
use crate::time_it;

fn problem1(f: &str) -> u64 {
    let lines = read_lines(f);
    let mut result: u64 = 0;
    
    result
}

fn problem2(f: &str) -> u64 {
    let lines = read_lines(f);
    let mut result: u64 = 0;
    
    result
}

pub fn solve() {
    // https://adventofcode.com/2023/day/3
    time_it!(3, 1, problem1("files/3.txt"));
    time_it!(3, 2, problem2("files/3.txt"));
}

mod tests {

    #[test]
    fn test_problem1() {
        assert_eq!(crate::day3::problem1("files/3_test.txt"), 4361);
    }

    #[test]
    fn test_problem2() {
        assert_eq!(crate::day3::problem2("files/3_test.txt"), 467835);
    }
}