fn main() {
    println!("Solution for Part 1 tests: {}", part1(test_input()));
    println!("Solution for Part 1: {}", part1(input()));
    println!("Solution for Part 2 tests: {}", part2(test_input()));
    println!("Solution for Part 2: {}", part2(input()));
}

fn part1(input: &str) -> i64 {
    let (data, operations) = parse_input(input);
    let mut result: i64 = 0;

    // Zip columns with their operations
    for i in 0..data[0].len() {
        let ops = &operations;
        let mut result_row: i64 = match ops[i] {
            Operation::Mul => 1,
            Operation::Add => 0,
        };

        for j in 0..data.len() {
            let cell = data[j][i];
            match ops[i] {
                Operation::Mul => result_row *= cell as i64,
                Operation::Add => result_row += cell as i64,
            }
        }

        result += result_row;
    }

    result
}

fn part2(input: &str) -> i64 {
    // Now we need to consider that the values are vertically aligned
    // 123
    //  45
    //   6
    // +
    // This will be 356 + 24 + 1
    let data_lines: Vec<&str> = input.lines().filter(|line| !line.trim().is_empty()).collect();
    let operations_line = data_lines.last().unwrap();
    let num_rows = data_lines.len() - 1;
    let mut start = 0;
    let mut sum = 0;
    while next_operation(operations_line, start).is_some() {
        let (op, _) = next_operation(operations_line, start).unwrap();
        let (_, next_start) = next_operation(operations_line, start + 1)
            .unwrap_or((Operation::Add, operations_line.len() + 1));
        
        // Process numbers
        let mut result = match op {
            Operation::Mul => 1,
            Operation::Add => 0,
        };

        for x in start..(next_start-1){
            let mut number = 0;

            for row in 0..num_rows {
                let line = data_lines[row];
                let c = line.chars().nth(x).unwrap();
                if c.is_whitespace() {
                    continue;
                }
                number = number * 10 + c.to_digit(10).unwrap() as i64;
            }

            result = match op {
                Operation::Mul => result * number,
                Operation::Add => result + number,
            };
        }

        start = next_start;
        sum += result;
    }

    sum
}   

fn next_operation(str: &str, start: usize) -> Option<(Operation, usize)> {
    for (i, c) in str.chars().enumerate().skip(start) {
        if let Some(op) = Operation::from_char(c) {
            return Some((op, i));
        }
    }

    None
}

fn parse_input(input: &str) -> (Vec<Vec<i32>>, Vec<Operation>) {
    let data = input
        .lines()
        .take(input.lines().count() - 1)
        .filter(|line| line.len() > 0)
        .map(|line| {
            line.split_whitespace()
                .filter_map(|num_str| num_str.parse::<i32>().ok())
                .collect()
        })
        .collect();

    let operations: Vec<Operation> = input
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .filter_map(|op_str| Operation::from_char(op_str.chars().next()?))
        .collect();

    (data, operations)
}

enum Operation {
    Mul,
    Add,
}

impl Operation {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '*' => Some(Operation::Mul),
            '+' => Some(Operation::Add),
            _ => None,
        }
    }
}

fn test_input() -> &'static str {
r"
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
"
}

fn input() -> &'static str {
    include_str!("../input/day6.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = test_input();
        let result = part1(input);
        assert_eq!(result, 4277556);
    }

    #[test]
    fn test_part2() {
        let input = test_input();
        let result = part2(input);
        assert_eq!(result, 3263827);
    }
}