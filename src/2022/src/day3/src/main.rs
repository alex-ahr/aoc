//! https://adventofcode.com/2022/day/3

#![feature(iter_intersperse)]

#[cfg(feature = "speedtest")]
use std::time::Instant;

use aoc::{printpart, printday, printtotaltime};
use itertools::{self, Itertools};

fn solve(input: &str) -> (i32,) {
    let rucksack = input_to_rucksack(input);
    
    /// Sum of values of common element 
    /// in each compartment of the rucksack.
    fn part1(rucksack: &Vec<[Vec<char>; 2]>) -> i32 {
        rucksack.iter()
            .map(|rucksack|
                // Leave only the one item that is in both `Vec<char>`s
                char_to_value(intersection_of_vec_chars(&rucksack[0], &rucksack[1])[0])
            ).collect::<Vec<i32>>()
            .iter()
            .sum()
    }

    /// Sum of the values of the badges.
    // fn part2(rucksack: &Vec<Vec<char>>) -> Vec<Vec<i32>> {
    //     rucksack.iter()
    //         .map(|bag|
    //             char_to_value(intersection_of_vec_chars(&bag[0],))
    //         )
    //         // .map(|line|
    //         //     line.iter().map(|char| char_to_value(*char)).collect::<Vec<i32>>()
    //         // ).collect::<Vec<Vec<i32>>>()
    //         // .chunks_exact(3)
    //         // .dedup()
    //         // // .map(|x| *x)
    //         // .collect::<Vec<Vec<_>>>()
    //         // // .collect::<Vec<Vec<i32>>>()
    //         // // .iter()
    //         // // .map(|x| *x)
    //         // // .collect::<Vec<Vec<i32>>>()
    // }

    return (part1(&rucksack),);
}

fn input_to_rucksack(input: &str) -> Vec<[Vec<char>; 2]> {
    input
        .split("\n")
        .map(|row| line_into_compartments(row.chars().collect::<Vec<char>>()))
        .collect::<Vec<[Vec<char>; 2]>>()
}

fn input_to_no_compartments(input: &str) -> Vec<Vec<char>> {
    input
        .split("\n")
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

/// A compartment is the line split in 2.
fn line_into_compartments(line: Vec<char>) -> [Vec<char>; 2] {
    [
        line.split_at(line.len() / 2).0.to_vec(),
        line.split_at(line.len() / 2).1.to_vec(),
    ]
}

/// Convers a char to it's value.
/// 
/// `'a'..='z'` = `1..=26`,
/// `'A'..='Z'` = `27..=52`.
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

fn intersection_of_vec_chars(chars1: &Vec<char>, chars2: &Vec<char>) -> Vec<char> {
    use std::collections::HashSet;
    let mut set1 = HashSet::new();

    for char in chars1 {
        set1.insert(char);
    }

    let mut set2 = HashSet::new();
    for char in chars2 {
        set2.insert(char);
    }

    set1.intersection(&set2).map(|x| **x).collect()
}

fn get_common_element_of_compartments(
    rucksack: Vec<[Vec<char>; 2]>
) -> Vec<char> {
    rucksack.iter()
        .map(|bag|
            intersection_of_vec_chars(&bag[0], &bag[1])[0]
        ).collect::<Vec<char>>()
}

fn main() {
    printday("Rucksack Reorganization");

    #[cfg(feature = "speedtest")]
    let time = Instant::now();

    let solution = solve(include_str!("../input.txt"));
    printpart(
        1,
        "Sum of priorities of item types",
        solution.0.to_string()
    );
    printpart(
        2,
        "Sum of priorities of item shared by group of 3",
        "SKIPPED".to_string()
        // solution.1.to_string()
    );

    #[cfg(feature = "speedtest")]
    printtotaltime(time);
}
