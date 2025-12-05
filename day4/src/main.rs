fn main() {
    println!("Solution part 1 test: {}", part1(test_input()));
    println!("Solution part 1: {}", part1(input()));
    println!("Solution part 2 test: {}", part2(test_input()));
    println!("Solution part 2: {}", part2(input()));
}

fn part1(input: &str) -> u32 {
    // Parse the input into a 2D matrix
    let matrix: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut count = 0;
    for y in 0..matrix.len() as i32 {
        for x in 0..matrix[0].len() as i32 {
            if let Some(cell) = get_cell(&matrix, x, y) {
                if cell == '@' && count_surrounding_toilet_paper(&matrix, x, y) < 4 {
                    count += 1;
                }
            }
        }
    }

    count
}

fn part2(input: &str) -> u32 {
    let mut matrix: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut count = 0;
    loop {
        let mut changes = 0;

        for y in 0..matrix.len() as i32 {
            for x in 0..matrix[0].len() as i32 {
                if let Some(cell) = get_cell(&matrix, x, y) {
                    let surrounding_count = count_surrounding_toilet_paper(&matrix, x, y);
                    if cell == '@' && surrounding_count < 4 {
                        matrix[y as usize][x as usize] = '.';
                        changes += 1;
                    }
                }
            }
        }

        count += changes;

        if changes == 0 {
            break;
        }
    }

    count
}

fn test_input() -> &'static str {
"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
}

fn input() -> &'static str {
    include_str!("../input/day4.txt")
}

fn get_cell(matrix: &Vec<Vec<char>>, x: i32, y: i32) -> Option<char> {
    let rows = matrix.len() as i32;
    let cols = matrix[0].len() as i32;

    if x >= cols || y >= rows {
        return None;
    }

    if x < 0 || y < 0 {
        return None;
    }

    Some(matrix[y as usize][x as usize])
}

fn count_surrounding_toilet_paper(matrix: &Vec<Vec<char>>, x: i32, y: i32) -> u32 {
    let directions = [
        (-1, -1), (0, -1), (1, -1),
        (-1,  0),          (1,  0),
        (-1,  1), (0,  1), (1,  1),
    ];

    let mut count = 0;
    for (dx, dy) in directions.iter() {
        if let Some(cell) = get_cell(matrix, x + dx, y + dy) {
            if cell == '@' {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = test_input();
        let expected = 13;
        let result = part1(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part2() {
        let input = test_input();
        let expected = 43;
        let result = part2(input);
        assert_eq!(result, expected);
    }
}