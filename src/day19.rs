use std::collections::HashMap;

pub struct LinenLayout {
    towels: Vec<String>,
    designs: Vec<String>,
}

pub fn parse_day19(lines: &[String]) -> LinenLayout {
    let towels = lines[0].split(",").map(|s| s.trim().to_string()).collect();
    let designs = lines[2..].iter().map(|s| s.to_string()).collect();

    LinenLayout { towels, designs }
}

pub fn get_result_day19_stage1(layout: &LinenLayout) -> usize {
    let mut total_count = 0;
    for design in layout.designs.iter() {
        let mut memo = HashMap::new();
        if count_ways(design, &layout.towels, &mut memo) > 0 {
            total_count += 1;
        }
    }

    total_count
}

pub fn get_result_day19_stage2(layout: &LinenLayout) -> usize {
    let mut total_count = 0;
    for design in layout.designs.iter() {
        let mut memo = HashMap::new();
        total_count += count_ways(design, &layout.towels, &mut memo);
    }

    total_count
}

fn count_ways<'a>(design: &'a str, towels: &[String], memo: &mut HashMap<&'a str, usize>) -> usize {
    if design.is_empty() {
        return 1;
    }
    if let Some(&way_count) = memo.get(design) {
        return way_count;
    }

    let mut result = 0;
    for towel in towels {
        if design.starts_with(towel) {
            let remaining = &design[towel.len()..];
            result += count_ways(remaining, towels, memo)
        }
    }
    memo.insert(design, result);
    result
}

#[cfg(test)]
mod day19 {
    use super::*;

    fn get_example() -> Vec<String> {
        vec![
            String::from("r, wr, b, g, bwu, rb, gb, br"),
            String::from(""),
            String::from("brwrr"),
            String::from("bggr"),
            String::from("gbbr"),
            String::from("rrbgbr"),
            String::from("ubwu"),
            String::from("bwurrg"),
            String::from("brgr"),
            String::from("bbrgwb"),
        ]
    }

    #[test]
    fn parse() {
        let ll = parse_day19(&get_example());
        let expected_towels = vec![
            "r".to_string(),
            "wr".to_string(),
            "b".to_string(),
            "g".to_string(),
            "bwu".to_string(),
            "rb".to_string(),
            "gb".to_string(),
            "br".to_string(),
        ];
        let expected_designs = vec![
            "brwrr".to_string(),
            "bggr".to_string(),
            "gbbr".to_string(),
            "rrbgbr".to_string(),
            "ubwu".to_string(),
            "bwurrg".to_string(),
            "brgr".to_string(),
            "bbrgwb".to_string(),
        ];
        assert_eq!(ll.towels, expected_towels);
        assert_eq!(ll.designs, expected_designs);
    }

    #[test]
    fn stage1() {
        let layout = parse_day19(&get_example());
        let result = get_result_day19_stage1(&layout);
        assert_eq!(result, 6);
    }

    #[test]
    fn stage2() {
        let layout = parse_day19(&get_example());
        let result = get_result_day19_stage2(&layout);
        assert_eq!(result, 16);
    }
}
