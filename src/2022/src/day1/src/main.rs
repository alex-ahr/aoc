//! https://adventofcode.com/2022/day/1

// A bit of bs to make it faster
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

// /// Inventories are lists of numbers
// /// separated by 2 newlines for each elf.
// ///
// /// ```
// /// 1000\n
// /// 2000\n
// /// 3000\n
// /// \n
// /// 2000\n
// /// 2000\n
// /// \n
// /// ```
// ///
// /// Or more accurately:
// ///
// /// ```
// /// 1000\n2000\n3000\n\n2000\n2000\n\n
// /// ```
// ///
// /// should yield `vec![1000, 2000, 3000, 2000, 2000]`
// pub struct Day01; impl Solution for Day01 {
//     fn name() { "Calorie Counting".to_string() }
//     fn solve() {
//         let input = include_str!("../input.txt");
//         fn part1(input: String) -> Vec<i32> {
//             include_str!("./input.txt")
//                 .split("\n\n")
//                .map(|elf| {
//                     elf.split("\n")
//                         .map(|x| {
//                             x.trim_end()
//                                 .parse::<i32>()
//                                 .unwrap()
//                         }).sum::<i32>()
//             }).collect::<Vec<_>>();
//         };
//         fn part2();

//         return (
//             {
//                 println!("#P1: ")
//             },
//             {}
//         );
//     }
// }

fn max_element_position(vec: Vec<i32>) -> i32 {
    vec.iter()
        .position(|x| *x == *vec.iter().max().unwrap())
        .unwrap() as i32
}
fn main() {
    /// Inventories are lists of numbers
    /// separated by 2 newlines for each elf.
    ///
    /// ```
    /// 1000\n
    /// 2000\n
    /// 3000\n
    /// \n
    /// 2000\n
    /// 2000\n
    /// \n
    /// ```
    ///
    /// Or more accurately:
    ///
    /// ```
    /// 1000\n2000\n3000\n\n2000\n2000\n\n
    /// ```
    ///
    /// should yield `vec![1000, 2000, 3000, 2000, 2000]`
    let elf_inventories: Vec<i32> = include_str!("./input.txt")
        .split("\n\n")
        .map(|elf| {
            elf.split("\n")
                .map(|x| x.trim_end().parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect::<Vec<_>>();

    println!(
        "#P1: Highest amount of total calories:\n{:?}",
        elf_inventories[max_element_position(elf_inventories.clone()) as usize]
    );

    println!("#P2: Sum of top 3:\n{:?}", {
        let mut sorted = elf_inventories.clone();
        sorted.sort_unstable();
        {
            sorted[sorted.len() - 1] + sorted[sorted.len() - 2] + sorted[sorted.len() - 3]
        }
    });
}
