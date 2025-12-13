use crate::read_input;

pub fn solve() {
    let input = read_input(2);

    println!("  Part 1: {}", part1(&input));
    println!("  Part 2: {}", part2(&input));
}

fn part1(input: &str) -> isize {
    input
        .split(',')
        .map(|range| {
            range
                .split_once('-')
                .expect("there should be a '-' in the range")
        })
        .fold(0, |acc, (start, end)| {
            let start_num = start.parse::<isize>().unwrap();
            let end_num = end.parse::<isize>().unwrap();

            let mut total = 0;
            let half_len = start.len() / 2;
            let half = &start[..half_len];
            let mut half_num = half.parse::<isize>().unwrap_or(0);
            if start.len() % 2 == 1 {
                if half_num == 0 {
                    half_num = 1
                } else {
                    let digits = (half_num as f64).log10().floor() as u32 + 1;
                    half_num = 10_u64.pow(digits) as isize;
                }
            };

            loop {
                let cur_num = format!("{}{}", half_num, half_num)
                    .parse::<isize>()
                    .unwrap();
                half_num += 1;

                if cur_num < start_num {
                    continue;
                }

                if cur_num > end_num {
                    break;
                }

                total += cur_num;
            }
            acc + total
        })
}

fn part2(input: &str) -> isize {
    input
        .split(',')
        .map(|range| {
            range
                .split_once('-')
                .expect("there should be a '-' in the range")
        })
        .fold(0, |acc, (start, end)| {
            let start_num = start.parse::<isize>().unwrap();
            let end_num = end.parse::<isize>().unwrap();
            let mut seen = std::collections::HashSet::new();

            let mut total = 0;
            let mut chunks = 2;

            while chunks <= end.len() {
                let chunk_len = start.len() / chunks;
                let chunk = &start[..chunk_len];
                let mut chunk_num = chunk.parse::<isize>().unwrap_or(0);
                if start.len() % chunks != 0 {
                    if chunk_num == 0 {
                        chunk_num = 1
                    } else {
                        let digits = (chunk_num as f64).log10().floor() as u32 + 1;
                        chunk_num = 10_u64.pow(digits) as isize;
                    }
                };

                loop {
                    let cur_num = chunk_num
                        .to_string()
                        .repeat(chunks)
                        .parse::<isize>()
                        .unwrap();
                    chunk_num += 1;

                    if cur_num < start_num {
                        continue;
                    }

                    if cur_num > end_num {
                        break;
                    }

                    if !seen.contains(&cur_num) {
                        total += cur_num;
                        seen.insert(cur_num);
                    }
                }

                chunks += 1;
            }
            acc + total
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 1227775554);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 4174379265);
    }
}
