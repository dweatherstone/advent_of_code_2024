use std::{collections::HashSet, fmt::Display};

pub struct GardenRegion {
    plots: Vec<Coord>,
    plot_set: HashSet<Coord>,
}

impl GardenRegion {
    fn new(plots: Vec<Coord>) -> Self {
        let plot_set = HashSet::from_iter(plots.iter().cloned());
        GardenRegion { plots, plot_set }
    }

    fn area(&self) -> usize {
        self.plot_set.len()
    }

    pub fn perimeter(&self) -> usize {
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let mut perimeter = 0;

        for &(r, c) in self.plots.iter() {
            for &(dr, dc) in directions.iter() {
                let neighbour = (r + dr, c + dc);
                if !self.plot_set.contains(&neighbour) {
                    perimeter += 1;
                }
            }
        }
        perimeter
    }

    fn price(&self) -> usize {
        self.area() * self.perimeter()
    }
}

impl Display for GardenRegion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Plot Set = {:?}", self.plot_set)
    }
}

pub fn parse_grid(lines: &[String]) -> Vec<Vec<char>> {
    let mut grid = Vec::new();
    for line in lines {
        grid.push(line.chars().collect::<Vec<char>>());
    }
    grid
}

pub fn find_regions(grid: &[Vec<char>]) -> Vec<GardenRegion> {
    let mut visited = HashSet::new();
    let mut regions = Vec::new();

    for (r, row) in grid.iter().enumerate() {
        for (c, _) in row.iter().enumerate() {
            let start = (r as isize, c as isize);
            if visited.contains(&start) {
                continue;
            }
            let region = flood_fill(grid, start, &mut visited);
            regions.push(region);
        }
    }

    regions
}

pub fn stage1_result(regions: &[GardenRegion]) -> usize {
    regions.iter().map(|region| region.price()).sum()
}

fn flood_fill(grid: &[Vec<char>], start: Coord, visited: &mut HashSet<Coord>) -> GardenRegion {
    let directions: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut stack = vec![start];
    let mut plots = Vec::new();
    let plot_type = grid[start.0 as usize][start.1 as usize];
    visited.insert(start);

    while let Some((r, c)) = stack.pop() {
        plots.push((r, c));

        for (dr, dc) in directions {
            let next = (r + dr, c + dc);
            // Is `next`` inside the grid?
            if next.0 < 0
                || next.1 < 0
                || next.0 as usize >= grid.len()
                || next.1 as usize >= grid[0].len()
            {
                continue;
            }
            // Has `next` already been visited?
            if visited.contains(&next) {
                continue;
            }
            // Is `next` the same plot_type?
            if grid[next.0 as usize][next.1 as usize] != plot_type {
                continue;
            }
            visited.insert(next);
            stack.push(next);
        }
    }
    GardenRegion::new(plots)
}

type Coord = (isize, isize);

#[cfg(test)]
mod day12 {
    use super::*;

    fn example1_lines() -> Vec<String> {
        vec![
            String::from("AAAA"),
            String::from("BBCD"),
            String::from("BBCC"),
            String::from("EEEC"),
        ]
    }

    fn example2_lines() -> Vec<String> {
        vec![
            String::from("OOOOO"),
            String::from("OXOXO"),
            String::from("OOOOO"),
            String::from("OXOXO"),
            String::from("OOOOO"),
        ]
    }

    fn example3_lines() -> Vec<String> {
        vec![
            String::from("RRRRIICCFF"),
            String::from("RRRRIICCCF"),
            String::from("VVRRRCCFFF"),
            String::from("VVRCCCJFFF"),
            String::from("VVVVCJJCFE"),
            String::from("VVIVCCJJEE"),
            String::from("VVIIICJJEE"),
            String::from("MIIIIIJJEE"),
            String::from("MIIISIJEEE"),
            String::from("MMMISSJEEE"),
        ]
    }

    #[test]
    fn example1_perimeter() {
        let grid = parse_grid(&example1_lines());
        let regions = find_regions(&grid);
        let expected: [usize; 5] = [10, 8, 10, 4, 8];
        assert_eq!(regions.len(), 5);
        for (region, exp) in regions.iter().zip(&expected) {
            let result = region.perimeter();
            assert_eq!(&result, exp);
        }
    }

    #[test]
    fn example1_stage1() {
        let grid = parse_grid(&example1_lines());
        let regions = find_regions(&grid);
        let result = stage1_result(&regions);
        assert_eq!(result, 140);
    }

    #[test]
    fn example2_stage1() {
        let grid = parse_grid(&example2_lines());
        let regions = find_regions(&grid);
        let result = stage1_result(&regions);
        assert_eq!(result, 772);
    }

    #[test]
    fn example3_stage1() {
        let grid = parse_grid(&example3_lines());
        let regions = find_regions(&grid);
        let result = stage1_result(&regions);
        assert_eq!(result, 1930);
    }
}
