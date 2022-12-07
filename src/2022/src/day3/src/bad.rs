//! I wrote this after almost completely
//! misunderstanding the problem.
//! 
//! https://adventofcode.com/2022/day/3

#![feature(iter_intersperse)]

#[cfg(feature = "speedtest")]
use std::time::Instant;

use itertools::{self, Itertools};

use aoc::{Solution, printpart, printday, printtotaltime};

fn solve(input: &str) -> (i32,) {
    // List of rucksacks and their contents.
    let rucksack = input.split("\n")
        .map(|row|
            row.chars().collect::<Vec<char>>()
        ).collect::<Vec<Vec<char>>>()
    ;

    println!("rucksack: {:?}", &rucksack);

    let rucksack_compartments = rucksack_to_compartments(&rucksack);
    
    println!("rucksack_compartments: {:?}", &rucksack_compartments);

    println!("rucksack_to_values(&rucksack): {:?}", rucksack_to_values(&rucksack));

    println!(
        "rucksack_compartments_to_values(rucksack_compartments.clone()): {:?}",
        rucksack_compartments_to_values(rucksack_compartments.clone())
    );

    println!(
        "rucksack_compartments_values_common_values(&rucksack_compartments_to_values(rucksack_compartments.clone())): {:?}",
        rucksack_compartments_values_common_values(&rucksack_compartments_to_values(rucksack_compartments.clone()))
    );

    /// Sum the single item that exists in both parts
    /// of the tuple, and then sum each tuple.
    fn part1<'a>(
        rucksack_compartments_common_values: &Vec<(Vec<i32>, Vec<i32>)>
    ) -> i32 {
        rucksack_compartments_values_common_values(
            rucksack_compartments_common_values
        ).iter().sum()
    }

    return (part1(&rucksack_compartments_to_values(rucksack_compartments)),);
}

fn main() {

    dbg!(
        rucksack_to_values(&include_str!("./test.txt").split("\n")
        .map(|row|
            row.chars().collect::<Vec<char>>()
        ).collect::<Vec<Vec<char>>>())
    );

    let solution = solve(include_str!("./test.txt"));
    println!("{:?}", &solution);
    assert_eq!(solution.0, 157);

    // printday("Rucksack Reorganization");

    // #[cfg(feature = "speedtest")]
    // let time = Instant::now();

    // let solution = solve(include_str!("../input.txt"));
    // printpart(
    //     1,
    //     "Sum of priorities of item types",
    //     solution.0.to_string()
    // );
    // // printpart(2, "?", &solution.1);

    // #[cfg(feature = "speedtest")]
    // printtotaltime(time);
}

fn rucksack_to_compartments(
    rucksack: &Vec<Vec<char>>
) -> Vec<(&Vec<char>, &Vec<char>)> {
    rucksack.iter()
        .step_by(2)
        .collect::<Vec<_>>()
        .iter()
        .map(|x| *x)
        .collect::<Vec<_>>()
        .iter()
    .zip(
        rucksack.iter()
            .skip(1)
            .step_by(2)
            .collect::<Vec<_>>()
            .iter()
            .map(|x| *x)
            .collect::<Vec<_>>()
            .iter()
    )
    .map(|pair|
        (*pair.0, *pair.1)
    )
    .collect::<Vec<(&Vec<char>, &Vec<char>)>>()
}

fn char_to_value(character: char) -> i32 {
    match character.is_ascii_uppercase() {
        // Uppercase
        true => {
            (character as u8 - 65 + 1 + 26) as i32
        },
        // Lowercase
        false => {
           (character as u8 - 97 + 1) as i32
        }
    }
}

fn rucksack_to_values(rucksack: &Vec<Vec<char>>) -> Vec<Vec<i32>> {
    rucksack.iter()
    .map(|bag| bag.iter()
        .map(|item|
            char_to_value(*item)
        ).collect::<Vec<i32>>()
    ).collect::<Vec<Vec<i32>>>()
}

fn charlist_to_values(charlist: &Vec<char>) -> Vec<i32> {
    charlist.iter().map(|x| char_to_value(*x)).collect::<Vec<i32>>()
}

fn rucksack_compartments_to_values<'a>(
    rucksack_compartments: Vec<(&'a Vec<char>, &'a Vec<char>)>
) -> Vec<(Vec<i32>, Vec<i32>)> {
    rucksack_compartments.iter()
        .map(|pair| // Pair of `Vec<char>`
            (
                pair.0.iter().map(|x| char_to_value(*x)).collect::<Vec<i32>>(),
                pair.0.iter().map(|x| char_to_value(*x)).collect::<Vec<i32>>()
            )
        ).collect::<Vec<(Vec<i32>, Vec<i32>)>>()
}

// fn rucksack_compartments_values_common_values(
//     rucksack_compartments_values: &Vec<(Vec<i32>, Vec<i32>)>
// ) -> Vec<i32> {
//     rucksack_compartments_values.iter()
//         .map(|tuple| // Only values that are in the other `Vec`
//             tuple.0.iter().filter(|x| tuple.1.contains(*x))
//             .exactly_one()
//             // .map(|item| *item)
//             // .collect::<i32>()
//         ).collect::<Vec<i32>>()
// }

// the problem
fn rucksack_compartments_values_common_values(
    rucksack_compartments_values: &Vec<(Vec<i32>, Vec<i32>)>
) -> Vec<i32> {
    rucksack_compartments_values
        .iter()
        .map(|tuple|
            ( // Only contains values that also exist in the other `Vec`
                tuple.0.iter().filter(|x| tuple.1.contains(*x))
                    .map(|item| *item)
                    .collect::<Vec<i32>>(),
                tuple.1.iter().filter(|x| tuple.0.contains(*x))
                    .map(|item| *item)
                    .collect::<Vec<i32>>()
            )
        ).collect::<Vec<(Vec<i32>, Vec<i32>)>>()
        .iter()
        .map(|tuple| // Sum each tuple
            tuple.0[0] + tuple.1[0]
        ).collect::<Vec<i32>>()
}
// fn rucksack_compartments_values_common_values(
//     rucksack_compartments_values: &Vec<(Vec<i32>, Vec<i32>)>
// ) -> Vec<i32> {
//     rucksack_compartments_values.iter()
//         .map(|tuple| // Only values that are in the other `Vec`
//             tuple.0.iter().filter(|x| tuple.1.contains(*x))
//             .map(|item| *item)
//             .sum(),
//         ).collect::<Vec<i32>>()
// }