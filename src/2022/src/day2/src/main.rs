//! https://adventofcode.com/2022/day/2

#[cfg(feature = "speedtest")]
use std::time::Instant;

use aoc::Solution;

use self::RPSResult::*;

// A bit of bs to make it faster
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

struct Day2;
impl Solution<i32, i32> for Day2 {
    fn name() -> String {
        return "Rock Paper Scissors".to_string();
    }

    fn part1explan() -> String {
        "Total score when letters mean what to play".to_string()
    }

    fn part2explan() -> String {
        "Total score when letter means what outcome you need to get".to_string()
    }

    /// - rock: A, X, 1, Lose,
    /// - paper: B, Y, 2, Draw,
    /// - scisors: C, Z, 3, Win
    fn solve(input: String) -> (i32, i32) {
        let raw_strategy = input
            .split("\n")
            .into_iter()
            .map(|line| {
                line.trim_end()
                    .split(" ")
                    .map(|letter| // Turn each letter into String
                        letter.parse::<String>().unwrap())
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<String>>>();

        /// Shape score:
        ///
        /// - Rock: 1
        /// - Paper: 2
        /// - Scissors: 3
        ///
        /// Scores for outcome:
        ///
        /// - Win: 6
        /// - Tie: 3
        /// - Loss: 0
        fn get_score(strategy: &Vec<(RPSResult, i32)>) -> i32 {
            strategy
                .iter()
                .map(|game| match game.0 {
                    Win => 6 + game.1,
                    Tie => 3 + game.1,
                    Loss => 0 + game.1,
                })
                .sum::<i32>()
        }

        /// Raw strategy by choice of play.
        fn part1(raw_strategy: &Vec<Vec<String>>) -> i32 {
            get_score(
                // &raw_strategy_by_choice(raw_strategy)
                &raw_strategy
                    .iter()
                    .map(|interaction| match interaction[0].as_str() {
                        // Oppenent plays rock
                        "A" => match interaction[1].as_str() {
                            "Z" => (Loss, 3),
                            "X" => (Tie, 1),
                            "Y" => (Win, 2),
                            _ => unreachable!(),
                        },
                        // Opponent plays paper
                        "B" => match interaction[1].as_str() {
                            "X" => (Loss, 1),
                            "Y" => (Tie, 2),
                            "Z" => (Win, 3),
                            _ => unreachable!(),
                        },
                        // Opponent plays scissors
                        "C" => match interaction[1].as_str() {
                            "Y" => (Loss, 2),
                            "Z" => (Tie, 3),
                            "X" => (Win, 1),
                            _ => unreachable!(),
                        },

                        _ => unreachable!(),
                    })
                    .collect::<Vec<(RPSResult, i32)>>(),
            )
        }

        /// Raw strategy by intended outcome.
        fn part2(raw_strategy: &Vec<Vec<String>>) -> i32 {
            get_score(
                // &raw_strategy_by_outcome(raw_strategy)
                &raw_strategy
                    .iter()
                    .map(|interaction| match interaction[0].as_str() {
                        // Oppenent plays rock
                        "A" => match interaction[1].as_str() {
                            // You have to win
                            "Z" => (
                                Win, 2, // Paper
                            ),
                            // You have to lose
                            "X" => (
                                Loss, 3, // Scissors
                            ),
                            // You have to tie
                            "Y" => (
                                Tie, 1, // Rock
                            ),
                            _ => unreachable!(),
                        },
                        // Opponent plays paper
                        "B" => match interaction[1].as_str() {
                            "X" => (
                                Loss, 1, // Rock
                            ),
                            "Y" => (
                                Tie, 2, // Paper
                            ),
                            "Z" => (
                                Win, 3, // Scissors
                            ),
                            _ => unreachable!(),
                        },
                        // Opponent plays scissors
                        "C" => match interaction[1].as_str() {
                            "Y" => (
                                Tie, 3, // Scissors
                            ),
                            "Z" => (
                                Win, 1, // Rock
                            ),
                            "X" => (
                                Loss, 2, // Paper
                            ),
                            _ => unreachable!(),
                        },

                        _ => unreachable!(),
                    })
                    .collect::<Vec<(RPSResult, i32)>>(),
            )
        }

        // return (1234, 1234);
        return (part1(&raw_strategy), part2(&raw_strategy));
    }
}

/// Possible results of a game of Rock Paper Scissors.
#[derive(Debug, PartialEq)]
enum RPSResult {
    Win,
    Tie,
    Loss,
}

fn main() {
    #[cfg(feature = "speedtest")]
    let time = Instant::now();

    Day2::printsol(
        Day2::part1explan(),
        Day2::part2explan(),
        Day2::solve(include_str!("../input.txt").to_string()),
    );

    #[cfg(feature = "speedtest")]
    Day2::printspeed(&time.elapsed());
}
