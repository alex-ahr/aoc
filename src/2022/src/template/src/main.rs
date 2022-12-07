//! link to day

static DAY_NAME: &str = "Day Name";

fn solve(input: &str) -> (???,) {
    fn part1() -> ??? {}

    // fn part2() -> ??? {}

    return (part1(),);
}

#[cfg(not(feature = "example"))]
fn main() {
    printday(DAY_NAME);

    #[cfg(feature = "speedtest")]
    let time = Instant::now();

    let solution = solve(include_str!("../input.txt"));
    printpart(
        1,
        "?",
        solution.0.to_string()
    );
    // printpart(
    //     2,
    //     "?",
    //     solution.1.to_string()
    // );

    #[cfg(feature = "speedtest")]
    printtotaltime(time);
}

#[cfg(feature = "example")]
fn main() {
    printday(DAY_NAME);

    let solution = solve(include_str!("../example.txt"));

    assert_eq!((???,), solution);

    printpart(
        1,
        "?",
        solution.0.to_string()
    );
    // printpart(
    //     2,
    //     "?",
    //     solution.1.to_string()
    // );
}