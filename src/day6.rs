use std::collections::HashSet;

#[derive(Clone)]
pub struct Map {
    obstructions: HashSet<Position>,
    position: Position,
    direction: Direction,
    rows: isize,
    cols: isize,
    visited: HashSet<Position>,
}

impl Map {
    pub fn patrol(&mut self) -> usize {
        while !self.will_leave() {
            let next_pos = self.get_next_position();

            if self.obstructions.contains(&next_pos) {
                self.direction = self.direction.turn();
                continue;
            }
            self.position = next_pos;
            self.visited.insert(next_pos);
        }

        self.visited.len()
    }

    pub fn count_loop_positions(&self) -> usize {
        let mut m = self.clone();
        m.patrol();
        let path = m.visited;

        let mut count = 0;
        for &p in path.iter() {
            if p == self.position || self.obstructions.contains(&p) {
                continue;
            }

            if is_infinite_loop(
                self.rows,
                self.cols,
                &self.obstructions,
                self.position,
                self.direction,
                p,
            ) {
                count += 1;
            }
        }
        count
    }

    fn will_leave(&self) -> bool {
        let nr = self.position.0 as isize + self.direction.offset().0;
        let nc = self.position.1 as isize + self.direction.offset().1;
        nr < 0 || nc < 0 || nr >= self.rows || nc >= self.cols
    }

    fn get_next_position(&self) -> Position {
        let (dr, dc) = self.direction.offset();
        let nr = self.position.0 as isize + dr;
        let nc = self.position.1 as isize + dc;

        assert!(
            nr >= 0 && nc >= 0 && nr < self.rows && nc < self.cols,
            "get_next_position() called when next step leaves the map"
        );

        (nr as usize, nc as usize)
    }
}

type Position = (usize, usize);

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    const fn offset(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    const fn turn(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

pub fn parse_day6(lines: &[String]) -> Map {
    let mut obstructions = HashSet::new();
    let mut position = (0usize, 0usize);
    let mut direction = Direction::Right;
    let rows = lines.len() as isize;
    let cols = lines[0].len() as isize;
    for (row, line) in lines.iter().enumerate() {
        for (col, ch) in line.char_indices() {
            match ch {
                '.' => continue,
                '#' => {
                    obstructions.insert((row, col));
                }
                '^' => {
                    position = (row, col);
                    direction = Direction::Up;
                }
                'v' => {
                    position = (row, col);
                    direction = Direction::Down;
                }
                '>' => {
                    position = (row, col);
                    direction = Direction::Right;
                }
                '<' => {
                    position = (row, col);
                    direction = Direction::Left;
                }
                _ => panic!("unknown character at ({row}, {col}): {ch}"),
            }
        }
    }

    let mut visited = HashSet::new();
    visited.insert(position);

    Map {
        rows,
        cols,
        obstructions,
        position,
        direction,
        visited,
    }
}

fn is_infinite_loop(
    rows: isize,
    cols: isize,
    obstructions: &HashSet<Position>,
    start: Position,
    start_dir: Direction,
    extra_block: Position,
) -> bool {
    let mut seen = vec![vec![[false; 4]; cols as usize]; rows as usize];
    let mut r = start.0 as isize;
    let mut c = start.1 as isize;
    let mut dir = start_dir;

    // Treat the extra bklock as an obstruction too
    let mut blocks = obstructions.clone();
    blocks.insert(extra_block);

    loop {
        let dir_idx = match dir {
            Direction::Up => 0,
            Direction::Right => 1,
            Direction::Down => 2,
            Direction::Left => 3,
        };

        if seen[r as usize][c as usize][dir_idx] {
            return true; // loop found
        }
        seen[r as usize][c as usize][dir_idx] = true;

        let (dr, dc) = dir.offset();
        let nr = r + dr;
        let nc = c + dc;

        if nr < 0 || nc < 0 || nr >= rows || nc >= cols {
            return false; // left the map, no loop
        }

        if blocks.contains(&(nr as usize, nc as usize)) {
            dir = dir.turn(); // rotate on obstruction
        } else {
            r = nr;
            c = nc;
        }
    }
    false // shouldn't get here!
}

#[cfg(test)]
mod day6 {
    use super::*;

    fn get_input_lines() -> Vec<String> {
        vec![
            String::from("....#....."),
            String::from(".........#"),
            String::from(".........."),
            String::from("..#......."),
            String::from(".......#.."),
            String::from(".........."),
            String::from(".#..^....."),
            String::from("........#."),
            String::from("#........."),
            String::from("......#..."),
        ]
    }

    #[test]
    fn day6_parse() {
        let map = parse_day6(&get_input_lines());
        assert_eq!(map.direction, Direction::Up);
        assert_eq!(map.position, (6, 4));
        assert_eq!(map.rows, 10);
        assert_eq!(map.cols, 10);
        let mut expected_visited = HashSet::new();
        expected_visited.insert((6, 4));
        assert_eq!(map.visited, expected_visited);
        let mut expected_obstructions = HashSet::new();
        expected_obstructions.insert((0, 4));
        expected_obstructions.insert((1, 9));
        expected_obstructions.insert((3, 2));
        expected_obstructions.insert((4, 7));
        expected_obstructions.insert((6, 1));
        expected_obstructions.insert((7, 8));
        expected_obstructions.insert((8, 0));
        expected_obstructions.insert((9, 6));
        assert_eq!(map.obstructions, expected_obstructions);
    }

    #[test]
    fn day6_stage1() {
        let mut map = parse_day6(&get_input_lines());
        let result = map.patrol();
        assert_eq!(result, 41);
    }

    #[test]
    fn day6_stage2() {
        let map = parse_day6(&get_input_lines());
        let result = map.count_loop_positions();
        assert_eq!(result, 6);
    }
}
