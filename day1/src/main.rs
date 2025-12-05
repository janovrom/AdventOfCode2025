fn main() {
    println!("Solution to part 1: {}", part1(input_part1()));
    println!("Solution to part 2: {}", part2(input_part1()));
    println!("Solution to part 2: {}", part2_v2(input_part1()));
}

fn part1(input: &str) -> i32 {
    // Each line is a char and a number, e.g. "L3" means move left 3 units.
    let mut start = 50;
    
    input
        .lines()
        .map(|line| {
            let (direction, distance) = parse_line(line);
            direction * distance
        })
        .map(|x| {
            start += x;
            start = start.rem_euclid(100);

            match start {
                0 => 1,
                _ => 0,
            }
        })
        .sum()
}

fn parse_line(line: &str) -> (i32, i32) {
    let (left, right) = line.split_at(1);
    let direction = match left {
        "L" => -1,
        "R" => 1,
        _ => panic!("Invalid direction"),
    };

    let distance: i32 = right.parse().unwrap();
    (direction, distance)
}

fn input_part1() -> &'static str {
    include_str!("../input/day1.txt")
}

fn part2(input: &str) -> i32 {
    let mut start = 50;
    input
        .lines()
        .map(|line| {
            let (direction, distance) = parse_line(line);
            direction * distance
        })
        .map(|x: i32| {
            let full_rotations = x.abs() / 100;
            
            let mut res = full_rotations;
            let diff = x - full_rotations * 100 * x.signum();
            let end: i32 = start + diff;

            if start != 0 && (end >= 100 || end <= 0) {
                // crossed zero
                res += 1;
            }
            
            let end = end.rem_euclid(100);
            start = end;

            res
        })
        .sum()
}

fn part2_v2(input: &str) -> i32 {
    let mut start = 50;
    input
        .lines()
        .map(|line| {
            let (direction, distance) = parse_line(line);
            direction * distance
        })
        .map(|x: i32| {
            let dir: i32 = if x >= 0 { 1 } else { -1 };
            let mut res = 0;

            for _ in 0..x.abs() {
                start += dir;
                start = start.rem_euclid(100);
                if start == 0 {
                    res += 1;
                }
            }
            
            res
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../input/test.txt");
        let result = part1(input);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input/test.txt");
        let result = part2(input);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_part2_v2() {
        let input = include_str!("../input/test.txt");
        let result = part2_v2(input);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_modulo() {
        assert_eq!((-1i32).rem_euclid(100), 99);
        assert_eq!((-150i32).rem_euclid(100), 50);
    }
}
