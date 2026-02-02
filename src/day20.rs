use core::iter::Iterator;
use std::collections::{HashMap, HashSet};

use strum::{EnumIter, IntoEnumIterator};

pub struct Track {
    walls: Vec<Vec<bool>>,
    height: usize,
    width: usize,
    start: (usize, usize),
    end: (usize, usize),
    cheats: HashMap<CheatKey, usize>, // ((start.row, start.col), (end.row, end.col)) -> saved
    honest_route: Vec<(usize, usize)>,
}

impl Track {
    pub fn get_honest_route(&mut self) -> Vec<(usize, usize)> {
        if !self.honest_route.is_empty() {
            return self.honest_route.clone();
        }
        let (mut current_row, mut current_col) = self.start;
        let mut visited = HashSet::new();

        visited.insert((current_row, current_col));
        self.honest_route.push((current_row, current_col));
        while (current_row, current_col) != self.end {
            for dir in Direction::iter() {
                let (dr, dc) = dir.delta();
                let nr = current_row.wrapping_add_signed(dr);
                let nc = current_col.wrapping_add_signed(dc);
                if nr >= self.height || nc >= self.width {
                    continue;
                }
                if visited.contains(&(nr, nc)) || self.walls[nr][nc] {
                    continue;
                }
                visited.insert((nr, nc));
                self.honest_route.push((nr, nc));
                current_row = nr;
                current_col = nc;

                break;
            }
        }
        self.honest_route.clone()
    }

    pub fn find_cheats(&mut self, max_time: usize) -> HashMap<CheatKey, usize> {
        self.cheats.clear();
        let route = &self.honest_route;

        for i in 0..route.len() {
            for j in (i + 1)..route.len() {
                let p1 = route[i];
                let p2 = route[j];

                // Manhattan distance
                let dist = p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1);

                if dist <= max_time {
                    let actual_distance = j - i;
                    if actual_distance > dist {
                        let saved = actual_distance - dist;
                        self.cheats.insert([p1, p2], saved);
                    }
                }
            }
        }

        self.cheats.clone()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, EnumIter)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn delta(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

type CheatKey = [(usize, usize); 2];

pub fn parse_day20(lines: &[String]) -> Track {
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
                _ => panic!("unkown character: {ch}"),
            }
        }
    }
    Track {
        walls,
        height,
        width,
        start,
        end,
        cheats: HashMap::new(),
        honest_route: Vec::new(),
    }
}

pub fn day20_stage1_result(track: &mut Track, min_save: usize) -> usize {
    track.get_honest_route();
    let savings = track.find_cheats(2);
    savings.values().filter(|&&s| s >= min_save).count()
}

pub fn day20_stage2_result(track: &mut Track, min_save: usize) -> usize {
    track.get_honest_route();
    let savings = track.find_cheats(20);
    savings.values().filter(|&&s| s >= min_save).count()
}

#[cfg(test)]
mod day20 {
    use super::*;

    fn get_example() -> Vec<String> {
        vec![
            String::from("###############"),
            String::from("#...#...#.....#"),
            String::from("#.#.#.#.#.###.#"),
            String::from("#S#...#.#.#...#"),
            String::from("#######.#.#.###"),
            String::from("#######.#.#...#"),
            String::from("#######.#.###.#"),
            String::from("###..E#...#...#"),
            String::from("###.#######.###"),
            String::from("#...###...#...#"),
            String::from("#.#####.#.###.#"),
            String::from("#.#...#.#.#...#"),
            String::from("#.#.#.#.#.#.###"),
            String::from("#...#...#...###"),
            String::from("###############"),
        ]
    }

    #[test]
    fn parse_example() {
        let track = parse_day20(&get_example());
        assert_eq!(track.start, (3, 1));
        assert_eq!(track.end, (7, 5));
        assert_eq!(track.width, 15);
        assert_eq!(track.height, 15);
        let wall_count: usize = track.walls.iter().flatten().filter(|&&b| b).count();
        assert_eq!(wall_count, 140);
    }

    #[test]
    fn honest_route() {
        let mut track = parse_day20(&get_example());
        let honest_route = track.get_honest_route();
        assert_eq!(honest_route.len(), 85);
        assert_eq!(track.honest_route.len(), 85);
    }

    #[test]
    fn find_cheats() {
        let mut track = parse_day20(&get_example());
        track.get_honest_route();
        let savings = track.find_cheats(2);
        assert_eq!(savings.len(), 44);
        assert_eq!(savings.values().filter(|&&s| s == 2).count(), 14);
        assert_eq!(savings.values().filter(|&&s| s == 4).count(), 14);
        assert_eq!(savings.values().filter(|&&s| s == 6).count(), 2);
        assert_eq!(savings.values().filter(|&&s| s == 8).count(), 4);
        assert_eq!(savings.values().filter(|&&s| s == 10).count(), 2);
        assert_eq!(savings.values().filter(|&&s| s == 12).count(), 3);
        assert_eq!(savings.values().filter(|&&s| s == 20).count(), 1);
        assert_eq!(savings.values().filter(|&&s| s == 36).count(), 1);
        assert_eq!(savings.values().filter(|&&s| s == 38).count(), 1);
        assert_eq!(savings.values().filter(|&&s| s == 40).count(), 1);
        assert_eq!(savings.values().filter(|&&s| s == 64).count(), 1);

        assert_eq!(savings.get(&[(1, 7), (1, 9)]), Some(&12));
        assert_eq!(savings.get(&[(7, 9), (7, 11)]), Some(&20));
        assert_eq!(savings.get(&[(7, 8), (9, 8)]), Some(&38));
        assert_eq!(savings.get(&[(7, 7), (7, 5)]), Some(&64));
    }

    #[test]
    fn stage1() {
        let mut track = parse_day20(&get_example());
        assert_eq!(day20_stage1_result(&mut track, 10), 10);
        assert_eq!(day20_stage1_result(&mut track, 50), 1);
        assert_eq!(day20_stage1_result(&mut track, 70), 0);
        assert_eq!(day20_stage1_result(&mut track, 3), 30);
        assert_eq!(day20_stage1_result(&mut track, 2), 44);
    }

    #[test]
    fn stage2() {
        let mut track = parse_day20(&get_example());
        let result = day20_stage2_result(&mut track, 50);
        assert_eq!(result, 285);
        let savings = track.cheats;
        // expected = (num of cheats, saving)
        let expected = [
            (32, 50),
            (31, 52),
            (29, 54),
            (39, 56),
            (25, 58),
            (23, 60),
            (20, 62),
            (19, 64),
            (12, 66),
            (14, 68),
            (12, 70),
            (22, 72),
            (4, 74),
            (3, 76),
        ];
        for (exp_qty, saving) in expected {
            let actual_qty = savings.values().filter(|&&s| s == saving).count();
            assert_eq!(actual_qty, exp_qty);
        }
        assert_eq!(savings.get(&[(3, 1), (7, 3)]), Some(&76));
    }
}
