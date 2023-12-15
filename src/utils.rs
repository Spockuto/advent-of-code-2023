use colored::Colorize;
use std::fs::read_to_string;

pub(crate) fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

#[macro_export]
macro_rules! time_it {
    ($day:expr, $problem:expr, $s:expr) => {
        let timer = std::time::Instant::now();
        let soln = $s;
        //let day = $day.to_string().as_str();
        println!(
            "Solution for Day {:>2} problem {} is {:<14} Time: {:?}",
            stringify!($day),
            $problem,
            soln,
            timer.elapsed()
        );
    };
}
