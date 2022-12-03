//! Shared functions.

use std::fmt::Debug;

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
