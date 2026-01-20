use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap},
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn turn_left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::West => Direction::South,
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }

    fn delta(&self) -> (isize, isize) {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct State {
    row: usize,
    col: usize,
    direction: Direction,
    cost: u64,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost
            .cmp(&other.cost)
            // Reverse tie-breakers so heap ordering stays stable
            .then_with(|| self.row.cmp(&other.row))
            .then_with(|| self.col.cmp(&other.col))
            .then_with(|| self.direction.cmp(&other.direction))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Grid {
    walls: Vec<Vec<bool>>,
    height: usize,
    width: usize,
    start: (usize, usize),
    end: (usize, usize),
}

pub fn parse_day16(lines: &[String]) -> Grid {
    let height = lines.len();
    let width = lines[0].len();
    let mut walls = vec![vec![false; width]; height];
    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);
    for (r, row) in lines.iter().enumerate() {
        for (c, ch) in row.char_indices() {
            match ch {
                '.' => continue,
                '#' => walls[r][c] = true,
                'S' => start = (r, c),
                'E' => end = (r, c),
                _ => panic!("unknown character: {ch}"),
            }
        }
    }
    Grid {
        walls,
        height,
        width,
        start,
        end,
    }
}

pub fn get_score_day16_stage1(grid: &Grid) -> u64 {
    let mut heap = BinaryHeap::new();

    // Distance map: best known cost to reach a given state
    let mut dist: HashMap<(usize, usize, Direction), u64> = HashMap::new();

    // Initialise: starting position and direction (East as specified in the problem)
    let state = State {
        row: grid.start.0,
        col: grid.start.1,
        direction: Direction::East,
        cost: 0,
    };
    dist.insert((grid.start.0, grid.start.1, Direction::East), 0);
    heap.push(Reverse(state));

    while let Some(Reverse(state)) = heap.pop() {
        let State {
            row,
            col,
            direction,
            cost,
        } = state;
        // If we reached the end, this is the minimum cost
        if (row, col) == grid.end {
            return cost;
        }

        // Ignore stale entries
        if cost > *dist.get(&(row, col, direction)).unwrap() {
            continue;
        }

        // Generate neighbours
        for (next_dir, step_cost) in neighbours(&direction) {
            if let Some((nr, nc)) = step(row, col, &next_dir, grid) {
                let next_cost = cost + step_cost;

                let key = (nr, nc, next_dir);
                let best = dist.get(&key).copied().unwrap_or(u64::MAX);

                if next_cost < best {
                    dist.insert(key, next_cost);
                    heap.push(Reverse(State {
                        row: nr,
                        col: nc,
                        direction: next_dir,
                        cost: next_cost,
                    }));
                }
            }
        }
    }

    panic!("No path the end");
}

fn neighbours(dir: &Direction) -> impl Iterator<Item = (Direction, u64)> {
    [(*dir, 1), (dir.turn_left(), 1001), (dir.turn_right(), 1001)].into_iter()
}

fn step(row: usize, col: usize, dir: &Direction, grid: &Grid) -> Option<(usize, usize)> {
    let (dr, dc) = dir.delta();

    let nr = row.checked_add_signed(dr)?;
    let nc = col.checked_add_signed(dc)?;

    if nr >= grid.height || nc >= grid.width {
        return None;
    }

    if grid.walls[nr][nc] {
        return None;
    }

    Some((nr, nc))
}

#[cfg(test)]
mod day16 {
    use super::*;

    fn get_example1() -> Vec<String> {
        vec![
            String::from("###############"),
            String::from("#.......#....E#"),
            String::from("#.#.###.#.###.#"),
            String::from("#.....#.#...#.#"),
            String::from("#.###.#####.#.#"),
            String::from("#.#.#.......#.#"),
            String::from("#.#.#####.###.#"),
            String::from("#...........#.#"),
            String::from("###.#.#####.#.#"),
            String::from("#...#.....#.#.#"),
            String::from("#.#.#.###.#.#.#"),
            String::from("#.....#...#.#.#"),
            String::from("#.###.#.#.#.#.#"),
            String::from("#S..#.....#...#"),
            String::from("###############"),
        ]
    }

    fn get_example2() -> Vec<String> {
        vec![
            String::from("#################"),
            String::from("#...#...#...#..E#"),
            String::from("#.#.#.#.#.#.#.#.#"),
            String::from("#.#.#.#...#...#.#"),
            String::from("#.#.#.#.###.#.#.#"),
            String::from("#...#.#.#.....#.#"),
            String::from("#.#.#.#.#.#####.#"),
            String::from("#.#...#.#.#.....#"),
            String::from("#.#.#####.#.###.#"),
            String::from("#.#.#.......#...#"),
            String::from("#.#.###.#####.###"),
            String::from("#.#.#...#.....#.#"),
            String::from("#.#.#.#####.###.#"),
            String::from("#.#.#.........#.#"),
            String::from("#.#.#.#########.#"),
            String::from("#S#.............#"),
            String::from("#################"),
        ]
    }

    #[test]
    fn score_example1_stage1() {
        let grid = parse_day16(&get_example1());
        let score = get_score_day16_stage1(&grid);
        assert_eq!(score, 7036);
    }

    #[test]
    fn score_example2_stage1() {
        let grid = parse_day16(&get_example2());
        let score = get_score_day16_stage1(&grid);
        assert_eq!(score, 11048);
    }
}
