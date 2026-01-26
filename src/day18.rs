use std::collections::VecDeque;

pub fn parse_day18(lines: &[String]) -> Vec<(usize, usize)> {
    lines
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (x, y) = line.split_once(",").expect("missing comma");
            let x: usize = x.trim().parse().expect("x should be an integer");
            let y: usize = y.trim().parse().expect("y should be an integer");
            (x, y)
        })
        .collect()
}

pub fn shortest_path_day18_stage1(grid_size: usize, wall_slice: &[(usize, usize)]) -> Option<u32> {
    let start = (0, 0);
    let end = (grid_size, grid_size);

    // Cerate and fill the wall grid
    let mut is_wall = vec![vec![false; grid_size + 1]; grid_size + 1];
    for &(x, y) in wall_slice {
        is_wall[y][x] = true;
    }

    // Initialize visited grid
    let mut visited = vec![vec![false; grid_size + 1]; grid_size + 1];
    let mut queue = VecDeque::new();

    queue.push_back((start.0, start.1, 0));
    visited[start.1][start.0] = true;

    while let Some((x, y, dist)) = queue.pop_front() {
        // Check if we reached the goal
        if (x, y) == end {
            return Some(dist);
        }

        // Potential neighbours
        let neighbours = [
            (x.wrapping_add(1), y),
            (x.wrapping_sub(1), y),
            (x, y.wrapping_add(1)),
            (x, y.wrapping_sub(1)),
        ];

        // Explore neighbours
        for (nx, ny) in neighbours {
            // Check boundaries,
            if nx <= grid_size && ny <= grid_size {
                // Check if it is a wall or has already been visited
                if !is_wall[ny][nx] && !visited[ny][nx] {
                    visited[ny][nx] = true;
                    queue.push_back((nx, ny, dist + 1));
                }
            }
        }
    }
    None // Path not found
}

pub fn find_breaking_byte_day18_stage2(
    grid_size: usize,
    all_coords: &[(usize, usize)],
) -> (usize, usize) {
    let mut low = 0;
    let mut high = all_coords.len() - 1;
    let mut answer_idx = 0;

    while low <= high {
        let mid = low + (high - low) / 2;

        // Run BFS with the first 'mid' bytes as walls
        if shortest_path_day18_stage1(grid_size, &all_coords[..=mid]).is_some() {
            // Path still exists, the "killer" byte must fall LATER
            low = mid + 1;
        } else {
            // Path is blocked! This mid-point might be the answer,
            // but let's look EARLIER to find the very first byte that blocked it
            answer_idx = mid;
            high = mid.wrapping_sub(1);
        }
    }

    all_coords[answer_idx]
}

#[cfg(test)]
mod day18 {
    use super::*;

    fn get_example() -> Vec<String> {
        vec![
            String::from("5,4"),
            String::from("4,2"),
            String::from("4,5"),
            String::from("3,0"),
            String::from("2,1"),
            String::from("6,3"),
            String::from("2,4"),
            String::from("1,5"),
            String::from("0,6"),
            String::from("3,3"),
            String::from("2,6"),
            String::from("5,1"),
            String::from("1,2"),
            String::from("5,5"),
            String::from("2,5"),
            String::from("6,5"),
            String::from("1,4"),
            String::from("0,4"),
            String::from("6,4"),
            String::from("1,1"),
            String::from("6,1"),
            String::from("1,0"),
            String::from("0,5"),
            String::from("1,6"),
            String::from("2,0"),
        ]
    }

    #[test]
    fn result_day18_stage1() {
        let all_walls = parse_day18(&get_example());
        let result = shortest_path_day18_stage1(6, &all_walls[..12]);
        assert_eq!(result, Some(22));
    }

    #[test]
    fn result_day18_stage2() {
        let all_walls = parse_day18(&get_example());
        let result = find_breaking_byte_day18_stage2(6, &all_walls);
        assert_eq!(result, (6, 1));
    }
}
