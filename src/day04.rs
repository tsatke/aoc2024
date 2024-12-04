const INPUT: &str = include_str!("../inputs/input_day4.txt");

const LINE_LENGTH: usize = 140;

pub fn part1() -> usize {
    let bytes = INPUT.as_bytes();

    let line_sum = bytes
        .array_windows::<4>()
        .filter(|&w| w == b"XMAS" || w == b"SAMX")
        .count();

    let col_sum: usize = bytes
        .chunks(LINE_LENGTH + 1)
        .map_windows::<_, _, 4>(|&lines| {
            let mut count = 0;

            let mut buf = [0; 4];

            // very last line doesn't contain a line break, so use the last line's length
            for i in 0..LINE_LENGTH {
                buf[0] = lines[0][i];
                buf[1] = lines[1][i];
                buf[2] = lines[2][i];
                buf[3] = lines[3][i];

                count += (&buf == b"XMAS" || &buf == b"SAMX") as usize;
            }

            count
        })
        .sum();

    let diag = bytes
        .chunks(LINE_LENGTH + 1)
        .map_windows::<_, _, 4>(|&lines| {
            let mut count = 0;

            let mut buf_lr = [0; 4];
            let mut buf_rl = [0; 4];

            for i in 0..LINE_LENGTH - 3 {
                buf_lr[0] = lines[0][i];
                buf_lr[1] = lines[1][i + 1];
                buf_lr[2] = lines[2][i + 2];
                buf_lr[3] = lines[3][i + 3];

                buf_rl[0] = lines[0][i + 3];
                buf_rl[1] = lines[1][i + 2];
                buf_rl[2] = lines[2][i + 1];
                buf_rl[3] = lines[3][i];

                count += (&buf_lr == b"XMAS" || &buf_lr == b"SAMX") as usize;
                count += (&buf_rl == b"XMAS" || &buf_rl == b"SAMX") as usize;
            }

            count
        })
        .sum::<usize>();

    line_sum + col_sum + diag
}

pub fn part2() -> usize {
    INPUT
        .as_bytes()
        .chunks(LINE_LENGTH + 1)
        .map_windows::<_, _, 3>(|&lines| {
            let mut count = 0;

            let mut buf_lr = [0; 2];
            let mut buf_rl = [0; 2];

            for i in 1..LINE_LENGTH - 1 {
                if lines[1][i] != b'A' {
                    continue;
                }

                buf_lr[0] = lines[0][i - 1];
                buf_lr[1] = lines[2][i + 1];

                buf_rl[0] = lines[0][i + 1];
                buf_rl[1] = lines[2][i - 1];

                count += ((&buf_lr == b"MS" || &buf_lr == b"SM")
                    && (&buf_rl == b"MS" || &buf_rl == b"SM")) as usize;
            }

            count
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_results() {
        assert_eq!(part1(), 2414);
        assert_eq!(part2(), 1871);
    }
}
