macro_rules! solve {
    ($day:expr, $pt1:expr, $pt2:expr) => {
        println!("{:2}.1 {}", $day, $pt1());
        println!("{:2}.2 {}", $day, $pt2());
    };
}

pub fn main() {
    solve!(01, aoc2024::day01::part1, aoc2024::day01::part2);
}
