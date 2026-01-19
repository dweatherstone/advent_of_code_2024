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

pub struct Warehouse {
    robot: Coord,
    walls: HashSet<Coord>,
    boxes: HashSet<Coord>,
    rows: u32,
    cols: u32,
    moves: Vec<Direction>,
}

impl Warehouse {
    pub fn make_moves(&mut self, debug: bool) {
        'next_move: for direction in self.moves.iter() {
            if debug {
                println!("Start of move {direction}:\n{self}");
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
            if self.boxes.contains(&new_robot_pos) {
                // Find the end of the box chain
                let mut chain_end = new_robot_pos;

                loop {
                    match offset(chain_end, (dr, dc)) {
                        None => continue 'next_move, // out of bounds
                        Some(next) => {
                            if next.0 >= self.rows || next.1 >= self.cols {
                                continue 'next_move;
                            }
                            if self.walls.contains(&next) {
                                continue 'next_move;
                            }
                            if self.boxes.contains(&next) {
                                chain_end = next;
                            } else {
                                // Found empty space - can push!
                                break;
                            }
                        }
                    }
                }

                // Push all boxes in the chain forward
                // Start from the end and work backwards so we don't overwrite positions
                let mut current_pos = chain_end;
                loop {
                    // Move box at current_pos to the next position
                    self.boxes.remove(&current_pos);
                    let next_pos = offset(current_pos, (dr, dc)).unwrap();
                    self.boxes.insert(next_pos);

                    // If we've reached the robot's target, stop
                    if current_pos == new_robot_pos {
                        break;
                    }

                    // Move to the previous box in the chain
                    if let Some(prev_pos) = offset(current_pos, (-dr, -dc)) {
                        current_pos = prev_pos;
                    } else {
                        break;
                    }
                }

                // Move robot to the empty space where the first box was
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
            .map(|&(r, c)| r as u64 * 100 + c as u64)
            .sum()
    }
}

impl Display for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = vec![vec![".".to_string(); self.cols as usize]; self.rows as usize];
        output[self.robot.0 as usize][self.robot.1 as usize] = "@".to_string();
        for &(wall_row, wall_col) in self.walls.iter() {
            output[wall_row as usize][wall_col as usize] = "#".to_string();
        }
        for &(box_row, box_col) in self.boxes.iter() {
            output[box_row as usize][box_col as usize] = "O".to_string();
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
    let mut boxes: HashSet<Coord> = HashSet::new();
    let mut moves = Vec::new();

    let mut row = 0;
    for l in lines.iter() {
        if l.is_empty() {
            continue;
        }
        for (col, ch) in l.char_indices() {
            match ch {
                '#' => _ = walls.insert((row, col as u32)),
                'O' => _ = boxes.insert((row, col as u32)),
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

    Warehouse {
        robot: robot.unwrap(),
        walls,
        boxes,
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
        assert_eq!(warehouse.boxes, boxes);
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
        assert_eq!(warehouse.boxes, boxes);
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
}
