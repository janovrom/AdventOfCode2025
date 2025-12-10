fn main() {
    println!("Solution for Part 1 tests: {}", part1(test_input()));
    println!("Solution for Part 1: {}", part1(input()));
    println!("Solution for Part 2 tests: {}", part2(test_input()));
    println!("Solution for Part 2: {}", part2(input()));
}

fn part1(input: &str) -> u64 {
    let red_tiles = parse_input(input);
    cartesian_pairs(red_tiles.len())
        .map(|(i, j)| {
            let tile_1 = red_tiles[i as usize];
            let tile_2 = red_tiles[j as usize];
            
            tile_1.area(&tile_2)
        })
        .max()
        .unwrap_or(0)
}

fn part2(input: &str) -> u64 {
    let red_tiles = parse_input(input);
    let polygon = Polygon::from_red_tiles(&red_tiles);

    cartesian_pairs(red_tiles.len())
        .map(|(i, j)| {
            let tile_1 = red_tiles[i as usize];
            let tile_2 = red_tiles[j as usize];
            let rectangle = Rectangle {
                min: Point {
                    x: tile_1.0.min(tile_2.0),
                    y: tile_1.1.min(tile_2.1),
                },
                max: Point {
                    x: tile_1.0.max(tile_2.0),
                    y: tile_1.1.max(tile_2.1),
                },
            };
            if polygon.contains(&rectangle) {
                rectangle.area()
            } else {
                0
            }
        })
        .max()
        .unwrap_or(0)
}

fn parse_input(input: &str) -> Vec<(u32, u32)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse::<u32>().unwrap();
            let y = parts.next().unwrap().parse::<u32>().unwrap();
            (x, y)
        })
        .collect()
}

fn cartesian_pairs(length: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..length).flat_map(move |x| (x+1..length).map(move |y| (x, y)))
}

trait Area {
    fn area(&self, other: &(u32, u32)) -> u64;
}

impl Area for (u32, u32) {
    fn area(&self, other: &(u32, u32)) -> u64 {
        let width = (self.0 as i32 - other.0 as i32 + 1).abs() as u64;
        let height = (self.1 as i32 - other.1 as i32 + 1).abs() as u64;
        width * height
    }
}

struct Edge {
    start: Point,
    end: Point,
}

struct Rectangle {
    min: Point,
    max: Point,
}

struct Polygon {
    edges: Vec<Edge>,
}

struct Point {
    x: u32,
    y: u32,
}

impl Rectangle {
    fn corners(&self) -> [Point; 4] {
        [
            Point { x: self.min.x, y: self.min.y },
            Point { x: self.max.x, y: self.min.y },
            Point { x: self.min.x, y: self.max.y },
            Point { x: self.max.x, y: self.max.y },
        ]
    }

    fn area(&self) -> u64 {
        let width = (self.max.x - self.min.x + 1) as u64;
        let height = (self.max.y - self.min.y + 1) as u64;
        width * height
    }
}

impl Polygon {
    fn contains(&self, rectangle: &Rectangle) -> bool {
        // First check if all corners are inside the polygon
        for corner in rectangle.corners() {
            if !self.contains_point(&corner) {
                return false;
            }
        }
        
        // If all corners are inside, check that no edges cross the polygon boundary
        // Actually, we need to check if the rectangle boundary intersects the polygon boundary
        let rect_edges = [
            // Top edge
            Edge { start: Point { x: rectangle.min.x, y: rectangle.min.y }, end: Point { x: rectangle.max.x, y: rectangle.min.y } },
            // Right edge  
            Edge { start: Point { x: rectangle.max.x, y: rectangle.min.y }, end: Point { x: rectangle.max.x, y: rectangle.max.y } },
            // Bottom edge
            Edge { start: Point { x: rectangle.max.x, y: rectangle.max.y }, end: Point { x: rectangle.min.x, y: rectangle.max.y } },
            // Left edge
            Edge { start: Point { x: rectangle.min.x, y: rectangle.max.y }, end: Point { x: rectangle.min.x, y: rectangle.min.y } },
        ];
        
        // Check if any rectangle edge intersects with any polygon edge
        for rect_edge in &rect_edges {
            for poly_edge in &self.edges {
                if self.edges_intersect(rect_edge, poly_edge) {
                    return false;  // Rectangle crosses polygon boundary
                }
            }
        }

        true
    }
    
    fn edges_intersect(&self, edge1: &Edge, edge2: &Edge) -> bool {
        // Simple line segment intersection check
        let x1 = edge1.start.x as f64;
        let y1 = edge1.start.y as f64;
        let x2 = edge1.end.x as f64;
        let y2 = edge1.end.y as f64;
        
        let x3 = edge2.start.x as f64;
        let y3 = edge2.start.y as f64;
        let x4 = edge2.end.x as f64;
        let y4 = edge2.end.y as f64;
        
        let denom = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
        if denom.abs() < 1e-10 {
            return false; // Parallel lines
        }
        
        let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / denom;
        let u = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / denom;
        
        // Check if intersection point is within both line segments
        t > 0.0 && t < 1.0 && u > 0.0 && u < 1.0
    }

    fn contains_point(&self, point: &Point) -> bool {
        // First check if point lies exactly on any edge
        for edge in &self.edges {
            if self.point_on_edge(point, edge) {
                return true;
            }
        }
        
        // Ray casting algorithm - cast ray to the right and count intersections
        let mut count = 0;
        for edge in &self.edges {
            let y1 = edge.start.y as i64;
            let y2 = edge.end.y as i64;
            let x1 = edge.start.x as i64;
            let x2 = edge.end.x as i64;
            let point_y = point.y as i64;
            let point_x = point.x as i64;
            
            // Skip horizontal edges for ray casting
            if y1 == y2 {
                continue;
            }
            
            // Check if point's y is within edge's y range (excluding upper bound to avoid double counting)
            let min_y = y1.min(y2);
            let max_y = y1.max(y2);
            if point_y < min_y || point_y >= max_y {
                continue;
            }
            
            // Calculate intersection x coordinate using linear interpolation
            let intersect_x = x1 + (x2 - x1) * (point_y - y1) / (y2 - y1);
            
            // Count intersection if it's to the right of the point
            if intersect_x > point_x {
                count += 1;
            }
        }
        count % 2 == 1
    }
    
    fn point_on_edge(&self, point: &Point, edge: &Edge) -> bool {
        let x1 = edge.start.x;
        let y1 = edge.start.y;
        let x2 = edge.end.x;
        let y2 = edge.end.y;
        let px = point.x;
        let py = point.y;
        
        // Check if point is within the bounding box of the edge
        let min_x = x1.min(x2);
        let max_x = x1.max(x2);
        let min_y = y1.min(y2);
        let max_y = y1.max(y2);
        
        if px < min_x || px > max_x || py < min_y || py > max_y {
            return false;
        }
        
        // Check if point lies on the line segment
        if x1 == x2 {
            // Vertical line
            px == x1
        } else if y1 == y2 {
            // Horizontal line
            py == y1
        } else {
            // Use cross product to check collinearity
            // Vector from start to point: (px - x1, py - y1)
            // Vector from start to end: (x2 - x1, y2 - y1)
            // Cross product should be 0 for collinear points
            ((px as i64 - x1 as i64) * (y2 as i64 - y1 as i64)) == ((py as i64 - y1 as i64) * (x2 as i64 - x1 as i64))
        }
    }

    fn from_red_tiles(red_tiles: &Vec<(u32, u32)>) -> Self {
        let mut edges = Vec::new();
        for i in 0..red_tiles.len() {
            let start = red_tiles[i];
            let end = red_tiles[(i + 1) % red_tiles.len()];
            edges.push(Edge {
                start: Point { x: start.0.min(end.0), y: start.1.min(end.1) },
                end: Point { x: end.0.max(start.0), y: end.1.max(start.1) },
            });
        }

        Polygon { edges }
    }
}

fn input() -> &'static str {
    include_str!("../input/day9.txt")
}

fn test_input() -> &'static str {
"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = test_input();
        let result = part1(input);
        assert_eq!(result, 50);
    }

    #[test]
    fn test_part2() {
        let input = test_input();
        let result = part2(input);
        assert_eq!(result, 24);
    }

    #[test]
    fn test_polygon_contains_7_3_and_11_1() {
        let input = test_input();
        let red_tiles = parse_input(input);
        let polygon = Polygon::from_red_tiles(&red_tiles);
        let rectangle = Rectangle {
            min: Point { x: 7, y: 1 },
            max: Point { x: 11, y: 3 },
        };
        assert!(polygon.contains(&rectangle));
        assert_eq!(rectangle.area(), 15);
    }

    #[test]
    fn test_polygon_contains_9_7_and_9_5() {
        let input = test_input();
        let red_tiles = parse_input(input);
        let polygon = Polygon::from_red_tiles(&red_tiles);
        let rectangle = Rectangle {
            min: Point { x: 9, y: 7 },
            max: Point { x: 9, y: 5 },
        };
        assert!(polygon.contains(&rectangle));
    }

    #[test]
    fn test_polygon_contains_2_3_and_9_5() {
        let input = test_input();
        let red_tiles = parse_input(input);
        let polygon = Polygon::from_red_tiles(&red_tiles);
        let rectangle = Rectangle {
            min: Point { x: 2, y: 3 },
            max: Point { x: 9, y: 5 },
        };
        assert!(polygon.contains(&rectangle));
    }

    #[test]
    fn test_polygon_not_contains_2_1_and_11_5() {
        let input = test_input();
        let red_tiles = parse_input(input);
        let polygon = Polygon::from_red_tiles(&red_tiles);
        let rectangle = Rectangle {
            min: Point { x: 2, y: 1 },
            max: Point { x: 11, y: 5 },
        };
        assert!(!polygon.contains(&rectangle));
    }
}