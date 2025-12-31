use std::collections::HashSet;

pub struct Map {
    obstructions: Vec<Position>,
    position: Position,
    direction: Direction,
    rows: isize,
    cols: isize,
    visited: HashSet<Position>,
}

impl Map {
    pub fn patrol(&mut self) -> usize {
        while !self.will_leave() {
            let nr = (self.position.0 as isize + self.direction.offset().0) as usize;
            let nc = (self.position.1 as isize + self.direction.offset().1) as usize;

            if self.obstructions.contains(&(nr, nc)) {
                self.direction = self.direction.turn();
                continue;
            }
            self.position = (nr, nc);
            self.visited.insert((nr, nc));
        }

        self.visited.len()
    }

    fn will_leave(&self) -> bool {
        let nr = self.position.0 as isize + self.direction.offset().0;
        let nc = self.position.1 as isize + self.direction.offset().1;
        nr < 0 || nc < 0 || nr >= self.rows || nc >= self.cols
    }
}

type Position = (usize, usize);

#[derive(Debug, PartialEq)]
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
    let mut obstructions = Vec::new();
    let mut position = (0usize, 0usize);
    let mut direction = Direction::Right;
    let rows = lines.len() as isize;
    let cols = lines[0].len() as isize;
    for (row, line) in lines.iter().enumerate() {
        for (col, ch) in line.char_indices() {
            match ch {
                '.' => continue,
                '#' => obstructions.push((row, col)),
                '^' => {
                    position = (row, col);
                    direction = Direction::Up;
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
        let expected_obstructions = vec![
            (0, 4),
            (1, 9),
            (3, 2),
            (4, 7),
            (6, 1),
            (7, 8),
            (8, 0),
            (9, 6),
        ];
        assert_eq!(map.obstructions, expected_obstructions);
    }

    #[test]
    fn day6_stage1() {
        let mut map = parse_day6(&get_input_lines());
        let result = map.patrol();
        assert_eq!(result, 41);
    }
}
