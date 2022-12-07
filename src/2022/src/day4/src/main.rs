use std::{collections::HashSet, time::Instant};

use aoc::{printday, printpart, printtotaltime};

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn solve(input: &str) -> (i32, i32) {
    let assignments = input
        .split("\n") // `"2-4,6-8"`
        .map(|line| {
            line.split(",") // `["2-4", "6-8"]`
                .collect::<Vec<&str>>()
                .iter()
                .map(|range| range.split("-").collect::<Vec<_>>()) // `[["2", "4"], ["6", "8"]]`
                .collect::<Vec<Vec<_>>>()
                .iter()
                // Convert `&str` -> `i32`
                .map(|rangepair| {
                    rangepair
                        .iter()
                        .map(|num| num.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>();

    // println!("{:?}", &assignments);

    /// How many assignments *completely* envelop another.
    fn part1(assignments: &Vec<Vec<Vec<i32>>>) -> i32 {
        assignments
            .iter()
            .map(|pair| {
                vec_completely_contains(
                    &(pair[0][0]..=pair[0][1]).collect::<Vec<i32>>(),
                    &(pair[1][0]..=pair[1][1]).collect::<Vec<i32>>(),
                )
            })
            .collect::<Vec<_>>()
            .iter()
            .filter(|x| **x == true)
            .count() as i32
    }

    /// All assignments that overlap.
    fn part2(assignments: &Vec<Vec<Vec<i32>>>) -> i32 {
        assignments
            .iter()
            .map(|pair|
                vec_intersection(
                    &(pair[0][0]..=pair[0][1]).collect::<Vec<i32>>(),
                    &(pair[1][0]..=pair[1][1]).collect::<Vec<i32>>()
                )
            ).collect::<Vec<_>>()
                .iter()
                // Convert lists with numbers to `true`,
                // and lists without to `false`.
                .map(|common_nums| {
                    if common_nums.len() != 0 {
                        true
                    } else { false }
                })
                .collect::<Vec<bool>>()
                .iter()
                .filter(|x| **x == true).count() as i32
    }

    return (part1(&assignments), part2(&assignments));
}

#[cfg(not(feature = "test"))]
fn main() {
    printday("Camp Cleanup");

    #[cfg(feature = "speedtest")]
    let time = Instant::now();

    let solution = solve(include_str!("../input.txt"));
    printpart(
        1,
        "In how many assignment pairs does one range fully contain the other?",
        solution.0.to_string(),
    );
    printpart(
        2,
        "In how many assignment pairs do the ranges overlap?",
        solution.1.to_string(),
    );

    #[cfg(feature = "speedtest")]
    printtotaltime(time);
}

/// Stolen from https://leetcode.com/problems/intersection-of-two-arrays/solutions/241522/rust-0ms-hashset-solution
fn vec_intersection(nums1: &Vec<i32>, nums2: &Vec<i32>) -> Vec<i32> {
    let n1 = nums1.clone();
    let n2 = nums2.clone();

    n1.into_iter()
        .collect::<HashSet<_>>()
        .intersection(&n2.into_iter().collect())
        .into_iter()
        .cloned()
        .collect()
}

/// Returns if either contains the other.
fn vec_completely_contains(nums1: &Vec<i32>, nums2: &Vec<i32>) -> bool {
    let mut n1 = nums1.clone();
    let mut n2 = nums2.clone();

    n1.sort_unstable();
    n2.sort_unstable();

    // `n1` larger than `n2`
    if n1[0] <= n2[0] && n1[n1.len() - 1] >= n2[n2.len() - 1] {
        true
    // `n2` larger than `n1`
    } else if n2[0] <= n1[0] && n2[n2.len() - 1] >= n1[n1.len() - 1] {
        true
    } else {
        false
    }
}

#[cfg(feature = "test")]
fn main() {
    let solution = solve(include_str!("../test.txt"));

    assert_eq!((2, 4), solution);

    printpart(
        1,
        "In how many assignment pairs does one range fully contain the other?",
        solution.0.to_string(),
    );
    printpart(
        2,
        "In how many assignment pairs do the ranges overlap?",
        solution.1.to_string(),
    );
}
