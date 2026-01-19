use std::{collections::HashSet, fmt::Display};

type Coord = (u32, u32);

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn get_change(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "^"),
            Direction::Down => write!(f, "v"),
            Direction::Left => write!(f, "<"),
            Direction::Right => write!(f, ">"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct WarehouseBox {
    left: Coord,
    right: Coord,
}

impl WarehouseBox {
    fn get_coords(&self) -> (Coord, Coord) {
        (self.left, self.right)
    }

    fn leading_edge_tiles(&self, dir: &Direction) -> Vec<Coord> {
        match dir {
            Direction::Right => {
                let row = self.left.0;
                let col = self.left.1.max(self.right.1) + 1;
                vec![(row, col)]
            }
            Direction::Left => {
                let row = self.left.0;
                let col = self.left.1.min(self.right.1).saturating_sub(1);
                vec![(row, col)]
            }
            Direction::Down => {
                let row = self.left.0.max(self.right.0) + 1;
                vec![(row, self.left.1), (row, self.right.1)]
            }
            Direction::Up => {
                let row = self.left.0.min(self.right.0).saturating_sub(1);
                vec![(row, self.left.1), (row, self.right.1)]
            }
        }
    }

    fn shifted(&self, dir: &Direction) -> WarehouseBox {
        let (dr, dc) = dir.get_change();
        let shift = |(r, c): Coord| ((r as i32 + dr) as u32, (c as i32 + dc) as u32);

        WarehouseBox {
            left: shift(self.left),
            right: shift(self.right),
        }
    }
}

impl Display for WarehouseBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.left == self.right {
            write!(f, "O")
        } else {
            write!(f, "[]")
        }
    }
}

pub struct Warehouse {
    robot: Coord,
    walls: HashSet<Coord>,
    boxes: HashSet<WarehouseBox>,
    rows: u32,
    cols: u32,
    moves: Vec<Direction>,
}

impl Warehouse {
    pub fn make_moves(&mut self, debug: bool) {
        'next_move: for direction in self.moves.iter() {
            if debug {
                println!("Start of move {direction}:\n{self}");
                let box_tiles = self
                    .boxes
                    .iter()
                    .map(|b| {
                        format!(
                            "(({}, {}), ({}, {}))",
                            b.left.0, b.left.1, b.right.0, b.right.1
                        )
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                println!("Boxes: {box_tiles}");
            }
            let (dr, dc) = direction.get_change();
            let nr = self.robot.0 as i32 + dr;
            let nc = self.robot.1 as i32 + dc;
            if nr < 0 || nc < 0 || nr >= self.rows as i32 || nc >= self.cols as i32 {
                continue;
            }
            let new_robot_pos = (nr as u32, nc as u32);
            if self.walls.contains(&new_robot_pos) {
                continue;
            }
            if self.box_at(new_robot_pos).is_some() {
                let mut to_move: Vec<WarehouseBox> = Vec::new();
                let mut frontier: HashSet<Coord> = HashSet::from([new_robot_pos]);

                loop {
                    let mut next_frontier: HashSet<Coord> = HashSet::new();
                    let mut found_box = false;

                    for &tile in frontier.iter() {
                        if let Some(b) = self.box_at(tile) {
                            if !to_move.contains(&b) {
                                to_move.push(b.clone());
                            }
                            found_box = true;

                            let next_tiles = b.leading_edge_tiles(direction);

                            for &(r, c) in next_tiles.iter() {
                                if r >= self.rows || c >= self.cols {
                                    continue 'next_move;
                                }
                                if self.walls.contains(&(r, c)) {
                                    continue 'next_move;
                                }
                                next_frontier.insert((r, c));
                            }
                        } else {
                            // empty tile - still part of frontier, but nothing to add
                            next_frontier.insert(tile);
                        }
                    }
                    if !found_box {
                        break; // entire frontier is empty - valid push
                    }

                    frontier = next_frontier;
                }

                if debug {
                    println!(
                        "Movin boxes: {}",
                        to_move
                            .iter()
                            .map(|b| format!("({:?}, {:?})", b.left, b.right))
                            .collect::<Vec<_>>()
                            .join(", ")
                    );
                }

                // Comit moves in reverse
                for b in to_move.iter().rev() {
                    self.boxes.remove(b);
                    self.boxes.insert(b.shifted(direction));
                }

                self.robot = new_robot_pos;
            } else {
                // No boxes or walls, so the robot just moves
                self.robot = new_robot_pos;
            }
        }
    }

    pub fn get_gps_sum(&self) -> u64 {
        self.boxes
            .iter()
            .map(|b| b.left.0 as u64 * 100 + b.left.1 as u64)
            .sum()
    }

    pub fn make_wider(&mut self) {
        let mut walls = HashSet::new();
        for &(r, c) in self.walls.iter() {
            walls.insert((r, c * 2));
            walls.insert((r, c * 2 + 1));
        }
        let mut boxes = HashSet::new();
        for b in self.boxes.iter() {
            let ((left_r, left_c), (right_r, right_c)) = b.get_coords();
            let new_box = WarehouseBox {
                left: (left_r, left_c * 2),
                right: (right_r, right_c * 2 + 1),
            };
            boxes.insert(new_box);
        }
        self.robot.1 *= 2;
        self.walls = walls;
        self.boxes = boxes;
        self.cols *= 2;
    }

    fn box_at(&self, coord: Coord) -> Option<WarehouseBox> {
        self.boxes
            .iter()
            .find(|b| b.left == coord || b.right == coord)
            .cloned()
    }
}

impl Display for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = vec![vec![".".to_string(); self.cols as usize]; self.rows as usize];
        output[self.robot.0 as usize][self.robot.1 as usize] = "@".to_string();
        for &(wall_row, wall_col) in self.walls.iter() {
            output[wall_row as usize][wall_col as usize] = "#".to_string();
        }
        for b in self.boxes.iter() {
            let box_string = b.to_string();
            if box_string.len() == 1 {
                output[b.left.0 as usize][b.left.1 as usize] = box_string;
            } else {
                for (i, ch) in box_string.char_indices() {
                    output[b.left.0 as usize][b.left.1 as usize + i] = ch.to_string();
                }
            }
        }
        let result = output
            .iter()
            .map(|row| row.join(""))
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{result}")
    }
}

fn offset((r, c): Coord, (dr, dc): (i32, i32)) -> Option<Coord> {
    let nr = r as i32 + dr;
    let nc = c as i32 + dc;
    if nr < 0 || nc < 0 {
        None
    } else {
        Some((nr as u32, nc as u32))
    }
}

pub fn parse_day15(lines: &[String]) -> Warehouse {
    let mut robot: Option<Coord> = None;
    let mut walls: HashSet<Coord> = HashSet::new();
    let mut small_boxes: HashSet<Coord> = HashSet::new();
    let mut moves = Vec::new();

    let mut row = 0;
    for l in lines.iter() {
        if l.is_empty() {
            continue;
        }
        for (col, ch) in l.char_indices() {
            match ch {
                '#' => _ = walls.insert((row, col as u32)),
                'O' => _ = small_boxes.insert((row, col as u32)),
                '@' => robot = Some((row, col as u32)),
                '.' => continue,
                '<' => moves.push(Direction::Left),
                '>' => moves.push(Direction::Right),
                '^' => moves.push(Direction::Up),
                'v' => moves.push(Direction::Down),
                _ => panic!("Unknown character: {ch}"),
            }
        }
        row += 1;
    }

    let rows = walls.iter().map(|&(r, _)| r).max().unwrap() + 1;
    let cols = walls.iter().map(|&(_, c)| c).max().unwrap() + 1;

    // Convert small boxes to WarehouseBox, using left and right as the same values
    let boxes: Vec<WarehouseBox> = small_boxes
        .iter()
        .map(|&coord| WarehouseBox {
            left: coord,
            right: coord,
        })
        .collect();

    Warehouse {
        robot: robot.unwrap(),
        walls,
        boxes: HashSet::from_iter(boxes),
        rows,
        cols,
        moves,
    }
}

#[cfg(test)]
mod day15 {
    use super::*;

    fn get_large_example() -> Vec<String> {
        vec![
            String::from("##########"),
            String::from("#..O..O.O#"),
            String::from("#......O.#"),
            String::from("#.OO..O.O#"),
            String::from("#..O@..O.#"),
            String::from("#O#..O...#"),
            String::from("#O..O..O.#"),
            String::from("#.OO.O.OO#"),
            String::from("#....O...#"),
            String::from("##########"),
            String::from(""),
            String::from("<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^"),
            String::from("vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v"),
            String::from("><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<"),
            String::from("<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^"),
            String::from("^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><"),
            String::from("^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^"),
            String::from(">^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^"),
            String::from("<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>"),
            String::from("^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>"),
            String::from("v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"),
        ]
    }

    fn get_small_example() -> Vec<String> {
        vec![
            String::from("########"),
            String::from("#..O.O.#"),
            String::from("##@.O..#"),
            String::from("#...O..#"),
            String::from("#.#.O..#"),
            String::from("#...O..#"),
            String::from("#......#"),
            String::from("########"),
            String::from(""),
            String::from("<^^>>>vv<v>>v<<"),
        ]
    }

    #[test]
    fn parse_small() {
        use super::Direction::*;
        let warehouse = parse_day15(&get_small_example());
        let walls: HashSet<Coord> = HashSet::from_iter([
            (0, 0),
            (0, 1),
            (0, 2),
            (0, 3),
            (0, 4),
            (0, 5),
            (0, 6),
            (0, 7),
            (7, 0),
            (7, 1),
            (7, 2),
            (7, 3),
            (7, 4),
            (7, 5),
            (7, 6),
            (7, 7),
            (1, 0),
            (2, 0),
            (3, 0),
            (4, 0),
            (5, 0),
            (6, 0),
            (1, 7),
            (2, 7),
            (3, 7),
            (4, 7),
            (5, 7),
            (6, 7),
            (2, 1),
            (4, 2),
        ]);
        assert_eq!(warehouse.walls.len(), walls.len());
        assert_eq!(warehouse.walls, walls);
        assert_eq!(warehouse.robot, (2, 2));
        assert_eq!(warehouse.rows, 8);
        assert_eq!(warehouse.cols, 8);
        let boxes: HashSet<Coord> =
            HashSet::from_iter([(1, 3), (1, 5), (2, 4), (3, 4), (4, 4), (5, 4)]);
        assert_eq!(warehouse.boxes.len(), boxes.len());
        let result_boxes: HashSet<Coord> = warehouse.boxes.iter().map(|b| b.left).collect();
        assert_eq!(result_boxes, boxes);
        let moves = vec![
            Left, Up, Up, Right, Right, Right, Down, Down, Left, Down, Right, Right, Down, Left,
            Left,
        ];
        assert_eq!(warehouse.moves.len(), moves.len());
        assert_eq!(warehouse.moves, moves);
    }

    #[test]
    fn all_moves_small() {
        let mut warehouse = parse_day15(&get_small_example());
        warehouse.make_moves(false);
        assert_eq!(warehouse.robot, (4, 4));
        let boxes: HashSet<Coord> =
            HashSet::from_iter([(1, 5), (1, 6), (3, 6), (4, 3), (5, 4), (6, 4)]);
        assert_eq!(warehouse.boxes.len(), boxes.len());
        let result_boxes: HashSet<Coord> = warehouse.boxes.iter().map(|b| b.left).collect();
        assert_eq!(result_boxes, boxes);
    }

    #[test]
    fn stage1_small() {
        let mut warehouse = parse_day15(&get_small_example());
        warehouse.make_moves(false);
        let gps = warehouse.get_gps_sum();
        assert_eq!(gps, 2028);
    }

    #[test]
    fn stage1_large() {
        let mut warehouse = parse_day15(&get_large_example());
        warehouse.make_moves(false);
        let gps = warehouse.get_gps_sum();
        assert_eq!(gps, 10092);
    }

    #[test]
    fn wider_large() {
        let mut warehouse = parse_day15(&get_large_example());
        warehouse.make_wider();
        // Is robot in the right place?
        assert_eq!(warehouse.robot, (4, 8));
        // Check warehouse dimensions
        assert_eq!(warehouse.rows, 10);
        assert_eq!(warehouse.cols, 20);
        // Check walls
        let mut expected_walls = HashSet::new();
        // Top and bottom
        for i in 0..20 {
            expected_walls.insert((0, i));
            expected_walls.insert((9, i));
        }
        // Left and right
        for i in 0..10 {
            expected_walls.insert((i, 0));
            expected_walls.insert((i, 1));
            expected_walls.insert((i, 18));
            expected_walls.insert((i, 19));
        }
        // Extra 2 walls
        expected_walls.insert((5, 4));
        expected_walls.insert((5, 5));
        assert_eq!(warehouse.walls, expected_walls);
        // Check boxes
        let mut expected_boxes = HashSet::new();
        expected_boxes.insert(WarehouseBox {
            left: (1, 6),
            right: (1, 7),
        });
        expected_boxes.insert(WarehouseBox {
            left: (1, 12),
            right: (1, 13),
        });
        expected_boxes.insert(WarehouseBox {
            left: (1, 16),
            right: (1, 17),
        });
        expected_boxes.insert(WarehouseBox {
            left: (2, 14),
            right: (2, 15),
        });
        expected_boxes.insert(WarehouseBox {
            left: (3, 4),
            right: (3, 5),
        });
        expected_boxes.insert(WarehouseBox {
            left: (3, 6),
            right: (3, 7),
        });
        expected_boxes.insert(WarehouseBox {
            left: (3, 12),
            right: (3, 13),
        });
        expected_boxes.insert(WarehouseBox {
            left: (3, 16),
            right: (3, 17),
        });
        expected_boxes.insert(WarehouseBox {
            left: (4, 6),
            right: (4, 7),
        });
        expected_boxes.insert(WarehouseBox {
            left: (4, 14),
            right: (4, 15),
        });
        expected_boxes.insert(WarehouseBox {
            left: (5, 2),
            right: (5, 3),
        });
        expected_boxes.insert(WarehouseBox {
            left: (5, 10),
            right: (5, 11),
        });
        expected_boxes.insert(WarehouseBox {
            left: (6, 2),
            right: (6, 3),
        });
        expected_boxes.insert(WarehouseBox {
            left: (6, 8),
            right: (6, 9),
        });
        expected_boxes.insert(WarehouseBox {
            left: (6, 14),
            right: (6, 15),
        });
        expected_boxes.insert(WarehouseBox {
            left: (7, 4),
            right: (7, 5),
        });
        expected_boxes.insert(WarehouseBox {
            left: (7, 6),
            right: (7, 7),
        });
        expected_boxes.insert(WarehouseBox {
            left: (7, 10),
            right: (7, 11),
        });
        expected_boxes.insert(WarehouseBox {
            left: (7, 14),
            right: (7, 15),
        });
        expected_boxes.insert(WarehouseBox {
            left: (7, 16),
            right: (7, 17),
        });
        expected_boxes.insert(WarehouseBox {
            left: (8, 10),
            right: (8, 11),
        });
        assert_eq!(warehouse.boxes, expected_boxes);
    }

    #[test]
    fn stage2_large() {
        let mut warehouse = parse_day15(&get_large_example());
        warehouse.make_wider();
        warehouse.make_moves(false);
        /*
        ####################
        ##[].......[].[][]##
        ##[]...........[].##
        ##[]........[][][]##
        ##[]......[]....[]##
        ##..##......[]....##
        ##..[]............##
        ##..@......[].[][]##
        ##......[][]..[]..##
        ####################
        */
        println!("{warehouse}");
        assert_eq!(warehouse.robot, (7, 4));
        let mut expected_boxes = HashSet::new();
        expected_boxes.insert(WarehouseBox {
            left: (1, 2),
            right: (1, 3),
        });
        expected_boxes.insert(WarehouseBox {
            left: (1, 11),
            right: (1, 12),
        });
        expected_boxes.insert(WarehouseBox {
            left: (1, 14),
            right: (1, 15),
        });
        expected_boxes.insert(WarehouseBox {
            left: (1, 16),
            right: (1, 17),
        });
        expected_boxes.insert(WarehouseBox {
            left: (2, 2),
            right: (2, 3),
        });
        expected_boxes.insert(WarehouseBox {
            left: (2, 15),
            right: (2, 16),
        });
        expected_boxes.insert(WarehouseBox {
            left: (3, 2),
            right: (3, 3),
        });
        expected_boxes.insert(WarehouseBox {
            left: (3, 12),
            right: (3, 13),
        });
        expected_boxes.insert(WarehouseBox {
            left: (3, 14),
            right: (3, 15),
        });
        expected_boxes.insert(WarehouseBox {
            left: (3, 16),
            right: (3, 17),
        });
        expected_boxes.insert(WarehouseBox {
            left: (4, 2),
            right: (4, 3),
        });
        expected_boxes.insert(WarehouseBox {
            left: (4, 10),
            right: (4, 11),
        });
        expected_boxes.insert(WarehouseBox {
            left: (4, 16),
            right: (4, 17),
        });
        expected_boxes.insert(WarehouseBox {
            left: (5, 12),
            right: (5, 13),
        });
        expected_boxes.insert(WarehouseBox {
            left: (6, 4),
            right: (6, 5),
        });
        expected_boxes.insert(WarehouseBox {
            left: (7, 11),
            right: (7, 12),
        });
        expected_boxes.insert(WarehouseBox {
            left: (7, 14),
            right: (7, 15),
        });
        expected_boxes.insert(WarehouseBox {
            left: (7, 16),
            right: (7, 17),
        });
        expected_boxes.insert(WarehouseBox {
            left: (8, 8),
            right: (8, 9),
        });
        expected_boxes.insert(WarehouseBox {
            left: (8, 10),
            right: (8, 11),
        });
        expected_boxes.insert(WarehouseBox {
            left: (8, 14),
            right: (8, 15),
        });
        assert_eq!(warehouse.boxes, expected_boxes);
        let gps = warehouse.get_gps_sum();
        assert_eq!(gps, 9021);
    }

    #[test]
    fn stage2_small() {
        let lines = vec![
            String::from("#######"),
            String::from("#...#.#"),
            String::from("#.....#"),
            String::from("#..OO@#"),
            String::from("#..O..#"),
            String::from("#.....#"),
            String::from("#######"),
            String::from(""),
            String::from("<vv<<^^<<^^"),
        ];
        let mut warehouse = parse_day15(&lines);
        warehouse.make_wider();
        warehouse.make_moves(false);
        /*
        ##############
        ##...[].##..##
        ##...@.[]...##
        ##....[]....##
        ##..........##
        ##..........##
        ##############
        */
        assert_eq!(warehouse.robot, (2, 5));
        let mut expected_boxes = HashSet::new();
        expected_boxes.insert(WarehouseBox {
            left: (1, 5),
            right: (1, 6),
        });
        expected_boxes.insert(WarehouseBox {
            left: (2, 7),
            right: (2, 8),
        });
        expected_boxes.insert(WarehouseBox {
            left: (3, 6),
            right: (3, 7),
        });
        assert_eq!(warehouse.boxes, expected_boxes);
        assert_eq!(warehouse.get_gps_sum(), 618);
    }
}
