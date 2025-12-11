use std::collections::HashMap;

fn main() {
    println!("Solution for part 1 tests: {}", part1(test_input()));
    println!("Solution for part 1: {}", part1(input()));
    println!("Solution for part 2 tests: {}", part2(test_input()));
    println!("Solution for part 2: {}", part2(input()));
}

fn part1(input: &str) -> usize {
    let machines = parse_input(input);

    let mut solutions: Vec<u32> = Vec::new();
    for machine in &machines {
        // I can basically find the minimum configuration by exploring all possible configurations
        // Start with only one button press, then two, etc.
        let mut found_solution = false;
        for i in 1..machine.buttons.len() {
            // find all combinations of i buttons
            let mut combinations: Vec<Vec<&Button>> = Vec::new();
                   
            let mut current: Vec<&Button> = Vec::new();
            backtrack(&machine.buttons, 0, &mut current, i, &mut combinations);

            for combination in combinations {
                let mut new_light = machine.lights.clone();
                for button in combination {
                    new_light.press(button);
                }
                if new_light.is_solved() {
                    solutions.push(i as u32);
                    found_solution = true;
                    break;
                }
            }

            if found_solution {
                break;
            }
        }

        if !found_solution {
            panic!("We should have found a solution by now");
        }
    }

    solutions.iter().sum::<u32>() as usize
}

fn backtrack<'a>(buttons: &'a Vec<Button>, start: usize, current: &mut Vec<&'a Button>, i: usize, combinations: &mut Vec<Vec<&'a Button>>) {
    if current.len() == i {
        combinations.push(current.clone());
        return;
    }
    for j in start..buttons.len() {
        current.push(&buttons[j]);
        backtrack(buttons, j + 1, current, i, combinations);
        current.pop();
    }
}

fn solve_joltage_heuristic_with_dfs(machine: &Machine) -> Option<usize> {
    let mut cache: HashMap<Vec<u32>, Option<usize>> = HashMap::new();
    let mut current = machine.joltage.clone();

    // Greedy approach: prioritize buttons that contribute most efficiently
    let mut current = vec![0u32; machine.joltage.target.len()];
    let mut presses = 0;

    // Record all presses made
    let mut press_sequence: Vec<(Vec<u32>, usize)> = Vec::new();
        
    while current != machine.joltage.target {
        press_sequence.push((current.clone(), presses));

        let mut best_button = None;
        let mut best_score = 0.0;
        
        for (btn_idx, button) in machine.buttons.iter().enumerate() {
            let mut score = 0.0;
            let mut useful = false;
            
            for (i, &current_val) in current.iter().enumerate() {
                if button.contributes_to(i) {
                    if current_val < machine.joltage.target[i] {
                        score += 1.0;
                        useful = true;
                    } else {
                        score -= 10.0; // Penalty for overshooting
                    }
                }
            }
            
            if useful && score > best_score {
                best_score = score;
                best_button = Some(btn_idx);
            }
        }
        
        if let Some(btn_idx) = best_button {
            for (i, val) in current.iter_mut().enumerate() {
                if machine.buttons[btn_idx].contributes_to(i) {
                    *val += 1;
                }
            }
            presses += 1;
        } else {
            // No useful button found, break to avoid infinite loop
            // but continue to DFS search
            break;
        }
    }

    let mut current_joltage = machine.joltage.clone();
    current_joltage.state = current.clone();

    let mut visited: HashMap<Vec<u32>, usize> = HashMap::new();
    for (joltage, presses) in press_sequence.iter().rev() {
        let mut current_joltage = Joltage { state: joltage.clone(), target: machine.joltage.target.clone() };
        let result = find_joltage(&machine.buttons, &mut current_joltage, *presses, &mut cache, &mut visited);
        if let Some(steps) = result {
            return Some(steps);
        }

        // Reverse the computation to get to the heuristic state
        let mut current_joltage = Joltage::new(joltage);
        _ = find_joltage(&machine.buttons, &mut current_joltage, 0, &mut cache, &mut visited)
    }

    None
}

fn find_joltage_with_heuristic(buttons: &Vec<Button>, current: &mut Joltage, depth: u32, cache: &mut HashMap<Vec<u32>, (u32, Vec<u32>)>) -> (u32, Vec<u32>) {
    if let Some(cached) = cache.get(&current.state) {
        return cached.clone();
    }

    if current.is_solved() {
        return (depth, current.state.clone());
    }

    if current.too_high() {
        return (u32::MAX, current.state.clone());
    }

    let mut min_steps = (u32::MAX, current.state.clone());

    let ordered_buttons = order_buttons_by_contribution(&Machine {
        lights: Lights::new(0),
        buttons: buttons.clone(),
        joltage: current.clone(),
    }, &current.state);

    for &btn_idx in &ordered_buttons {
        let button = &buttons[btn_idx];
        current.press(button);
        let steps = find_joltage_with_heuristic(buttons, current, depth + 1, cache);
        
        if steps.0 < min_steps.0 {
            min_steps = steps;
        }

        // backtrack
        for (j, val) in current.state.iter_mut().enumerate() {
            if (button.wiring & (1 << j)) != 0 {
                *val -= 1;
            }
        }
    }

    cache.insert(current.state.clone(), min_steps.clone());

    min_steps
}

fn solve_joltage_heuristic(machine: &Machine) -> (u32, Vec<u32>) {
    // Greedy approach: prioritize buttons that contribute most efficiently
    let mut current = vec![0u32; machine.joltage.target.len()];
    let mut presses = 0;
    
    while current != machine.joltage.target {
        let mut best_button = None;
        let mut best_score = 0.0;
        
        for (btn_idx, button) in machine.buttons.iter().enumerate() {
            let mut score = 0.0;
            let mut useful = false;
            
            for (i, &current_val) in current.iter().enumerate() {
                if button.contributes_to(i) {
                    if current_val < machine.joltage.target[i] {
                        score += 1.0;
                        useful = true;
                    } else {
                        score -= 10.0; // Penalty for overshooting
                    }
                }
            }
            
            if useful && score > best_score {
                best_score = score;
                best_button = Some(btn_idx);
            }
        }
        
        if let Some(btn_idx) = best_button {
            for (i, val) in current.iter_mut().enumerate() {
                if machine.buttons[btn_idx].contributes_to(i) {
                    *val += 1;
                }
            }
            presses += 1;
        } else {
            break;
        }
    }
    
    (presses, current)
}

fn get_most_contributing_button(machine: &Machine, current: &Vec<u32>) -> Option<usize> {
    let mut best_button = None;
    let mut best_contribution = 0;

    for (btn_idx, button) in machine.buttons.iter().enumerate() {
        let mut contribution = 0;
        for (i, &current_val) in current.iter().enumerate() {
            if button.contributes_to(i) && current_val < machine.joltage.target[i] {
                contribution += 1;
            }
        }

        if contribution > best_contribution {
            best_contribution = contribution;
            best_button = Some(btn_idx);
        }
    }

    best_button
}

fn order_buttons_by_contribution(machine: &Machine, current: &Vec<u32>) -> Vec<usize> {
    let mut button_contributions: Vec<(usize, u32)> = machine.buttons.iter().enumerate().map(|(idx, button)| {
        let mut contribution = 0;
        for (i, &current_val) in current.iter().enumerate() {
            if button.contributes_to(i) && current_val < machine.joltage.target[i] {
                contribution += 1;
            }
        }
        (idx, contribution)
    }).collect();

    button_contributions.sort_by(|a, b| b.1.cmp(&a.1)); // Sort descending by contribution

    button_contributions.into_iter().map(|(idx, _)| idx).collect()
}

fn part2(input: &str) -> usize {
    let machines = parse_input(input);

    let mut solutions: Vec<u32> = Vec::new();
    for machine in &machines {
        // Techinically this is an equation (0 0 0 1) * x + (0 1 0 1) * y + (1 0 1 0) * z + (1 1 0 0) * w = (3 5 4 7)
        // where x,y,z,w are the number of times each button is pressed
        // The only problem is that x,y,z,w have to be non-negative integers
        // and I have no idea how to solve that easily
        // Also what if we have more buttons than equations?


        println!("Solving machine with target: {:?}", machine.joltage.target);
        
        let solution = solve_joltage_heuristic_with_dfs(machine);
        println!();

        solutions.push(solution.unwrap() as u32 );
    }
    solutions.iter().sum::<u32>() as usize
}

fn solve_joltage_bfs(machine: &Machine) -> Option<usize> {
    let current = machine.joltage.clone();
    let mut cache: HashMap<Vec<u32>, usize> = HashMap::new();

    // Populate cache with one button presses
    // for button in &machine.buttons {
    //     let mut new_state = current.clone();
    //     new_state.press(button);
    //     cache.insert(new_state.state.clone(), 1);
    // }
    
    let mut queue = std::collections::VecDeque::new();
    queue.push_back((current.clone(), 0usize)); // (state, depth)

    while let Some((state, depth)) = queue.pop_front() {
        print!("\rExplored depth {} with queue size {}", depth, queue.len());

        if state.is_solved() {
            return Some(depth);
        }

        if state.too_high() {
            continue;
        }

        // let best_button = get_most_contributing_button(&Machine {
        //     lights: Lights::new(0),
        //     buttons: machine.buttons.clone(),
        //     joltage: state.clone(),
        // }, &state.state);
        // // Apply the best button first
        // if let Some(btn_idx) = best_button {
        //     let mut new_state = state.clone();
        //     new_state.press(&machine.buttons[btn_idx]);
        //     if !cache.contains_key(&new_state.state) {
        //         cache.insert(new_state.state.clone(), depth + 1);
        //         queue.push_back((new_state, depth + 1));
        //     }
        // }

        let ordered_buttons = order_buttons_by_contribution(&Machine {
            lights: Lights::new(0),
            buttons: machine.buttons.clone(),
            joltage: state.clone(),
        }, &state.state);

        for &btn_idx in &ordered_buttons {
            let mut new_state = state.clone();
            new_state.press(&machine.buttons[btn_idx]);
            if !cache.contains_key(&new_state.state) {
                cache.insert(new_state.state.clone(), depth + 1);
                queue.push_back((new_state, depth + 1));
            }
        }
    }

    None
}

// fn solve_joltage_bfs(machine: &Machine) -> Option<usize> {
//     let current = machine.joltage.clone();
//     let mut cache: HashMap<Vec<u32>, Option<usize>> = HashMap::new();
    
//     let mut queue = std::collections::VecDeque::new();
//     queue.push_back((current.clone(), 0usize)); // (state, depth)

//     let mut min_depth = None;
//     while let Some((state, depth)) = queue.pop_front() {
//         print!("\rExplored depth {} with queue size {}", depth, queue.len());

//         // If we have already found a solution at a lower depth, skip
//         if let Some(min_depth_val) = min_depth {
//             if depth >= min_depth_val {
//                 continue;
//             }
//         }

//         if state.is_solved() {
//             if min_depth.is_none() || depth < min_depth.unwrap() {
//                 min_depth = Some(depth);
//             }
//             continue;
//         }

//         if state.too_high() {
//             continue;
//         }

//         // If cache contains this state at a lower depth, skip
//         if let Some(cached) = cache.get(&state.state) {
//             if let Some(cached_depth) = cached {
//                 if *cached_depth <= depth {
//                     continue;
//                 }
//             }
//         }

//         for button in &machine.buttons {
//             let mut new_state = state.clone();
//             new_state.press(button);

//             if !cache.contains_key(&new_state.state) {
//                 cache.insert(new_state.state.clone(), Some(depth + 1));
//                 queue.push_back((new_state, depth + 1));
//             } else {
//                 if let Some(cached_depth) = cache.get(&new_state.state).unwrap() {
//                     if *cached_depth > depth + 1 {
//                         cache.insert(new_state.state.clone(), Some(depth + 1));
//                         queue.push_back((new_state, depth + 1));
//                     }
//                 }
//             }
//         }

//         // if !cache.contains_key(&state.state) {
//         //     cache.insert(state.state.clone(), Some(depth));
//         // } else {
//         //     if let Some(cached_depth) = cache.get(&state.state).unwrap() {
//         //         if *cached_depth > depth {
//         //             cache.insert(state.state.clone(), Some(depth));
//         //         }
//         //     }
//         // }
//     }

//     min_depth
// }

fn solve_joltage_dfs(machine: &Machine) -> Option<usize> {
    let mut cache: HashMap<Vec<u32>, Option<usize>> = HashMap::new();
    let mut visited: HashMap<Vec<u32>, usize> = HashMap::new();
    let mut current = machine.joltage.clone();

    // // Get all possible combinations of button presses that could lead to the target
    // // and use these sets instead of exploring all buttons at each step
    // let
    
    // Mask out only few of the buttons at a time to see if we can solve partial targets
    // and fill the cache with these partial solutions
    let mut combinations: Vec<Vec<&Button>> = Vec::new();
    for i in 1..=machine.buttons.len() {
        let mut current_combination: Vec<&Button> = Vec::new();
        backtrack(&machine.buttons, 0, &mut current_combination, i, &mut combinations);
    }

    for combo in &combinations {
        let mut new_joltage = machine.joltage.clone();
        let mut depth = 0;
        while !new_joltage.too_high() {
            // Apply the combination
            for button in combo {
                new_joltage.press(button);    
            }

            if new_joltage.too_high() {
                break;
            }

            depth += combo.len();

            // Cache the current state
            if !visited.contains_key(&new_joltage.state) {
                visited.insert(new_joltage.state.clone(), depth);
            } else {
                // It might be possible to reach this state in fewer steps
                if let Some(cached_depth) = visited.get(&new_joltage.state) {
                    if *cached_depth > depth {
                        visited.insert(new_joltage.state.clone(), depth);
                    }
                }
            }
        }
    }

    find_joltage(&machine.buttons, &mut current, 0, &mut cache, &mut visited)
}

/// Too slow, keeping for reference
#[allow(dead_code)]
fn find_joltage(buttons: &Vec<Button>, current: &mut Joltage, depth: usize, cache: &mut HashMap<Vec<u32>, Option<usize>>, visited: &mut HashMap<Vec<u32>, usize>) -> Option<usize> {
    // We also need to find a way to cache states we've already seen at a certain depth
    if let Some(visited_depth) = visited.get(&current.state) {
        if *visited_depth < depth {
            return None;
        } else {
            // This is now the best depth we've seen for this state so far
            visited.insert(current.state.clone(), depth);
        }
    } else {
        visited.insert(current.state.clone(), depth);
    }


    if let Some(cached) = cache.get(&current.state) {
        return *cached;
    }
    
    if current.is_solved() {
        return Some(depth);
    }

    if current.too_high() {
        return None;
    }

    let mut min_steps = None;

    let ordered_buttons = order_buttons_by_contribution(&Machine {
        lights: Lights::new(0),
        buttons: buttons.clone(),
        joltage: current.clone(),
    }, &current.state);

    for &btn_idx in &ordered_buttons {
        let button = &buttons[btn_idx];
        current.press(button);
        let steps = find_joltage(buttons, current, depth + 1, cache, visited);
        
        if let Some(s) = steps { 
            if min_steps.is_none() || s < min_steps.unwrap() {
                min_steps = Some(s);
            }
        }

        // backtrack
        for (j, val) in current.state.iter_mut().enumerate() {
            if (button.wiring & (1 << j)) != 0 {
                *val -= 1;
            }
        }
    }

    cache.insert(current.state.clone(), min_steps);

    min_steps
}

fn test_input() -> &'static str {
"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"
}

fn input() -> &'static str {
    include_str!("../input/day10.txt")
}

fn parse_input(input: &str) -> Vec<Machine> {
    input.lines().map(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let lights: Lights = parts[0].parse().unwrap();
        let buttons: Vec<Button> = parts[1..parts.len()-1]
            .iter()
            .map(|&s| s.parse().unwrap())
            .collect();
        let joltage: Joltage = parts[parts.len()-1].parse().unwrap();
        Machine { lights, buttons, joltage }
    }).collect()
}

struct Machine {
    lights: Lights,
    buttons: Vec<Button>,
    joltage: Joltage,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Lights {
    target: u32,
    state: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Button {
    wiring: u32,
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Joltage {
    state: Vec<u32>,
    target: Vec<u32>,
}

impl Lights {
    fn new(target: u32) -> Self {
        Self { target, state: 0 }
    }

    fn press(&mut self, button: &Button) {
        self.state ^= button.wiring;
    }

    fn is_solved(&self) -> bool {
        self.state == self.target
    }
}

impl Button {
    fn new(wiring: u32) -> Self {
        Self { wiring }
    }

    fn contributes_to(&self, idx: usize) -> bool {
        (self.wiring & (1 << idx)) != 0
    }
}

impl Joltage{
    fn new(target: &[u32]) -> Self {
        Self { state: vec![0; target.len()], target: target.to_vec() }
    }

    fn press(&mut self, button: &Button) {
        for (i, val) in self.state.iter_mut().enumerate() {
            if (button.wiring & (1 << i)) != 0 {
                *val += 1;
            }
        }
    }

    fn is_solved(&self) -> bool {
        self.state == self.target
    }

    fn too_high(&self) -> bool {
        for (i, val) in self.state.iter().enumerate() {
            if *val > self.target[i] {
                return true;
            }
        }
        false
    }
}

impl std::str::FromStr for Lights {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Example: "[.##.]"
        let target = s
            .chars()
            .enumerate()
            .fold(0, |acc, (i, c)| if c == '#' { acc | (1 << (i - 1)) } else { acc });
        Ok(Lights::new(target))
    }
}

impl std::str::FromStr for Button {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Example: "(0,2,3,4)"
        let wiring = s
            .chars()
            .fold(0, |acc, c| 
                match c {
                    '0'..='9' => acc | (1 << (c as u32 - '0' as u32)),
                    _ => acc,
                });
        Ok(Button { wiring })
    }
}

impl std::str::FromStr for Joltage {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Example: "{3,5,4,7}"
        let target =s
            .trim_start_matches('{')
            .trim_end_matches('}')
            .split(',')
            .map(|num_str| num_str.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        Ok(Joltage { state: vec![0; target.len()], target: target.to_vec() })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(test_input()), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(test_input()), 33);
    }
}