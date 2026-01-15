use std::{
    collections::HashMap,
    fmt::Display,
    io::{Read, stdin},
    os::unix::io::AsRawFd,
};

use getch::Getch;
use strum::{EnumIter, IntoEnumIterator};

type Coord = (i64, i64);

#[derive(Debug, PartialEq)]
pub struct Robot {
    position: Coord,
    vector: Coord,
}

pub struct Floorplan {
    robots: Vec<Robot>,
    occupied: HashMap<Coord, u64>,
    rows: i64,
    cols: i64,
}

impl Floorplan {
    fn new(robots: Vec<Robot>, rows: i64, cols: i64) -> Self {
        let mut occupied = HashMap::new();
        for robot in robots.iter() {
            *occupied.entry(robot.position).or_insert(0) += 1;
        }
        Floorplan {
            robots,
            occupied,
            rows,
            cols,
        }
    }

    pub fn move_robots(&mut self, n: i64) {
        for robot in self.robots.iter_mut() {
            let new_x = robot.vector.0 * n + robot.position.0;
            let rem = new_x % self.cols;
            let nx = if rem < 0 { self.cols + rem } else { rem };
            let new_y = robot.vector.1 * n + robot.position.1;
            let rem = new_y % self.rows;
            let ny = if rem < 0 { self.rows + rem } else { rem };
            let new_pos = (nx, ny);
            if let Some(qty) = self.occupied.get_mut(&robot.position) {
                if *qty > 1 {
                    *qty -= 1;
                } else {
                    self.occupied.remove(&robot.position);
                }
            }
            *self.occupied.entry(new_pos).or_insert(0) += 1;
            robot.position = new_pos;
        }
    }

    fn count_quadrant(&self, quadrant: Quadrant) -> u64 {
        let ((min_x, min_y), (max_x, max_y)) = quadrant.get_limits(self.rows, self.cols);
        self.occupied
            .iter()
            .filter_map(|(&(x, y), &qty)| {
                if x >= min_x && x < max_x && y >= min_y && y < max_y {
                    Some(qty)
                } else {
                    None
                }
            })
            .sum()
    }

    fn clustering_score(&self) -> f64 {
        if self.occupied.is_empty() {
            return 0.0;
        }

        // Compute weighted centroid
        let (sum_x, sum_y, total_count) =
            self.occupied
                .iter()
                .fold((0.0, 0.0, 0.0), |(sx, sy, sc), (&(x, y), &count)| {
                    (
                        sx + x as f64 * count as f64,
                        sy + y as f64 * count as f64,
                        sc + count as f64,
                    )
                });
        let centroid_x = sum_x / total_count;
        let centroid_y = sum_y / total_count;

        // Compare weighted sum of squared distances to centroid
        self.occupied
            .iter()
            .map(|(&(x, y), &count)| {
                let dx = x as f64 - centroid_x;
                let dy = y as f64 - centroid_y;
                (dx * dx + dy * dy) * count as f64
            })
            .sum()
    }
}

impl Display for Floorplan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output: Vec<Vec<String>> =
            vec![vec![".".to_string(); self.cols as usize]; self.rows as usize];
        for (&(x, y), qty) in self.occupied.iter() {
            output[y as usize][x as usize] = qty.to_string();
        }
        let result = output
            .iter()
            .map(|row| row.join(""))
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{result}")
    }
}

#[derive(EnumIter)]
enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Quadrant {
    fn get_limits(&self, rows: i64, cols: i64) -> (Coord, Coord) {
        let mid_row = rows / 2;
        let mid_col = cols / 2;
        match self {
            Quadrant::TopLeft => ((0, 0), (mid_col, mid_row)),
            Quadrant::TopRight => ((mid_col + 1, 0), (cols, mid_row)),
            Quadrant::BottomLeft => ((0, mid_row + 1), (mid_col, rows)),
            Quadrant::BottomRight => ((mid_col + 1, mid_row + 1), (cols, rows)),
        }
    }
}

pub fn parse_day14(lines: &[String], size: Coord) -> Floorplan {
    let mut robots = Vec::new();
    for line in lines {
        let (p, v) = line.split_once(' ').expect("missing space in line");
        let (px_str, py_str) = p.split_once(',').expect("missing comma in position");
        let px = px_str
            .trim_start_matches("p=")
            .parse::<i64>()
            .expect("not an integer (px)");
        let py = py_str.parse::<i64>().expect("not an integer (py)");
        let (vx_str, vy_str) = v.split_once(',').expect("missing comma in vector");
        let vx = vx_str
            .trim_start_matches("v=")
            .parse::<i64>()
            .expect("not an integer (vx)");
        let vy = vy_str.parse::<i64>().expect("not an integer (vy)");
        robots.push(Robot {
            position: (px, py),
            vector: (vx, vy),
        });
    }

    Floorplan::new(robots, size.1, size.0)
}

pub fn get_result_day14_stage1(floorplan: &mut Floorplan, n: i64) -> u64 {
    floorplan.move_robots(n);

    Quadrant::iter()
        .map(|quadrant| floorplan.count_quadrant(quadrant))
        .product()
}

pub fn get_result_day14_stage2(floorplan: &mut Floorplan) {
    let mut best_score: f64 = f64::MAX;

    for t in 1..=100_000 {
        floorplan.move_robots(1);
        let score = floorplan.clustering_score();

        if score < best_score {
            best_score = score;

            println!("Iteration: {t}");
            println!("Clustering Score: {best_score}");
            println!("{floorplan}");

            println!("Press 'q' to quit, any other key to continue...");
            let c = wait_for_key();
            if c == 'q' {
                return; // Exit the function if 'q' is pressed
            }
        }
    }
}

fn wait_for_key() -> char {
    getch::Getch::new().getch().unwrap() as char
}
#[cfg(test)]
mod day14 {
    use super::*;

    fn get_lines() -> Vec<String> {
        vec![
            String::from("p=0,4 v=3,-3"),
            String::from("p=6,3 v=-1,-3"),
            String::from("p=10,3 v=-1,2"),
            String::from("p=2,0 v=2,-1"),
            String::from("p=0,0 v=1,3"),
            String::from("p=3,0 v=-2,-2"),
            String::from("p=7,6 v=-1,-3"),
            String::from("p=3,0 v=-1,-2"),
            String::from("p=9,3 v=2,3"),
            String::from("p=7,3 v=-1,2"),
            String::from("p=2,4 v=2,-3"),
            String::from("p=9,5 v=-3,-3"),
        ]
    }

    #[test]
    fn parse() {
        let floorplan = parse_day14(&get_lines(), (11, 7));
        let expected_robots = [
            Robot {
                position: (0, 4),
                vector: (3, -3),
            },
            Robot {
                position: (6, 3),
                vector: (-1, -3),
            },
            Robot {
                position: (10, 3),
                vector: (-1, 2),
            },
            Robot {
                position: (2, 0),
                vector: (2, -1),
            },
            Robot {
                position: (0, 0),
                vector: (1, 3),
            },
            Robot {
                position: (3, 0),
                vector: (-2, -2),
            },
            Robot {
                position: (7, 6),
                vector: (-1, -3),
            },
            Robot {
                position: (3, 0),
                vector: (-1, -2),
            },
            Robot {
                position: (9, 3),
                vector: (2, 3),
            },
            Robot {
                position: (7, 3),
                vector: (-1, 2),
            },
            Robot {
                position: (2, 4),
                vector: (2, -3),
            },
            Robot {
                position: (9, 5),
                vector: (-3, -3),
            },
        ];
        assert_eq!(floorplan.robots.len(), expected_robots.len());
        for (res, exp) in floorplan.robots.iter().zip(expected_robots.iter()) {
            assert_eq!(res, exp);
        }
        assert_eq!(floorplan.rows, 7);
        assert_eq!(floorplan.cols, 11);
        let expected_occupied: HashMap<Coord, u64> = HashMap::from_iter([
            ((0, 4), 1),
            ((6, 3), 1),
            ((10, 3), 1),
            ((2, 0), 1),
            ((0, 0), 1),
            ((3, 0), 2),
            ((7, 6), 1),
            ((9, 3), 1),
            ((7, 3), 1),
            ((2, 4), 1),
            ((9, 5), 1),
        ]);
        assert_eq!(floorplan.occupied, expected_occupied);
    }

    #[test]
    fn after_100_moves() {
        let mut floorplan = parse_day14(&get_lines(), (11, 7));
        floorplan.move_robots(100);
        let expected_occupied: HashMap<Coord, u64> = HashMap::from_iter([
            ((0, 2), 1),
            ((1, 3), 1),
            ((1, 6), 1),
            ((2, 3), 1),
            ((3, 5), 1),
            ((4, 5), 2),
            ((5, 4), 1),
            ((6, 0), 2),
            ((6, 6), 1),
            ((9, 0), 1),
        ]);
        assert_eq!(floorplan.occupied, expected_occupied);
    }

    #[test]
    fn stage1() {
        let mut floorplan = parse_day14(&get_lines(), (11, 7));
        let result = get_result_day14_stage1(&mut floorplan, 100);
        assert_eq!(result, 12);
    }
}
