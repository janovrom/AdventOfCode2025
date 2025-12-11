use std::{collections::{HashMap, HashSet, VecDeque}, env};

fn main() {
    println!("Solution for part 1 tests: {}", part1(test_input()));
    println!("Solution for part 1: {}", part1(input()));
    println!("Solution for part 2: {}", part2(input()));
    
    if env::args().any(|arg| arg == "--export-graphviz") {
        export_part2_to_graphviz(input());
    }
}

fn export_part2_to_graphviz(input: &str) {
    let devices = parse_input(input);
    let start = Device { value: "svr".to_string() };

    let mut file_content = String::new();
    file_content.push_str("digraph G {\n");

    let mut visited = HashSet::new();
    let mut stack = Vec::new();
    stack.push(start.clone());
    visited.insert(start.clone());

    while let Some(current) = stack.pop() {
        if let Some(neighbors) = devices.get(&current) {
            for neighbor in neighbors {
                file_content.push_str(&format!("    \"{}\" -> \"{}\";\n", current.value, neighbor.value));

                if !visited.contains(&neighbor.clone()) {
                    stack.push(neighbor.clone());
                    visited.insert(neighbor.clone());
                }
            }
        }
    }

    file_content.push_str("}\n");

    std::fs::write("day11_part2_graphviz.dot", file_content).expect("Unable to write file");
}

fn part1(input: &str) -> usize {
    let devices = parse_input(input);
    let start = Device { value: "you".to_string() };
    let out_device = Device { value: "out".to_string() };

    solve_bfs(&devices, &start, &out_device)
}

fn part2(input: &str) -> u64 {
    let devices = parse_input(input);
    let start = Device { value: "svr".to_string() };
    let out_device = Device { value: "out".to_string() };
    let fast_fourier_transform = Device { value: "fft".to_string() };
    let digital_analog_converter = Device { value: "dac".to_string() };
    
    // From graphviz analysis, we know following:
    // - fft is quite early in the graph
    // - dac is quite late in the graph
    // Therefore, we can optimize, by finding all paths from srv->fft first,
    // then from fft->dac, and finally from dac->out.

    let solutions_srv_fft = solve_bfs_with_depth(&devices, &start, &fast_fourier_transform);
    println!("Solutions from srv to fft: {}", solutions_srv_fft);
    
    // As fft to dac is still quite large, we can use the nodes that separate them to split the search
    // yyy, brz, quh, cya, eya, lvi
    let separators = vec!["yyy", "brz", "quh", "eya", "lvi"];
    let mut solutions_fft_dac = 0;
    for sep in &separators {
        let sep_device = Device { value: sep.to_string() };
        let solutions_fft_sep = solve_bfs_with_depth(&devices, &fast_fourier_transform, &sep_device);
        let solutions_sep_dac = solve_bfs_with_depth(&devices, &sep_device, &digital_analog_converter);
        println!("Solutions from fft to {}: {}", sep, solutions_fft_sep);
        println!("Solutions from {} to dac: {}", sep, solutions_sep_dac);
        solutions_fft_dac += solutions_fft_sep * solutions_sep_dac;
    }

    // let solutions_fft_dac = solve_bfs_with_depth(&devices, &fast_fourier_transform, &digital_analog_converter);
    // println!("Solutions from fft to dac: {}", solutions_fft_dac);
    let solutions_dac_out = solve_bfs_with_depth(&devices, &digital_analog_converter, &out_device);
    println!("Solutions from dac to out: {}", solutions_dac_out);

    solutions_srv_fft as u64 * solutions_fft_dac as u64 * solutions_dac_out as u64
}

fn solve_bfs_with_depth(devices: &HashMap<Device, Vec<Device>>, start: &Device, target: &Device) -> usize {
    let mut queue = VecDeque::new();

    queue.push_back((start.clone(), 0));

    let mut target_depth = None;
    let mut solution_count = 0;
    while let Some((current, depth)) = queue.pop_front() {
        if &current == target {
            solution_count += 1;
            if target_depth.is_none() {
                target_depth = Some(depth);
            }
            continue;
        }

        // There are often many paths that are longer than the shortest path.
        // We can limit the search depth to target_depth + 1 to avoid exploring too deep
        if depth > target_depth.unwrap_or(100000) + 1 {
            continue;
        }

        if let Some(neighbors) = devices.get(&current) {
            for neighbor in neighbors {
                queue.push_back((neighbor.clone(), depth + 1));
            }
        }
    }

    solution_count
}

fn solve_bfs(devices: &HashMap<Device, Vec<Device>>, start: &Device, target: &Device) -> usize {
    let mut queue = VecDeque::new();

    queue.push_back((start.clone(), 0));

    let mut solution_count = 0;
    while let Some((current, depth)) = queue.pop_front() {
        if &current == target {
            solution_count += 1;
            continue;
        }

        if let Some(neighbors) = devices.get(&current) {
            for neighbor in neighbors {
                queue.push_back((neighbor.clone(), depth + 1));
            }
        }
    }

    solution_count
}

fn parse_input(input: &str) -> HashMap<Device, Vec<Device>> {
    let mut devices = HashMap::new();
    for line in input.lines() {
        let parts = line.split(": ").collect::<Vec<&str>>();
        let device = Device { value: parts[0].to_string() };
        let connections = if parts.len() > 1 {
            parts[1].split(" ").map(|s| Device { value: s.to_string() }).collect()
        } else {
            Vec::new()
        };
        devices.insert(device, connections);
    }
    devices
}

fn input() -> &'static str {
    include_str!("../input/day11.txt")
}

fn test_input() -> &'static str {
"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out"
}

#[allow(dead_code)]
fn test_input_2() -> &'static str {
"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Device {
    value: String
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(test_input());
        assert_eq!(result, 5);
    }

    #[test]
    fn test_part2() {
        let result = part2(test_input_2());
        assert_eq!(result, 2);
    }

    // --- IGNORE ---
    // Runs too long
    // #[test]
    // fn test_fft_to_dac() {
    //     let input = input();
    //     let devices = parse_input(input);
    //     let fft = Device { value: "fft".to_string() };
    //     let dac = Device { value: "dac".to_string() };

    //     let result = solve_bfs_with_depth(&devices, &fft, &dac);
    //     assert!(result > 0);
    // }

    #[test]
    fn test_dac_to_fft_does_not_exist() {
        let input = input();
        let devices = parse_input(input);
        let fft = Device { value: "fft".to_string() };
        let dac = Device { value: "dac".to_string() };

        let result = solve_bfs(&devices, &dac, &fft);
        assert_eq!(result, 0);
    }
}