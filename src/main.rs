mod day01;
mod day02;
mod day03;

fn main() {
    println!("🎄 Advent of Code 2025 🎄\n");

    println!("Day 1:");
    day01::solve();
    println!("\nDay 2:");
    day02::solve();
    println!("\nDay 3:");
    day03::solve();
}

// Helper function to read input file for a given day
pub fn read_input(day: u8) -> String {
    let path = format!("inputs/day{:02}.txt", day);
    std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("Could not read input file: {}", path))
}

// Helper to read input as lines
pub fn read_lines(day: u8) -> Vec<String> {
    read_input(day).lines().map(|s| s.to_string()).collect()
}

// Helper to parse input as numbers
pub fn read_numbers<T: std::str::FromStr>(day: u8) -> Vec<T>
where
    T::Err: std::fmt::Debug,
{
    read_input(day)
        .lines()
        .map(|s| s.parse().unwrap())
        .collect()
}
