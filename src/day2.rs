pub fn parse_day2(lines: &[String]) -> Vec<Vec<i32>> {
    let mut levels = Vec::new();
    for line in lines {
        let level = line
            .split_whitespace()
            .map(|l| l.parse::<i32>().expect("expected an integer"))
            .collect::<Vec<_>>();
        levels.push(level);
    }

    levels
}

pub fn safe_qty(levels: &[Vec<i32>], is_stage_1: bool) -> u32 {
    if is_stage_1 {
        levels.iter().map(|l| is_safe_stage1(l) as u32).sum()
    } else {
        levels.iter().map(|l| is_safe_stage2(l) as u32).sum()
    }
}

fn is_safe_stage1(level: &[i32]) -> bool {
    if level.len() < 2 {
        return false;
    }

    let mut direction: Option<i32> = None;

    for i in 1..level.len() {
        let diff = level[i] - level[i - 1];
        if diff == 0 {
            return false;
        }

        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }

        if let Some(dir) = direction {
            if diff.signum() != dir {
                return false;
            }
        } else {
            direction = Some(diff.signum());
        }
    }
    true
}

fn is_safe_stage2(level: &[i32]) -> bool {
    if is_safe_stage1(level) {
        return true;
    }

    for i in 0..level.len() {
        let mut reduced = Vec::with_capacity(level.len() - 1);
        reduced.extend_from_slice(&level[..i]);
        reduced.extend_from_slice(&level[i + 1..]);

        if is_safe_stage1(&reduced) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod day2 {
    use super::*;

    fn get_input_lines() -> Vec<String> {
        vec![
            String::from("7 6 4 2 1"),
            String::from("1 2 7 8 9"),
            String::from("9 7 6 2 1"),
            String::from("1 3 2 4 5"),
            String::from("8 6 4 4 1"),
            String::from("1 3 6 7 9"),
        ]
    }

    #[test]
    fn day2_parse() {
        let levels = parse_day2(&get_input_lines());
        let expected = [
            [7, 6, 4, 2, 1],
            [1, 2, 7, 8, 9],
            [9, 7, 6, 2, 1],
            [1, 3, 2, 4, 5],
            [8, 6, 4, 4, 1],
            [1, 3, 6, 7, 9],
        ];
        assert_eq!(levels.len(), expected.len());
        for (level, expected_level) in levels.iter().zip(expected.iter()) {
            assert_eq!(level, expected_level);
        }
    }

    #[test]
    fn day2_stage1() {
        let levels = parse_day2(&get_input_lines());
        let safe_qty = safe_qty(&levels, true);
        assert_eq!(safe_qty, 2);
    }

    #[test]
    fn day2_stage2() {
        let levels = parse_day2(&get_input_lines());
        let num_safe = safe_qty(&levels, false);
        assert_eq!(num_safe, 4);
    }
}
