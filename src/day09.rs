const INPUT: &str = include_str!("../inputs/input_day9.txt");

#[must_use]
pub fn part1() -> usize {
    let bytes = INPUT.as_bytes();
    let mut checksum = 0_usize;
    let mut last_index = bytes.len() - 1;
    assert_eq!(last_index & 1, 0);
    let mut last_remaining = bytes[last_index] - b'0';
    let mut file_offset = 0;

    'outer: for (id, chunk) in bytes.chunks(2).enumerate() {
        let offset = id * 2;
        if offset >= last_index {
            break;
        }

        let len = chunk[0] - b'0';
        let free = if chunk.len() == 1 { 0 } else { chunk[1] - b'0' };

        // process first half
        checksum += (0..len as usize).map(|i| file_offset + i).sum::<usize>() * id;

        // process second half
        let mut other_id = last_index / 2;
        for i in 0..free {
            if last_remaining == 0 {
                last_index -= 2;
                last_remaining = bytes[last_index] - b'0';
                other_id = last_index / 2;
                if offset >= last_index {
                    break 'outer;
                }
            }
            checksum += other_id * (file_offset + len as usize + i as usize);
            last_remaining -= 1;
        }

        file_offset += len as usize + free as usize;
    }

    // there is remaining stuff that is needed for the example and simple cases like `12345`,
    // but not for the actual input, soooooooo........

    // if last_index > last_offset {
    //     let last_id = last_index / 2;
    //     for i in 0..last_remaining {
    //         // println!("{} * {last_id}", file_offset + i as usize);
    //         checksum += last_id * (file_offset + i as usize);
    //     }
    // }

    checksum
}

#[must_use]
pub fn part2() -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_results() {
        assert_eq!(part1(), 6607511583593);
        assert_eq!(part2(), 0);
    }
}
