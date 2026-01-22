use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

pub struct Grid {
    walls: Vec<Vec<bool>>,
    height: usize,
    width: usize,
    start: (usize, usize),
    end: (usize, usize),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    row: usize,
    col: usize,
    dir: Direction,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct HeapItem {
    cost: u64,
    node: Node,
}

impl Ord for HeapItem {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for HeapItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
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
    let dist = dijkstra(grid, true);

    dist.iter()
        .filter_map(|(&node, &cost)| {
            if (node.row, node.col) == grid.end {
                Some(cost)
            } else {
                None
            }
        })
        .min()
        .unwrap()
}

pub fn get_seats_day16_stage2(grid: &Grid) -> usize {
    let dist = dijkstra(grid, false);

    let best_end_cost = dist
        .iter()
        .filter_map(|(&node, &cost)| {
            if (node.row, node.col) == grid.end {
                Some(cost)
            } else {
                None
            }
        })
        .min()
        .unwrap();

    let mut queue: Vec<Node> = dist
        .iter()
        .filter_map(|(&node, &cost)| {
            if (node.row, node.col) == grid.end && cost == best_end_cost {
                Some(node)
            } else {
                None
            }
        })
        .collect();
    let mut visited: HashSet<Node> = HashSet::new();
    let mut tiles: HashSet<(usize, usize)> = HashSet::new();

    while let Some(node) = queue.pop() {
        if !visited.insert(node) {
            continue;
        }
        tiles.insert((node.row, node.col));
        let curr_cost = dist[&node];

        let (dr, dc) = node.dir.delta();
        let pr = match node.row.checked_sub_signed(dr) {
            Some(v) => v,
            None => continue,
        };
        let pc = match node.col.checked_sub_signed(dc) {
            Some(v) => v,
            None => continue,
        };

        for (pd, step_cost) in predecessor_moves(&node.dir) {
            let prev = Node {
                row: pr,
                col: pc,
                dir: pd,
            };
            if let Some(&prev_cost) = dist.get(&prev)
                && prev_cost + step_cost == curr_cost
            {
                queue.push(prev);
            }
        }
    }

    tiles.len()
}

fn dijkstra(grid: &Grid, stop_at_end: bool) -> HashMap<Node, u64> {
    let mut heap: BinaryHeap<HeapItem> = BinaryHeap::new();

    // Distance map: best known cost to reach a given state
    let mut dist: HashMap<Node, u64> = HashMap::new();

    let start_node = Node {
        row: grid.start.0,
        col: grid.start.1,
        dir: Direction::East,
    };

    dist.insert(start_node, 0);
    heap.push(HeapItem {
        cost: 0,
        node: start_node,
    });

    while let Some(HeapItem { cost, node }) = heap.pop() {
        // Ignore stale entries
        if cost > dist[&node] {
            continue;
        }

        // Safe because the first time we pop an end node, its cost is minimal
        if stop_at_end && (node.row, node.col) == grid.end {
            break;
        }

        // Generate neighbours
        for (next_dir, step_cost) in neighbours(&node.dir) {
            if let Some((nr, nc)) = step(node.row, node.col, &next_dir, grid) {
                let next_cost = cost + step_cost;

                let key = Node {
                    row: nr,
                    col: nc,
                    dir: next_dir,
                };

                if next_cost < dist.get(&key).copied().unwrap_or(u64::MAX) {
                    dist.insert(key, next_cost);
                    heap.push(HeapItem {
                        cost: next_cost,
                        node: key,
                    });
                }
            }
        }
    }
    dist
}

fn neighbours(dir: &Direction) -> impl Iterator<Item = (Direction, u64)> {
    [(*dir, 1), (dir.turn_left(), 1001), (dir.turn_right(), 1001)].into_iter()
}

fn predecessor_moves(curr: &Direction) -> impl Iterator<Item = (Direction, u64)> {
    [
        (*curr, 1),
        (curr.turn_left(), 1001),
        (curr.turn_right(), 1001),
    ]
    .into_iter()
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

    #[test]
    fn seats_example1_stage2() {
        let grid = parse_day16(&get_example1());
        let seats = get_seats_day16_stage2(&grid);
        assert_eq!(seats, 45);
    }

    #[test]
    fn seats_example2_stage2() {
        let grid = parse_day16(&get_example2());
        let seats = get_seats_day16_stage2(&grid);
        assert_eq!(seats, 64);
    }
}
