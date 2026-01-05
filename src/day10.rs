use std::collections::HashSet;

pub struct TopographicMap {
    map: Vec<Vec<u8>>,
    trailheads: Vec<(usize, usize)>,
    rows: isize,
    cols: isize,
}

impl TopographicMap {
    pub fn get_score_stage1(&self) -> usize {
        let mut total = 0;
        for &(trailhead_row, trailhead_col) in self.trailheads.iter() {
            let mut visited = vec![vec![false; self.cols as usize]; self.rows as usize];
            let mut stack = vec![(trailhead_row, trailhead_col)];
            let mut reachable_nines = HashSet::new();

            while let Some((r, c)) = stack.pop() {
                if visited[r][c] {
                    continue;
                }
                visited[r][c] = true;

                let h = self.map[r][c];
                if h == 9 {
                    reachable_nines.insert((r, c));
                    continue;
                }
                for (nr, nc) in self.get_neighbours(r, c) {
                    if self.map[nr][nc] == h + 1 {
                        stack.push((nr, nc));
                    }
                }
            }
            total += reachable_nines.len();
        }
        total
    }

    pub fn get_rating_stage2(&self) -> usize {
        let mut total = 0;
        for &(trailhead_row, trailhead_col) in self.trailheads.iter() {
            let mut visited = vec![vec![false; self.cols as usize]; self.rows as usize];
            total += self.trail_rating(trailhead_row, trailhead_col, &mut visited);
        }

        total
    }

    fn trail_rating(&self, r: usize, c: usize, visited: &mut Vec<Vec<bool>>) -> usize {
        let h = self.map[r][c];
        if h == 9 {
            return 1;
        }

        visited[r][c] = true;
        let mut total = 0;

        for (nr, nc) in self.get_neighbours(r, c) {
            if self.map[nr][nc] == h + 1 && !visited[nr][nc] {
                total += self.trail_rating(nr, nc, visited);
            }
        }

        visited[r][c] = false;
        total
    }

    fn get_neighbours(&self, row: usize, col: usize) -> Vec<(usize, usize)> {
        [(-1, 0), (0, -1), (1, 0), (0, 1)]
            .iter()
            .filter_map(|&(dr, dc)| {
                let nr = row as isize + dr;
                let nc = col as isize + dc;
                if nr >= 0 && nc >= 0 && nr < self.rows && nc < self.cols {
                    Some((nr as usize, nc as usize))
                } else {
                    None
                }
            })
            .collect()
    }
}

pub fn parse_day10(lines: &[String]) -> TopographicMap {
    let mut map = Vec::new();
    let mut trailheads = Vec::new();
    let rows = lines.len() as isize;
    let cols = lines[0].len() as isize;
    for (row, line) in lines.iter().enumerate() {
        let mut this_row = Vec::new();
        for (col, ch) in line.char_indices() {
            let height = ch.to_string().parse::<u8>().expect("not an integer");
            this_row.push(height);
            if height == 0 {
                trailheads.push((row, col));
            }
        }
        map.push(this_row);
    }

    TopographicMap {
        map,
        trailheads,
        rows,
        cols,
    }
}

#[cfg(test)]
mod day10 {
    use super::*;

    fn get_lines() -> Vec<String> {
        vec![
            String::from("89010123"),
            String::from("78121874"),
            String::from("87430965"),
            String::from("96549874"),
            String::from("45678903"),
            String::from("32019012"),
            String::from("01329801"),
            String::from("10456732"),
        ]
    }

    #[test]
    fn day10_stage1() {
        let map = parse_day10(&get_lines());
        let score = map.get_score_stage1();
        assert_eq!(score, 36);
    }

    #[test]
    fn day10_stage2() {
        let map = parse_day10(&get_lines());
        let rating = map.get_rating_stage2();
        assert_eq!(rating, 81);
    }
}
