use std::{collections::HashSet, fmt::Display};

use strum::{EnumIter, IntoEnumIterator};

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

    fn perimeter(&self) -> usize {
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

    fn sides(&self) -> usize {
        let edges = self.boundary_edges();
        count_sides(&edges)
    }

    fn boundary_edges(&self) -> Vec<Edge> {
        // iterate plots
        // for each plot, check 4 neighbours
        // if neighbour not in plot_set -> add Edge
        let mut edges = Vec::new();
        for &(r, c) in self.plots.iter() {
            for dir in EdgeDir::iter() {
                let (dr, dc) = dir.direction();
                let new_cell = (r + dr, c + dc);
                if self.plot_set.contains(&new_cell) {
                    continue;
                }
                edges.push(Edge { cell: (r, c), dir });
            }
        }
        edges
    }

    fn price_perimeter(&self) -> usize {
        self.area() * self.perimeter()
    }

    fn price_sides(&self) -> usize {
        self.area() * self.sides()
    }
}

impl Display for GardenRegion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Plots = {:?}", self.plots)
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
    regions.iter().map(|region| region.price_perimeter()).sum()
}

pub fn stage2_result(regions: &[GardenRegion]) -> usize {
    regions.iter().map(|region| region.price_sides()).sum()
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
            // Is `next` inside the grid?
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

#[derive(EnumIter, Clone, Copy, PartialEq, Eq, Hash)]
enum EdgeDir {
    North,
    South,
    East,
    West,
}

impl EdgeDir {
    const fn direction(&self) -> (isize, isize) {
        match self {
            EdgeDir::North => (-1, 0),
            EdgeDir::South => (1, 0),
            EdgeDir::East => (0, -1),
            EdgeDir::West => (0, 1),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Edge {
    cell: Coord,
    dir: EdgeDir,
}

fn count_sides(edges: &[Edge]) -> usize {
    // group edges by direction
    // count straight runs in each group
    let edge_set: HashSet<Edge> = HashSet::from_iter(edges.iter().cloned());
    edge_set
        .iter()
        .filter(|&edge| starts_new_side(edge, &edge_set))
        .count()
}

fn starts_new_side(edge: &Edge, edge_set: &HashSet<Edge>) -> bool {
    // based on direction:
    // check if a "previous" edge exists
    let (dr, dc) = match edge.dir {
        EdgeDir::North | EdgeDir::South => (0, -1), // check left
        EdgeDir::East | EdgeDir::West => (-1, 0),   // check up
    };
    let prev_cell = (edge.cell.0 + dr, edge.cell.1 + dc);
    let prev_edge = Edge {
        cell: prev_cell,
        dir: edge.dir,
    };
    !edge_set.contains(&prev_edge)
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

    fn example4_lines() -> Vec<String> {
        vec![
            String::from("AAAAAA"),
            String::from("AAABBA"),
            String::from("AAABBA"),
            String::from("ABBAAA"),
            String::from("ABBAAA"),
            String::from("AAAAAA"),
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

    #[test]
    fn example1_stage2() {
        let grid = parse_grid(&example1_lines());
        let regions = find_regions(&grid);
        let result = stage2_result(&regions);
        assert_eq!(result, 80);
    }

    #[test]
    fn example2_stage2() {
        let grid = parse_grid(&example2_lines());
        let regions = find_regions(&grid);
        let result = stage2_result(&regions);
        assert_eq!(result, 436);
    }

    #[test]
    fn example3_stage2() {
        let grid = parse_grid(&example3_lines());
        let regions = find_regions(&grid);
        let result = stage2_result(&regions);
        assert_eq!(result, 1206);
    }

    #[test]
    fn example4_stage2() {
        let grid = parse_grid(&example4_lines());
        let regions = find_regions(&grid);
        let result = stage2_result(&regions);
        assert_eq!(result, 368);
    }
}
