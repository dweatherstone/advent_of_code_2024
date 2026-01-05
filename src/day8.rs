use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub struct AntennaMap {
    antennas: HashMap<char, Vec<Pos>>,
    rows: isize,
    cols: isize,
}

impl AntennaMap {
    pub fn get_antinodes_stage1(&self) -> usize {
        let mut antinodes: HashSet<Pos> = HashSet::new();
        for antennas in self.antennas.values() {
            for pair in antennas.iter().combinations(2) {
                self.add_anitnode(pair[0], pair[1], &mut antinodes);
                self.add_anitnode(pair[1], pair[0], &mut antinodes);
            }
        }

        antinodes.len()
    }

    pub fn get_antinodes_stage2(&self) -> usize {
        let mut antinodes: HashSet<Pos> = HashSet::new();

        for antennas in self.antennas.values() {
            antennas.iter().for_each(|&pos| _ = antinodes.insert(pos));
            for pair in antennas.iter().combinations(2) {
                self.add_antinode_recur(pair[0], pair[1], &mut antinodes);
                self.add_antinode_recur(pair[1], pair[0], &mut antinodes);
            }
        }

        antinodes.len()
    }

    fn add_anitnode(&self, a: &Pos, b: &Pos, antinodes: &mut HashSet<Pos>) {
        let dr = a.0 as isize - b.0 as isize;
        let dc = a.1 as isize - b.1 as isize;
        let nr = a.0 as isize + dr;
        let nc = a.1 as isize + dc;
        if nr >= 0 && nc >= 0 && nr < self.rows && nc < self.cols {
            antinodes.insert((nr as usize, nc as usize));
        }
    }

    fn add_antinode_recur(&self, a: &Pos, b: &Pos, antinodes: &mut HashSet<Pos>) {
        let dr = a.0 as isize - b.0 as isize;
        let dc = a.1 as isize - b.1 as isize;
        let mut nr = a.0 as isize + dr;
        let mut nc = a.1 as isize + dc;
        while nr >= 0 && nc >= 0 && nr < self.rows && nc < self.cols {
            antinodes.insert((nr as usize, nc as usize));
            nr += dr;
            nc += dc;
        }
    }
}

type Pos = (usize, usize);

pub fn parse_day8(lines: &[String]) -> AntennaMap {
    let mut antennas: HashMap<char, Vec<Pos>> = HashMap::new();
    let rows = lines.len() as isize;
    let cols = lines[0].len() as isize;
    for (row, line) in lines.iter().enumerate() {
        for (col, ch) in line.char_indices() {
            match ch {
                '.' => continue,
                _ => antennas.entry(ch).or_default().push((row, col)),
            }
        }
    }
    AntennaMap {
        antennas,
        rows,
        cols,
    }
}

#[cfg(test)]
mod day8 {
    use super::*;

    fn get_lines() -> Vec<String> {
        vec![
            String::from("............"),
            String::from("........0..."),
            String::from(".....0......"),
            String::from(".......0...."),
            String::from("....0......."),
            String::from("......A....."),
            String::from("............"),
            String::from("............"),
            String::from("........A..."),
            String::from(".........A.."),
            String::from("............"),
            String::from("............"),
        ]
    }

    #[test]
    #[allow(non_snake_case)]
    fn day8_parse() {
        let am = parse_day8(&get_lines());
        let antennas_0 = am.antennas.get(&'0');
        assert!(antennas_0.is_some());
        let antennas_0 = antennas_0.unwrap();
        assert_eq!(antennas_0, &vec![(1, 8), (2, 5), (3, 7), (4, 4)]);
        let antennas_A = am.antennas.get(&'A');
        assert!(antennas_A.is_some());
        let antennas_A = antennas_A.unwrap();
        assert_eq!(antennas_A, &vec![(5, 6), (8, 8), (9, 9)]);
    }

    #[test]
    fn day8_stage1() {
        let map = parse_day8(&get_lines());
        let result = map.get_antinodes_stage1();
        assert_eq!(result, 14);
    }

    #[test]
    fn day8_stage2() {
        let map = parse_day8(&get_lines());
        let result = map.get_antinodes_stage2();
        assert_eq!(result, 34);
    }
}
