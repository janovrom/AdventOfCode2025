fn main() {
    println!("Solution part 1 test: {}", part1(test_input()));
    println!("Solution part 1: {}", part1(&input()));
    println!("Solution part 2 test: {}", part2(test_input()));
    println!("Solution part 2: {}", part2(&input()));
}

fn part1(input: &str) -> u64 {
    let (intervals, numbers) = get_intervals(input);
    let mut count = 0;

    for number in numbers {
        for (min, max) in &intervals {
            if is_fresh(number, *min, *max ) {
                count += 1;
                break;
            }
        }
    }

    count
}

fn part2(input: &str) -> u64 {
    let (mut intervals, _) = get_intervals(input);

    // Merge intervals
    intervals.sort_by(|a, b| a.0.cmp(&b.0));
    let mut merged_intervals: Vec<(u64, u64)> = Vec::new();

    for interval in intervals {
        if let Some(last) = merged_intervals.last_mut() {
            if interval.0 <= last.1 {
                last.1 = last.1.max(interval.1);
            } else {
                merged_intervals.push(interval);
            }
        } else {
            merged_intervals.push(interval);
        }
    }

    let mut count = 0;

    for interval in merged_intervals {
        count += interval.1 - interval.0 + 1;
    }

    count
}

fn get_intervals(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let sections = input.split("\n\n").collect::<Vec<&str>>();
    let intervals = sections[0]
        .lines()
        .map(|line| {
            let bounds = line
                .split('-')
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            (bounds[0], bounds[1])
        })
        .collect::<Vec<(u64, u64)>>();

    let numbers = sections[1]
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    (intervals, numbers)
}

fn is_fresh(x: u64, min: u64, max: u64) -> bool {
    x >= min && x <= max
}

fn strip_r(s: &str) -> String {
    s.replace("\r", "")
}

fn test_input() -> &'static str {
"3-5
10-14
16-20
12-18

1
5
8
11
17
32"
}

fn input() -> String {
    strip_r(include_str!("../input/day5.txt"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = test_input();
        let result = part1(input);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part2() {
        let input = test_input();
        let result = part2(input);
        assert_eq!(result, 14);
    }
}