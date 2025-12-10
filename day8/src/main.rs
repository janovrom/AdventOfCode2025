use std::collections::HashMap;

fn main() {
    println!("Solution part 1 test: {}", part1(test_input(), 10));
    println!("Solution part 1: {}", part1(input(), 1000));
    println!("Solution part 2 test: {}", part2(test_input()));
    println!("Solution part 2: {}", part2(input()));
}

fn part1(input: &str, connections: usize) -> u32 {
    let junction_boxes = parse_input(input);
    let mut distances = compute_distances(&junction_boxes);
    distances.sort_by(|x, y| {
        x.2.partial_cmp(&y.2).unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut sets = vec![0; junction_boxes.len()];
    
    distances
        .into_iter()
        .take(connections)
        .for_each(|(i, j, _dist)| {
            let set_i = sets[i];
            let set_j = sets[j];

            if set_i == 0 && set_j == 0 {
                let new_set = i + 1;
                sets[i] = new_set;
                sets[j] = new_set;
            } else if set_i != 0 && set_j == 0 {
                sets[j] = set_i;
            } else if set_i == 0 && set_j != 0 {
                sets[i] = set_j;
            } else if set_i != set_j {
                let higher_set = set_i.max(set_j);
                let lower_set = set_i.min(set_j);
                for k in 0..sets.len() {
                    if sets[k] == higher_set {
                        sets[k] = lower_set;
                    }
                }
            }
        });

    let mut set_sizes = HashMap::new();
    for set in sets {
        if set == 0 {
            continue;
        }

        set_sizes
            .entry(set)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    // Get 3 largest by value
    let mut sizes = set_sizes.values().cloned().collect::<Vec<u32>>();
    sizes.sort();
    sizes.into_iter().rev().take(3).product::<u32>()
}

fn part2(input: &str) -> u32 {
    let junction_boxes = parse_input(input);
    let mut distances = compute_distances(&junction_boxes);
    distances.sort_by(|x, y| {
        x.2.partial_cmp(&y.2).unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut sets = vec![0; junction_boxes.len()];
    
    let mut last_cable = 0;
    for (i, j, _dist) in distances {
        let set_i = sets[i];
        let set_j = sets[j];

        if set_i == 0 && set_j == 0 {
            let new_set = i + 1;
            sets[i] = new_set;
            sets[j] = new_set;
        } else if set_i != 0 && set_j == 0 {
            sets[j] = set_i;
        } else if set_i == 0 && set_j != 0 {
            sets[i] = set_j;
        } else if set_i != set_j {
            let higher_set = set_i.max(set_j);
            let lower_set = set_i.min(set_j);
            for k in 0..sets.len() {
                if sets[k] == higher_set {
                    sets[k] = lower_set;
                }
            }
        }

        // Check if all boxes are now connected
        let first_set = sets[0];
        if sets.iter().all(|&s| s == first_set) {
            last_cable = junction_boxes[i].x * junction_boxes[j].x;
            break;
        }
    }

    last_cable as u32
}

fn compute_distances(boxes: &Vec<JunctionBox>) -> Vec<(usize, usize, f32)> {
    let mut distances: Vec<(usize, usize, f32)> = vec![];

    for i in 0..boxes.len() {
        for j in i+1..boxes.len() {
            distances.push((i, j, boxes[i].distance(&boxes[j])));
        }
    }

    distances
}

fn parse_input(input: &str) -> Vec<JunctionBox> {
    input.lines().map(|line| JunctionBox::from_str(line)).collect()
}

#[derive(Eq, PartialEq, Debug)]
struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
}

impl JunctionBox {
    fn from_str(s: &str) -> Self {
        let coords = s
            .split(',')
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        JunctionBox {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        }
    }

    /// Euclidean distance between two junction boxes
    fn distance(&self, other: &JunctionBox) -> f32 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)) as f32).sqrt()
    }
}

fn test_input() -> &'static str {
    "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"
}

fn input() -> &'static str {
    include_str!("../input/day8.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = test_input();
        let result = part1(input, 10);
        assert_eq!(result, 40);
    }

    #[test]
    fn test_part2() {
        let input = test_input();
        let result = part2(input);
        assert_eq!(result, 25272);
    }
}