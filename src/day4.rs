pub fn parse_day4(lines: &[String]) -> Vec<Vec<char>> {
    lines
        .iter()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect()
}

pub fn count_xmas(puzzle: &[Vec<char>]) -> u32 {
    const DIRS: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    const PATTERN: [char; 4] = ['X', 'M', 'A', 'S'];

    let row_count = puzzle.len() as isize;
    let col_count = puzzle[0].len() as isize;
    let mut count = 0;

    for r in 0..puzzle.len() {
        for c in 0..puzzle[r].len() {
            if puzzle[r][c] != PATTERN[0] {
                continue;
            }

            count += DIRS
                .iter()
                .filter(|&&(dr, dc)| {
                    (1..4).all(|i| {
                        let nr = r as isize + dr * i;
                        let nc = c as isize + dc * i;

                        nr >= 0
                            && nr < row_count
                            && nc >= 0
                            && nc < col_count
                            && puzzle[nr as usize][nc as usize] == PATTERN[i as usize]
                    })
                })
                .count() as u32;
        }
    }
    count
}

pub fn count_x_mas(puzzle: &[Vec<char>]) -> u32 {
    let row_count = puzzle.len();
    let col_count = puzzle[0].len();
    let mut count = 0;
    if row_count < 3 || col_count < 3 {
        return 0;
    }
    for r in 1..row_count - 1 {
        for c in 1..col_count - 1 {
            if puzzle[r][c] != 'A' {
                continue;
            }
            let top_left = puzzle[r - 1][c - 1];
            let top_right = puzzle[r - 1][c + 1];
            let bottom_left = puzzle[r + 1][c - 1];
            let bottom_right = puzzle[r + 1][c + 1];

            // Check 4 valid X-MAS patterns
            if (top_left == 'M' && bottom_right == 'S' && top_right == 'M' && bottom_left == 'S')
                || (top_left == 'M'
                    && bottom_right == 'S'
                    && top_right == 'S'
                    && bottom_left == 'M')
                || (top_left == 'S'
                    && bottom_right == 'M'
                    && top_right == 'M'
                    && bottom_left == 'S')
                || (top_left == 'S'
                    && bottom_right == 'M'
                    && top_right == 'S'
                    && bottom_left == 'M')
            {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod day4 {
    use super::*;

    fn get_input_lines() -> Vec<String> {
        vec![
            String::from("MMMSXXMASM"),
            String::from("MSAMXMSMSA"),
            String::from("AMXSXMAAMM"),
            String::from("MSAMASMSMX"),
            String::from("XMASAMXAMM"),
            String::from("XXAMMXXAMA"),
            String::from("SMSMSASXSS"),
            String::from("SAXAMASAAA"),
            String::from("MAMMMXMMMM"),
            String::from("MXMXAXMASX"),
        ]
    }

    #[test]
    fn day4_parse() {
        let parsed = parse_day4(&get_input_lines());
        let expected = vec![
            "MMMSXXMASM".chars().collect::<Vec<char>>(),
            "MSAMXMSMSA".chars().collect::<Vec<char>>(),
            "AMXSXMAAMM".chars().collect::<Vec<char>>(),
            "MSAMASMSMX".chars().collect::<Vec<char>>(),
            "XMASAMXAMM".chars().collect::<Vec<char>>(),
            "XXAMMXXAMA".chars().collect::<Vec<char>>(),
            "SMSMSASXSS".chars().collect::<Vec<char>>(),
            "SAXAMASAAA".chars().collect::<Vec<char>>(),
            "MAMMMXMMMM".chars().collect::<Vec<char>>(),
            "MXMXAXMASX".chars().collect::<Vec<char>>(),
        ];
        assert_eq!(parsed.len(), expected.len());
        for (p, e) in parsed.iter().zip(expected.iter()) {
            assert_eq!(p, e);
        }
    }

    #[test]
    fn day4_stage1() {
        let parsed = parse_day4(&get_input_lines());
        let result = count_xmas(&parsed);
        assert_eq!(result, 18);
    }

    #[test]
    fn day4_stage2() {
        let parsed = parse_day4(&get_input_lines());
        let result = count_x_mas(&parsed);
        assert_eq!(result, 9);
    }
}
