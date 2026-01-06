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

pub fn number_of_stones(stones: &[String], n: usize) -> usize {
    let mut splitting_stones = stones.to_vec();
    for _ in 0..n {
        let mut temp_stones = splitting_stones.clone();
        let mut i = 0;
        for stone in splitting_stones.iter() {
            if stone.trim() == "0" {
                temp_stones[i] = String::from("1");
                i += 1;
                continue;
            }
            if stone.trim().len() % 2 == 0 {
                let s = stone.trim();
                let (a, b) = s.split_at(s.len() / 2);
                temp_stones[i] = a.parse::<u64>().expect("should be a number").to_string();
                temp_stones.insert(
                    i + 1,
                    b.parse::<u64>().expect("should be a number").to_string(),
                );
                i += 2;
                continue;
            }
            let val = stone.parse::<u64>().expect("should be a number") * 2024;
            temp_stones[i] = val.to_string();
            i += 1;
        }
        splitting_stones = temp_stones;
    }

    //println!("Stones: {:?}", splitting_stones);
    splitting_stones.len()
}

pub fn number_of_stones2(stones: &[String], n: usize) -> u64 {
    // initial population - value -> count
    let mut map: HashMap<u64, u64> = HashMap::new();
    for s in stones {
        let v = s.parse::<u64>().expect("not a nunber");
        *map.entry(v).or_insert(0) += 1;
    }

    for _ in 0..n {
        let mut next = HashMap::new();

        for (v, c) in map {
            if v == 0 {
                *next.entry(1).or_insert(0) += c;
            } else {
                let d = digits(v);
                if d % 2 == 0 {
                    let (a, b) = split(v, d);
                    *next.entry(a).or_insert(0) += c;
                    *next.entry(b).or_insert(0) += c;
                } else {
                    *next.entry(v * 2024).or_insert(0) += c;
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

    #[test]
    fn day11_stage2() {
        let stones = parse_day11(&get_input());
        let stone_count = number_of_stones2(&stones, 6);
        assert_eq!(stone_count, 22);
        // let stone_count = number_of_stones2(&stones, 25);
        // assert_eq!(stone_count, 55312);
    }
}
