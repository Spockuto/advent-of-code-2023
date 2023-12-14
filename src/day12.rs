use std::iter::zip;
use std::vec;

use crate::time_it;
use crate::utils::read_lines;

fn combination_builder(s: &str) -> Vec<String> {
    let mut combinations: Vec<String> = vec![];
    for c in s.chars() {
        match c {
            '.' => {
                if combinations.is_empty() {
                    combinations.push(String::from('.'));
                } else {
                    combinations.iter_mut().map(|x| x.push('.')).collect()
                }
            }
            '#' => {
                if combinations.is_empty() {
                    combinations.push(String::from('#'));
                } else {
                    combinations.iter_mut().map(|x| x.push('#')).collect()
                }
            }
            '?' => {
                if combinations.is_empty() {
                    combinations.push(String::from('.'));
                    combinations.push(String::from('#'));
                } else {
                    let mut temp = combinations.clone();
                    let _ = combinations
                        .iter_mut()
                        .map(|x| x.push('.'))
                        .collect::<Vec<_>>();
                    let _ = temp.iter_mut().map(|x| x.push('#')).collect::<Vec<_>>();
                    combinations.extend(temp);
                }
            }
            _ => todo!(),
        }
    }

    combinations
}

// fn pmatch(a: &[u8], b: &[u8]) -> bool {
//     // 63 -> ?
//     // 46 -> .
//     // 35 -> #
//     for (x, y) in zip(a, b) {
//         if x != y && *x != 63 {
//             return false;
//         }
//     }
//     true
// }

// fn walk(spring: &str, possible: Vec<usize>) -> usize {
//     println!("{:?}", spring);
//     println!("{:?}", possible);

//     let mut stops: Vec<usize> = spring
//         .chars()
//         .enumerate()
//         .filter_map(|(i, x)| if x == '#' { Some(i) } else { None })
//         .collect();

//     let mut stop2: Vec<usize> = vec![];
//     let test : Vec<_> = spring.split_terminator(&['.', '?']).collect();

//     println!("{:?}", test);
//     println!("{:?}", spring.match_indices("#").collect::<Vec<_>>());
//     let mut loc: usize = 0;
//     for x in test.iter() {
//         if x.is_empty() {
//             loc += 1;
//         } else {
//             stop2.push(loc);
//             loc += x.len();
//         }
//     }

//     println!("{:?}", stops);
//     println!("{:?}", stop2);

//     let mut indexes: Vec<usize> = vec![0];
//     let mut stop_it = stop2.iter_mut();

//     for (i, num) in possible.iter().enumerate() {
//         // 3 -> ###.
//         let mut looking: Vec<u8> = vec![];
//         for _ in 0..*num {
//             looking.push(35);
//         }
//         let window_size;
//         if i != possible.len() - 1 {
//             window_size = *num + 1;
//             looking.push(46);
//         } else {
//             window_size = *num;
//         }

//         let mut temp: Vec<usize> = vec![];
//         for start in indexes.iter() {
//             // let mut it = stops.iter().skip_while(|x| *x <= start);
//             let end = match stop_it.next() {
//                 Some(i) => {
//                     // println!("{}", *i);
//                     if *i + window_size < spring.len() - 1 {
//                         *i + window_size
//                     } else {
//                         spring.len() - 1
//                     }
//                 }
//                 None => spring.len() - 1,
//             };
//             // let end = spring.len() - 1;
//             println!("{} {}", start, end);
//             for (i, slice) in spring[*start..=end]
//                 .as_bytes()
//                 .windows(window_size)
//                 .enumerate()
//             {
//                 println!(
//                     "{:?} {:?} {:?}",
//                     slice,
//                     looking.as_slice(),
//                     pmatch(slice, looking.as_slice())
//                 );
//                 if pmatch(slice, looking.as_slice()) {
//                     temp.push(*start + i + window_size)
//                 }
//             }
//         }
//         println!("temp: {:?}", temp);
//         indexes.clear();
//         indexes.extend(temp);
//     }

//     println!("final {:?}", indexes);
//     indexes.len()
// }

fn problem1(f: &str) -> usize {
    let lines = read_lines(f);
    let mut result: usize = 0;

    for line in lines {
        let split: Vec<_> = line.split(' ').collect();
        let spring = split[0];
        let possible: Vec<usize> = split[1]
            .split(',')
            .filter_map(|x| match x.parse::<usize>() {
                Ok(value) => Some(value),
                Err(_) => None,
            })
            .collect();
        result += combination_builder(spring)
            .iter()
            .map(|x| {
                x.split_terminator('.')
                    .filter(|f| !f.is_empty())
                    .map(|c| c.len())
                    .collect()
            })
            .filter(|c: &Vec<usize>| possible == **c)
            .count();
    }

    result
}

fn problem2(f: &str) -> usize {
    let lines = read_lines(f);
    let mut result: usize = 0;

    for line in lines {
        let split: Vec<_> = line.split(' ').collect();

        let possible: Vec<usize> = split[1]
            .split(',')
            .filter_map(|x| match x.parse::<usize>() {
                Ok(value) => Some(value),
                Err(_) => None,
            })
            .collect::<Vec<usize>>()
            .repeat(5);

        let cache = combination_builder(split[0]);

        let mut combinations: Vec<String> = cache.clone();
        for _ in 0..4 {
            let mut temp: Vec<String> = vec![];
            for c in combinations.iter() {
                for e in cache.iter() {
                    temp.push(c.to_owned() + "." + e);
                    temp.push(c.to_owned() + "#" + e);
                }
            }
            combinations.clear();
            combinations.extend(temp);
        }
        dbg!(combinations.clone().len());
        dbg!(possible.clone());
        result += combinations
            .iter()
            .map(|x| {
                x.split_terminator('.')
                    .filter(|f| !f.is_empty())
                    .map(|c| c.len())
                    .collect()
            })
            .filter(|c: &Vec<usize>| possible == **c)
            .count();
    }

    result
}

pub fn solve() {
    // https://adventofcode.com/2023/day/12
    // time_it!("Time", let soln = problem1("files/12.txt"));
    // println!("Solution for Day 12 problem 1 is {}", soln);
    // time_it!("Time", let soln = problem2("files/12_test.txt"));
    // println!("Solution for Day 12 problem 2 is {}", soln);
}

mod tests {

    // #[test]
    // fn test_problem1() {
    //     assert_eq!(crate::day12::problem1("files/12_test.txt"), 4361);
    // }

    // #[test]
    // fn test_problem2() {
    //     assert_eq!(crate::day12::problem2("files/12_test.txt"), 467835);
    // }
}
