use std::collections::HashMap;

pub struct KeypadComundrum {
    codes: Vec<String>,
    keypads: Vec<KeypadType>,
    paths: HashMap<(char, char, usize), u64>, // (from_char, to_char, level) -> shortest length
}

#[derive(Clone)]
enum KeypadType {
    Numeric,
    Direction,
}

pub fn parse_day21(lines: &[String]) -> KeypadComundrum {
    let codes = lines.iter().map(|s| s.trim().to_string()).collect();
    KeypadComundrum {
        codes,
        keypads: vec![
            KeypadType::Direction,
            KeypadType::Direction,
            KeypadType::Direction,
            KeypadType::Numeric,
        ],
        paths: HashMap::new(),
    }
}

pub fn result_stage1_day21(kc: &mut KeypadComundrum) -> u64 {
    kc.codes
        .to_vec()
        .iter()
        .map(|code| {
            let numeric_part: u64 = code
                .trim_end_matches("A")
                .parse()
                .expect("Code should be a number followed by 'A'");
            let code_cost = kc.get_code_cost(code, kc.keypads.len() - 1);
            code_cost * numeric_part
        })
        .sum()
}

pub fn result_stage2_day21(kc: &mut KeypadComundrum) -> u64 {
    kc.paths.clear();
    let mut keypads = vec![KeypadType::Direction; 26];
    keypads.push(KeypadType::Numeric);
    kc.keypads = keypads;
    kc.codes
        .to_vec()
        .iter()
        .map(|code| {
            let numeric_part: u64 = code
                .trim_end_matches("A")
                .parse()
                .expect("Code should be a number followed by 'A'");
            let code_cost = kc.get_code_cost(code, kc.keypads.len() - 1);
            code_cost * numeric_part
        })
        .sum()
}

impl KeypadComundrum {
    /// Calculate the total cost to type a full code (e.g., "029A")
    fn get_code_cost(&mut self, code: &str, level: usize) -> u64 {
        let mut total = 0;
        let mut current = 'A';
        for target in code.chars() {
            total += self.get_min_cost(current, target, level);
            current = target;
        }
        total
    }

    /// The Recursive solver: Checks the cache, calculates cost if missing.
    fn get_min_cost(&mut self, from: char, to: char, level: usize) -> u64 {
        // 1. Return memoized value if it exists
        if let Some(&val) = self.paths.get(&(from, to, level)) {
            return val;
        }

        // 2. Logic for calculating cost based on keypad type at this level
        let cost = if level == 0 {
            1 // Human just presses the button, so 1 cost
        } else {
            self.calculate_recursive_cost(from, to, level)
        };

        self.paths.insert((from, to, level), cost);
        cost
    }

    /// Helper: Generates 1 or 2 valid arrow sequences (e.g., "<vA")
    /// that move from 'from' to 'to' without hitting the gap.
    fn get_valid_paths(&self, from: char, to: char, keypad: &KeypadType) -> Vec<String> {
        let (from_row, from_col) = keypad.get_coords(from);
        let (to_row, to_col) = keypad.get_coords(to);
        let gap = keypad.get_gap();

        let dr = to_row - from_row;
        let dc = to_col - from_col;

        let mut paths = Vec::new();

        let row_moves = if dr < 0 {
            "^".repeat(dr.unsigned_abs())
        } else {
            "v".repeat(dr as usize)
        };
        let col_moves = if dc < 0 {
            "<".repeat(dc.unsigned_abs())
        } else {
            ">".repeat(dc as usize)
        };

        // Option A: Horizontal then Vertical
        // The "corner" is (start_row, end_col)
        if (from_row, to_col) != gap {
            paths.push(format!("{}{}{}", col_moves, row_moves, "A"));
        }

        // Option B: Vertical then Horizontal
        // The "corner" is (end_row, start_col)
        if (to_row, from_col) != gap {
            let path = format!("{}{}{}", row_moves, col_moves, "A");
            if !paths.contains(&path) {
                paths.push(path);
            }
        }

        paths
    }

    fn calculate_recursive_cost(&mut self, from: char, to: char, level: usize) -> u64 {
        // Get all valid paths form 'from' to 'to' that don't hit the gap
        let possible_paths = self.get_valid_paths(from, to, &self.keypads[level]);

        // Find the one that is cheapest at level - 1
        possible_paths
            .iter()
            .map(|path| self.get_code_cost(path, level - 1))
            .min()
            .expect("Should always have at least one valid path")
    }
}

impl KeypadType {
    fn get_coords(&self, c: char) -> (isize, isize) {
        match self {
            KeypadType::Numeric => match c {
                '7' => (0, 0),
                '8' => (0, 1),
                '9' => (0, 2),
                '4' => (1, 0),
                '5' => (1, 1),
                '6' => (1, 2),
                '1' => (2, 0),
                '2' => (2, 1),
                '3' => (2, 2),
                '0' => (3, 1),
                'A' => (3, 2),
                _ => panic!("Invalid numeric char: {c}"),
            },
            KeypadType::Direction => match c {
                '^' => (0, 1),
                'A' => (0, 2),
                '<' => (1, 0),
                'v' => (1, 1),
                '>' => (1, 2),
                _ => panic!("Invalid direction char: {c}"),
            },
        }
    }

    fn get_gap(&self) -> (isize, isize) {
        match self {
            KeypadType::Numeric => (3, 0),
            KeypadType::Direction => (0, 0),
        }
    }
}

#[cfg(test)]
mod day21 {
    use super::*;

    fn get_example() -> Vec<String> {
        vec![
            String::from("029A"),
            String::from("980A"),
            String::from("179A"),
            String::from("456A"),
            String::from("379A"),
        ]
    }

    #[test]
    fn day21_parse() {
        let kc = parse_day21(&get_example());
        assert_eq!(kc.codes.len(), 5);
        assert_eq!(kc.codes, get_example());
    }

    #[test]
    fn day21_stage1() {
        let mut kc = parse_day21(&get_example());
        let result = result_stage1_day21(&mut kc);
        assert_eq!(result, 126384);
    }
}
