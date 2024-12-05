macro_rules! solve {
    ($day:expr, $pt1:expr, $pt2:expr) => {
        println!("{:2}.1 {}", $day, $pt1());
        println!("{:2}.2 {}", $day, $pt2());
    };
}

pub fn main() {
    solve!(1, aoc2024::day01::part1, aoc2024::day01::part2);
    solve!(2, aoc2024::day02::part1, aoc2024::day02::part2);
    solve!(3, aoc2024::day03::part1, aoc2024::day03::part2);
    solve!(4, aoc2024::day04::part1, aoc2024::day04::part2);
    solve!(5, aoc2024::day05::part1, aoc2024::day05::part2);
}
