fn main() {
    println!("Solution for part 1 tests: {}", part1(test_input()));
    println!("Solution for part 1: {}", part1(input()));
    println!("Solution for part 2 tests: {}", part2(test_input()));
    println!("Solution for part 2: {}", part2(input()));
}

fn part1(input: &str) -> usize {
    42
}

fn part2(input: &str) -> usize {
    84
}

fn test_input() -> &'static str {
    "test input"
}

fn input() -> &'static str {
    include_str!("../input/day10.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(test_input()), 42);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(test_input()), 84);
    }
}