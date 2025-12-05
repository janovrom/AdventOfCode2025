fn main() {
    println!("Part 1 Test: {}", part1(test_input()));
    println!("Solution for Part 1: {}", part1(input()));
    println!("Part 2 Test: {}", part2(test_input()));
    println!("Solution for Part 2: {}", part2(input()));
}

fn part1(input: &'static str) -> i32 {
    input
        .lines()
        .map(|line| line_to_numbers(line))
        .map(|numbers| {
            // Get the two largest numbers
            let mut max = 0;
            let mut max_index = 0;
            let mut second_max = 0;

            for (i, x) in numbers.iter().take(numbers.len() - 1).enumerate() {
                if *x > max {
                    max = *x;
                    max_index = i;
                }
            }

            // If we are second to last, we can just take it
            if max_index == numbers.len() - 2 {
                return max * 10 + numbers[numbers.len() - 1];
            } else {
                for (_, x) in numbers.iter().skip(max_index + 1).enumerate() {
                    if *x > second_max{
                        second_max = *x;
                    }
                }
            }

            max * 10 + second_max
        })
        .sum()
}

fn part2(input: &'static str) -> i64 {
    input
        .lines()
        .map(|line| line_to_numbers(line))
        .map(|numbers| {
            let res = maximum_subset_monotonic(&numbers);
            res
        })
        .sum()
}

fn maximum_subset_monotonic(numbers: &[i32]) -> i64 {
    let mut stack = Vec::new();
    let k = 12; // We want 12 digits
    let n = numbers.len();
    
    for (i, &digit) in numbers.iter().enumerate() {
        // While we can still remove elements and the current digit is larger
        while !stack.is_empty() 
            && stack.last().unwrap() < &digit 
            && stack.len() + (n - i) > k {
            stack.pop();
        }
        
        if stack.len() < k {
            stack.push(digit);
        }
    }
    
    // Convert to number
    stack.iter().fold(0i64, |acc, &digit| acc * 10 + digit as i64)
}

fn line_to_numbers(line: &str) -> Vec<i32> {
    line
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i32)
        .collect()
}

fn maximum_subset(numbers: &[i32], candidates: &mut Vec<usize>, visited: &mut Vec<bool>, start: usize, cache: &mut std::collections::HashMap<u64, Option<i64>>) -> Option<i64> {
    let key = hash_state(numbers, visited);
    if let Some(&cached) = cache.get(&key) {
        return cached;
    }
    
    if candidates.len() == 12 {
        // We have a full set. Sum it up.
        let result = to_number(numbers, candidates);

        cache.insert(key, Some(result));

        return Some(result);
    }

    let mut max = 0;
    for i in start..numbers.len() {
        if visited[i] {
            continue; // Already used
        }

        // This is a candidate, recurse
        candidates.push(i);
        visited[i] = true; // Mark as used

        if let Some(m) = maximum_subset(numbers, candidates, visited, start + 1, cache) {
            if max < m {
                max = m;
            }
        }

        candidates.pop();
        visited[i] = false;
    }

    cache.insert(key, Some(max));

    Some(max)
}

fn maximum_subset_slow(numbers: &[i32], candidates: &mut Vec<usize>, visited: &mut Vec<bool>, start: usize, cache: &mut std::collections::HashMap<u64, Option<i64>>) -> Option<i64> {
    let key = hash_state(numbers, visited);
    if let Some(&cached) = cache.get(&key) {
        return cached;
    }
    
    if candidates.len() == 12 {
        // We have a full set. Sum it up.
        let result = to_number(numbers, candidates);

        cache.insert(key, Some(result));

        return Some(result);
    }

    let mut max = 0;
    for i in start..numbers.len() {
        if visited[i] {
            continue; // Already used
        }

        // This is a candidate, recurse
        candidates.push(i);
        visited[i] = true; // Mark as used

        if let Some(m) = maximum_subset(numbers, candidates, visited, start + 1, cache) {
            if max < m {
                max = m;
            }
        }

        candidates.pop();
        visited[i] = false;
    }

    cache.insert(key, Some(max));

    Some(max)
}

fn test_input() -> &'static str {
"987654321111111
811111111111119
234234234234278
818181911112111"
}

fn to_number(numbers: &[i32], candidates: &Vec<usize>) -> i64 {
    let mut key = 0i64;
    let mut candidates = candidates.clone();
    candidates.sort();
    for c in candidates {
        key = key * 10 + numbers[c] as i64;
    }
    key
}

fn hash_state(numbers: &[i32], candidates: &[bool]) -> u64 {
    let mut key = 0u64;
    let mut i = 0;
    for visited in candidates {
        if *visited {
            let c = numbers[i] as u64;
            key = key * 10 + c;
        }

        i += 1;
    }
    key
}

fn input() -> &'static str {
    include_str!("../input/day3.txt")
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_part1() {
        let input = test_input();
        let result = part1(input);
        assert_eq!(result, 357);
    }

    #[test]
    fn test_part2() {
        let input = test_input();
        let result = part2(input);
        assert_eq!(result, 3121910778619);
    }
}