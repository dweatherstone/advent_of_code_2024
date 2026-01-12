use std::collections::HashMap;

pub fn parse_day11(lines: &[String]) -> Vec<String> {
    if lines.len() != 1 {
        panic!("unexpected number of lines");
    }
    lines[0]
        .split_whitespace()
        .map(|s| s.trim().to_string())
        .collect()
}

pub fn number_of_stones(stones: &[String], n: usize) -> u64 {
    let mut map: HashMap<u64, u64> = HashMap::new();

    for s in stones.iter() {
        let value = s.trim().parse::<u64>().expect("should be a number");
        *map.entry(value).or_insert(0) += 1;
    }

    for _ in 0..n {
        let mut next = HashMap::new();
        for (value, count) in map {
            if value == 0 {
                *next.entry(1).or_insert(0) += count;
            } else {
                let d = digits(value);
                if d % 2 == 0 {
                    let (a, b) = split(value, d);
                    *next.entry(a).or_insert(0) += count;
                    *next.entry(b).or_insert(0) += count;
                } else {
                    *next.entry(value * 2024).or_insert(0) += count;
                }
            }
        }
        map = next;
    }

    map.values().sum()
}

fn digits(x: u64) -> u32 {
    x.checked_ilog10().map(|v| v + 1).unwrap_or(1)
}

fn split(x: u64, d: u32) -> (u64, u64) {
    let pow = 10u64.pow(d / 2);
    (x / pow, x % pow)
}

#[cfg(test)]
mod day11 {
    use super::*;

    fn get_input() -> Vec<String> {
        vec![String::from("125 17")]
    }

    #[test]
    fn day11_stage1() {
        let stones = parse_day11(&get_input());
        let stone_count = number_of_stones(&stones, 6);
        assert_eq!(stone_count, 22);
        let stone_count = number_of_stones(&stones, 25);
        assert_eq!(stone_count, 55312);
    }
}
