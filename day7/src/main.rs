use std::{collections::{HashMap, HashSet}, vec};

fn main() {
    println!("Solution for Part 1 tests: {}", part1(test_input()));
    println!("Solution for Part 1: {}", part1(input()));
    println!("Solution for Part 2 tests: {}", part2(test_input()));
    println!("Solution for Part 2: {}", part2(input()));

}

fn part1(input: &str) -> usize {
    // > X
    // ^ Y
    let (sx, sy) = parse_start(input);
    let splitters = parse_splitters(input);
    let mut positions = vec![
        (sx, sy)
    ];

    let mut split_count = 0;
    while positions[0].1 < input.lines().count() {
        let mut new_positions: HashSet<(usize, usize)> = HashSet::new();
        while positions.len() > 0 {
            let (x, y) = positions.pop().unwrap();
            // Move down by one
            let new_pos = (x, y + 1);

            if splitters.contains(&new_pos) {
                new_positions.insert((x - 1, y + 1));
                new_positions.insert((x + 1, y + 1));

                split_count += 1;
            } else {
                new_positions.insert(new_pos);
            }
        }

        positions = new_positions.into_iter().collect();
    }

    split_count
}

fn part2(input: &str) -> usize {
    // > X
    // ^ Y
    let (sx, sy) = parse_start(input);
    let splitters = parse_splitters(input);
    
    traverse(sx, sy, &splitters, input.lines().count(), &mut HashMap::new())
}

fn traverse(sx: usize, sy: usize, splitters: &HashSet<(usize, usize)>, max_y: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    // > X
    // ^ Y
    if sy >= max_y {
        return 1;
    }

    let new_pos = (sx, sy + 1);

    if let Some(&cached) = cache.get(&new_pos) {
        return cached;
    }

    if splitters.contains(&new_pos) {
        let splits = traverse(sx - 1, sy + 1, splitters, max_y, cache) + traverse(sx + 1, sy + 1, splitters, max_y, cache);

        cache.insert((sx, sy), splits);

        splits
    } else {
        let splits = traverse(sx, sy + 1, splitters, max_y, cache);

        cache.insert((sx, sy), splits);

        splits
    }
}

fn parse_start(input: &str) -> (usize, usize) {
    for (y, line) in input.lines().enumerate() {
        if let Some(x) = line.chars().position(|c| c == 'S') {
            return (x, y);
        }
    }
    panic!("Start position 'S' not found in input");
}

fn parse_splitters(input: &str) -> HashSet<(usize, usize)> {
    let mut splitters = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '^' {
                splitters.insert((x, y));
            }
        }
    }
    splitters
}

fn test_input() -> &'static str {
".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."
}

fn input() -> &'static str {
    include_str!("../input/day7.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = test_input();
        let result = part1(input);
        assert_eq!(result, 21);
    }

    #[test]
    fn test_part2() {
        let input = test_input();
        let result = part2(input);
        assert_eq!(result, 40);
    }
}