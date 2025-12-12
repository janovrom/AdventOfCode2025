fn main() {
    println!("Solution for Part 1 tests: {}", part1(test_input()));
    println!("Solution for Part 1: {}", part1(input()));
}

fn part1(input: &str) -> usize {
    let (presents, regions) = parse_input(input);
    // For each region, try arrange required presents
    let mut total_possible_arrangements = 0;
    for region in regions {
        // Build the list of presents needed for this region
        let mut expected_presents = Vec::new();
        for (i, v) in region.expected_presents.iter().enumerate() {
            for _ in 0..*v {
                expected_presents.push(presents[i].clone());
            }
        }
        // We can quickly reject solution that has total area of presents larger than region area
        let region_area = region.width * region.height;
        let total_presents_area: usize = expected_presents.iter().map(|p| p.size).sum();
        if total_presents_area > region_area {
            continue;
        }

        // Try to arrange presents in the region
        let mut total_arrangements = 0;
        let mut current_arrangement = Vec::new();
        let index = 0;
        try_arrange_presents(&region, &presents, index, &mut current_arrangement, &mut total_arrangements);
        if total_arrangements > 0 {
            total_possible_arrangements += 1;
        }
    }

    total_possible_arrangements
}

fn try_arrange_presents(
    region: &Region,
    presents: &Vec<Present>,
    index: usize,
    current_arrangement: &mut Vec<Placement>,
    total_arrangements: &mut usize,
) {
    // Base case: all presents placed
    if index >= presents.len() {
        *total_arrangements += 1;
        // println!("Arrangement found:");
        // print_arrangement(region, presents, current_arrangement);

        return;
    }

    if *total_arrangements > 0 {
        // Display arrangement found
        return;
    }

    let positions = get_free_positions(region, presents, current_arrangement);

    // Check if I can place the present at any of the free positions in any transformation
    let present = &presents[index];
    let transformations = generate_transformations(present);
    for transformed_present in transformations {
        for (row, col) in &positions {
            if can_place_present(region, &transformed_present, &presents, &current_arrangement, *row, *col) {
                // Place present
                current_arrangement.push(Placement {
                    row: *row,
                    col: *col,
                    present_index: index,
                });
                // Recurse to place next present
                try_arrange_presents(region, presents, index + 1, current_arrangement, total_arrangements);

                // If arrangement found, no need to try further
                if *total_arrangements > 0 {
                    return;
                }

                // Backtrack
                current_arrangement.pop();
            }
        }
    }
}

#[allow(dead_code)]
fn print_arrangement(
    region: &Region,
    presents: &Vec<Present>,
    current_arrangement: &Vec<Placement>,
) {
    let mut grid = vec![vec!['.'; region.width]; region.height];
    for placement in current_arrangement {
        let present = &presents[placement.present_index];
        let present_char = (b'A' + placement.present_index as u8) as char;
        for r in 0..present.shape.len() {
            for c in 0..present.shape[0].len() {
                if present.shape[r][c] {
                    grid[placement.row + r][placement.col + c] = present_char;
                }
            }
        }
    }

    for row in grid {
        let line: String = row.into_iter().collect();
        println!("{}", line);
    }
}

fn can_place_present(
    region: &Region,
    present: &Present,
    presents: &Vec<Present>,
    current_arrangement: &Vec<Placement>,
    row: usize,
    col: usize,
) -> bool {
    let present_rows = present.shape.len();
    let present_cols = present.shape[0].len();

    // Check bounds
    if row + present_rows > region.height || col + present_cols > region.width {
        return false;
    }

    // Check for overlaps with already placed presents
    for placement in current_arrangement {
        let placed_present = &presents[placement.present_index];
        for r in 0..placed_present.shape.len() {
            for c in 0..placed_present.shape[0].len() {
                if placed_present.shape[r][c] {
                    let placed_row = placement.row + r;
                    let placed_col = placement.col + c;
                    for pr in 0..present_rows {
                        for pc in 0..present_cols {
                            if present.shape[pr][pc] {
                                let new_row = row + pr;
                                let new_col = col + pc;
                                if new_row == placed_row && new_col == placed_col {
                                    return false; // Overlap detected
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    true
}

fn get_free_positions(
    region: &Region,
    presents: &Vec<Present>,
    current_arrangement: &Vec<Placement>,
) -> Vec<(usize, usize)> {
    let mut occupied = vec![vec![false; region.width]; region.height];
    for placement in current_arrangement {
        let present = &presents[placement.present_index];
        for r in 0..present.shape.len() {
            for c in 0..present.shape[0].len() {
                if present.shape[r][c] {
                    occupied[placement.row + r][placement.col + c] = true;
                }
            }
        }
    }

    let mut free_positions = Vec::new();
    for r in 0..region.height {
        for c in 0..region.width {
            if !occupied[r][c] {
                free_positions.push((r, c));
            }
        }
    }

    free_positions
}

fn generate_transformations(present: &Present) -> Vec<Present> {
    let mut transformations = Vec::new();
    transformations.push(present.clone());

    // Generate rotations and flips
    let r_90 = rotate_90(present);
    let r_180 = rotate_180(present);
    let r_270 = rotate_270(present);
    let f_h = flip_horizontal(present);
    let f_v = flip_vertical(present);

    // Then for each rotation, add its flips
    transformations.push(flip_horizontal(&r_90));
    transformations.push(flip_vertical(&r_90));
    transformations.push(flip_horizontal(&r_180));
    transformations.push(flip_vertical(&r_180));
    transformations.push(flip_horizontal(&r_270));
    transformations.push(flip_vertical(&r_270));

    // The for each flip, add its rotations
    transformations.push(rotate_90(&f_h));
    transformations.push(rotate_180(&f_h));
    transformations.push(rotate_270(&f_h));
    transformations.push(rotate_90(&f_v));
    transformations.push(rotate_180(&f_v));
    transformations.push(rotate_270(&f_v));

    // Add the original rotations and flips
    transformations.push(r_90);
    transformations.push(r_180);
    transformations.push(r_270);
    transformations.push(f_h);
    transformations.push(f_v);

    // Remove duplicates
    transformations.sort_by(|a, b| a.shape.cmp(&b.shape));
    transformations.dedup();
    
    transformations
}

fn rotate_90(present: &Present) -> Present {
    let rows = present.shape.len();
    let cols = present.shape[0].len();
    let mut new_shape = vec![vec![false; rows]; cols];
    for r in 0..rows {
        for c in 0..cols {
            new_shape[c][rows - r - 1] = present.shape[r][c];
        }
    }
    Present { shape: new_shape, size: present.size}
}

fn rotate_180(present: &Present) -> Present {
    rotate_90(&rotate_90(present))
}

fn rotate_270(present: &Present) -> Present {
    rotate_90(&rotate_180(present))
}

fn flip_horizontal(present: &Present) -> Present {
    let rows = present.shape.len();
    let cols = present.shape[0].len();
    let mut new_shape = vec![vec![false; cols]; rows];
    for r in 0..rows {
        for c in 0..cols {
            new_shape[r][cols - c - 1] = present.shape[r][c];
        }
    }
    Present { shape: new_shape, size: present.size}
}

fn flip_vertical(present: &Present) -> Present {
    let rows = present.shape.len();
    let cols = present.shape[0].len();
    let mut new_shape = vec![vec![false; cols]; rows];
    for r in 0..rows {
        for c in 0..cols {
            new_shape[rows - r - 1][c] = present.shape[r][c];
        }
    }
    Present { shape: new_shape, size: present.size}
}

fn test_input() -> &'static str {
"0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2"
}

fn input() -> &'static str {
    include_str!("../input/day12.txt")
}

fn parse_input(input: &str) -> (Vec<Present>, Vec<Region>) {
    let sections: Vec<&str> = input.split("\n\n").collect();
    let mut presents = Vec::new();
    let mut regions = Vec::new();

    for section in &sections[..sections.len() - 1] {
        let lines: Vec<&str> = section.lines().collect();
        let shape: Vec<Vec<bool>> = lines[1..]
            .iter()
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect();
        let size = shape.iter().flatten().filter(|&&b| b).count();
        presents.push(Present { shape, size });
    }

    for region_line in sections.last().unwrap().lines() {
        let parts: Vec<&str> = region_line.split(':').collect();
        let dimensions: Vec<usize> = parts[0]
            .split('x')
            .map(|d| d.parse().unwrap())
            .collect();
        let expected_presents: Vec<usize> = parts[1]
            .trim()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        regions.push(Region {
            width: dimensions[0],
            height: dimensions[1],
            expected_presents,
        });
    }

    (presents, regions)
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Present {
    shape: Vec<Vec<bool>>,
    size: usize,
}

struct Placement {
    row: usize,
    col: usize,
    present_index: usize,
}

struct Region {
    width: usize,
    height: usize,
    expected_presents: Vec<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let test_result = part1(test_input());
        assert_eq!(test_result, 2);
    }
}