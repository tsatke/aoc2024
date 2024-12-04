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
            for i in 0..lines[3].len() {
                buf[0] = lines[0][i];
                buf[1] = lines[1][i];
                buf[2] = lines[2][i];
                buf[3] = lines[3][i];

                count += (&buf == b"XMAS" || &buf == b"SAMX") as usize;
            }

            count
        })
        .sum();

    let mut diag_sum_lr = (0..LINE_LENGTH)
        .map(move |i| {
            (i..LINE_LENGTH)
                .zip(0..LINE_LENGTH - i)
                .map(move |(c, r)| r * (LINE_LENGTH + 1) + c)
        })
        .filter(|it| it.len() >= 4)
        .rev()
        .map(|diag| count_xmas_samx(diag.map(|i| bytes[i])))
        .sum::<usize>();

    diag_sum_lr += (1..LINE_LENGTH)
        .map(move |i| {
            (i..LINE_LENGTH)
                .zip(0..LINE_LENGTH - i)
                .map(move |(r, c)| r * (LINE_LENGTH + 1) + c)
        })
        .filter(|it| it.len() >= 4)
        .map(|diag| count_xmas_samx(diag.map(|i| bytes[i])))
        .sum::<usize>();

    let mut diag_sum_rl = (0..LINE_LENGTH)
        .map(move |i| {
            (0..i + 1)
                .zip(0..i + 1)
                .map(move |(c, r)| r * (LINE_LENGTH + 1) + (i - c))
        })
        .filter(|it| it.len() >= 4)
        .map(|diag| count_xmas_samx(diag.map(|i| bytes[i])))
        .sum::<usize>();

    diag_sum_rl += (1..LINE_LENGTH)
        .map(move |i| {
            (i..LINE_LENGTH)
                .zip(i..LINE_LENGTH)
                .map(move |(c, r)| r * (LINE_LENGTH + 1) + (LINE_LENGTH - c + i - 1))
        })
        .filter(|it| it.len() >= 4)
        .map(|diag| count_xmas_samx(diag.map(|i| bytes[i])))
        .sum::<usize>();

    line_sum + col_sum + diag_sum_lr + diag_sum_rl
}

fn count_xmas_samx(haystack: impl Iterator<Item = u8>) -> usize {
    haystack
        .map_windows::<_, _, 4>(|w| w == b"XMAS" || w == b"SAMX")
        .filter(|&b| b)
        .count()
}

pub fn part2() -> usize {
    let bytes = INPUT.as_bytes();
    bytes
        .iter()
        .enumerate()
        .take(bytes.len() - LINE_LENGTH - 2)
        .skip(LINE_LENGTH + 2)
        .filter(|(i, b)| {
            if i % (LINE_LENGTH + 1) == 0 {
                return false;
            }

            if **b != b'A' {
                return false;
            }

            let top_right = i - LINE_LENGTH;
            let top_left = top_right - 2;
            let bottom_left = i + LINE_LENGTH;
            let bottom_right = bottom_left + 2;

            ((bytes[top_left] == b'M' && bytes[bottom_right] == b'S')
                || (bytes[top_left] == b'S' && bytes[bottom_right] == b'M'))
                && ((bytes[top_right] == b'M' && bytes[bottom_left] == b'S')
                    || (bytes[top_right] == b'S' && bytes[bottom_left] == b'M'))
        })
        .count()
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
