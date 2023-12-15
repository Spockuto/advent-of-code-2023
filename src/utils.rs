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
        let time_limit = std::time::Duration::from_millis(5);
        let timer = std::time::Instant::now();
        let soln = $s;
        let elapsed = timer.elapsed();
        let time_count;

        if time_limit > elapsed {
            time_count = format!{"{:?}", elapsed}.color("green");
        } else {
            time_count = format!{"{:?}", elapsed}.color("red");
        }
        let ans = format!{"{:?}", soln}.italic().bold().color("green");
        println!(
            "│ Day: {:>02}::{} │ Soln: {:<14} │ Time: {:<12}│",
            stringify!($day).color("yellow"),
            stringify!($problem).color("blue"),
            ans,
            time_count
        );
        println!("├────────────┼──────────────────────┼───────────────────┤");
    };
}
