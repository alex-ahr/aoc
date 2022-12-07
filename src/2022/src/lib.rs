//! Shared functions.

use std::env::current_exe;
use std::fmt::Debug;

#[cfg(feature = "speedtest")]
use std::time::{Duration, Instant};

pub trait Solution<ONE, TWO> {
    /// Returns the name of the day's puzzle.
    fn name() -> String;

    /// Explanation for Part 1.
    /// Printed when calling
    /// `printsol()`.
    fn part1explan() -> String;
    /// Explanation for part 2.
    /// Printed when calling
    /// `printsol()`.
    fn part2explan() -> String;
    fn solve(input: String) -> (ONE, TWO);

    fn printsol(part1txt: String, part2txt: String, solution: (ONE, TWO))
    where
        ONE: Debug,
        TWO: Debug,
    {
        println!("#part1: {part1txt}\n{:?}", solution.0);
        println!("#part2: {part2txt}\n{:?}", solution.1);
    }

    #[cfg(feature = "speedtest")]
    fn printspeed(time: &std::time::Duration) {
        println!("#time:\n{:?}", time);
    }
}

/// Prints day from name of program
/// plus the name of the day's puzzle.
pub fn printday(name: &str) {
    println!(
        "{} - {name} ///\n",
        trim_path(current_exe().unwrap().to_str().unwrap())
    );
}

/// Print each part, formatted correctly.
pub fn printpart(partnum: i32, help: &str, answer: String) {
    println!("#part{partnum}: {help}\n{answer}");
}

/// Prints how quickly each program executed.
#[cfg(feature = "speedtest")]
fn printtime(time: &Duration) {
    println!("#time:\n{:?}", time);
}

/// Trims up to the last entry in path.
///
/// ex: `target/release/day3` to `day3`
fn trim_path(path: &str) -> &str {
    let len = path.split("/").collect::<Vec<&str>>().len();
    path.split("/").collect::<Vec<&str>>()[len - 1]
}

/// Stops the timer a
#[cfg(feature = "speedtest")]
pub fn printtotaltime(time: Instant) {
    let time = time.elapsed();
    printtime(&time);
}
