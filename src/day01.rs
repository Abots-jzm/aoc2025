use crate::read_input;

pub fn solve() {
    let input = read_input(1);

    println!("  Part 1: {}", part1(&input));
    println!("  Part 2: {}", part2(&input));
}

fn part1(input: &str) -> isize {
    let mut position = 50;
    input
        .lines()
        .map(|line| line.parse::<Turn>().unwrap())
        .fold(0, |acc, turn| {
            position = match turn {
                Turn::Left(n) => (position - n).rem_euclid(100),
                Turn::Right(n) => (position + n).rem_euclid(100),
            };
            if position == 0 { acc + 1 } else { acc }
        })
}

fn part2(input: &str) -> isize {
    let mut position = 50;
    input
        .lines()
        .map(|line| line.parse::<Turn>().unwrap())
        .fold(0, |acc, turn| {
            let mut zero_count = 0;
            position = match turn {
                Turn::Left(n) => {
                    zero_count += n / 100;
                    let n = n % 100;
                    if position != 0 && n >= position {
                        zero_count += 1;
                    }
                    (position - n).rem_euclid(100)
                }
                Turn::Right(n) => {
                    zero_count += n / 100;
                    let n = n % 100;
                    if position != 0 && position + n >= 100 {
                        zero_count += 1;
                    }
                    (position + n).rem_euclid(100)
                }
            };
            acc + zero_count
        })
}

enum Turn {
    Left(isize),
    Right(isize),
}

impl std::str::FromStr for Turn {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, num) = s.split_at(1);
        let value: isize = num.parse().expect("should be a valid number");
        match dir {
            "L" => Ok(Turn::Left(value)),
            "R" => Ok(Turn::Right(value)),
            _ => Err(format!("Invalid turn direction: {}", dir)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 6);
    }
}
