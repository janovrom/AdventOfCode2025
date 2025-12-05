fn main() {
    println!("Solution for tests: {}", part1(test_input()));
    println!("Solution for part1: {}", part1(input()));
    println!("Solution for tests: {}", part2(test_input()));
    println!("Solution for part2: {}", part2(input()));
}

fn test_input() -> &'static str {
    "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"
}

fn input() -> &'static str {
    include_str!("../input/day2.txt")
}

fn part1(input: &'static str) -> i64 {
    let mut result: i64 = 0;
    input
        .split(",")
        .flat_map(|x| {
            // Each entry is start-end
            let split: Vec<&str> = x.split('-').collect();
            let start = split[0];
            let end = split[1];
            
            let start = start.parse::<i64>().unwrap();
            let end = end.parse::<i64>().unwrap() + 1;

            start..end
        })
        .for_each(|x| {
            if is_sequence(x) {
                result += x;
            }
        });

    result
}

fn part2(input: &'static str) -> i64 {
    let mut result: i64 = 0;
    input
        .split(",")
        .flat_map(|x| {
            // Each entry is start-end
            let split: Vec<&str> = x.split('-').collect();
            let start = split[0];
            let end = split[1];
            
            let start = start.parse::<i64>().unwrap();
            let end = end.parse::<i64>().unwrap() + 1;

            start..end
        })
        .for_each(|x| {
            if is_repeated_sequence(x) {
                result += x;
            }
        });

    result
}

fn is_sequence(x: i64) -> bool {
    let len = get_length(x);

    // If it's even check from start to middle
    // 1212 - len=4 -> 0..2, 2..
    // of in fact, modulo 100 and divide by 100
    // If it's odd, skip middle
    // 121 - len=3 -> 0..1, 2..
    if len % 2 == 0 {
        let m: i64 = 10_i64.pow(len / 2);
        x / m == x % m
    } else {
        // Sequence should be repeated twice (cannot be odd)
        false
        //let m: i64 = 10_i64.pow(len / 2);
        //let modulo = x % m;
        //let m = m * 10;
        //let divide = x / m;
        //modulo == divide
    }
}

fn is_repeated_sequence(x: i64) -> bool {
    // As above, but we cannot assume the length
    let len = get_length(x);

    // Expected maximum of iterations
    let max = len / 2 + 1;
    for i in 1..max {
        let m = 10_i64.pow(i);
        let pattern = x % m;
        let mut tmp = x;
        let mut is_repeated = true;

        // For 13: m=10, pattern=3
        // Iter 1: tmp=13, modulo=3, tmp=1
        // Iter 2: tmp=1, modulo=1, tmp=0

        while tmp != 0 {
            // For 1_001_001 will pass for pattern 001
            // so check that the remaining length is enough
            if get_length(tmp) < i {
                is_repeated = false;
                break
            }

            let modulo = tmp % m;

            if modulo != pattern {
                is_repeated = false;
                break;
            }

            tmp = tmp / m;
        }

        if is_repeated {
            return true;
        }
    }

    false
}

fn get_length(x: i64) -> u32 {
    let mut tmp = x;
    let mut len = 0;

    while tmp != 0 {
        len += 1;
        tmp /= 10;
    }

    len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let expected = 1227775554;
        let result = part1(test_input());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part2() {
        let expected = 4174379265;
        let result = part2(test_input());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_length() {
        let expected = 2;
        let result = get_length(15);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_init() {
        let expected = 3;
        let result = (0..3).len();
        assert_eq!(result, expected);
    }

    #[test]
    fn expected_sequence_part2() {
        let expected = 1111111;
        assert_eq!(is_repeated_sequence(expected), true);
    }
}
