use crate::read_input;

pub fn solve() {
    let input = read_input(3);

    println!("  Part 1: {}", part1(&input));
    println!("  Part 2: {}", part2(&input));
}

fn part1(input: &str) -> isize {
    input
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            let mut first = chars.next().unwrap();
            let mut second = chars.next().unwrap();

            chars.into_iter().for_each(|c| {
                if second > first {
                    first = second;
                    second = c;
                    return;
                }

                if c > second {
                    second = c;
                }
            });

            format!("{}{}", first, second).parse::<isize>().unwrap()
        })
        .sum()
}

fn part2(input: &str) -> isize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 357);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 3121910778619);
    }
}
