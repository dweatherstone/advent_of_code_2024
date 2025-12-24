use regex::Regex;

pub fn parse_day3(lines: &[String]) -> Vec<String> {
    let re = Regex::new(r"(?:mul\([0-9]{1,3},[0-9]{1,3}\)|do\(\)|don't\(\))").unwrap();
    let mut matches = Vec::new();
    for line in lines {
        let mul_statements = re.find_iter(line).map(|m| m.as_str().to_string());
        matches.extend(mul_statements);
    }
    matches
}

pub fn mul_sum(mul_stmts: &[String]) -> u64 {
    mul_stmts
        .iter()
        .filter_map(|s| {
            if s.as_str() == "do()" || s.as_str() == "don't()" {
                return None;
            }
            let (part1, part2) = s.split_once(',').expect("must have a comma");
            let (_, part1_num) = part1.split_once('(').expect("expected opening brace");
            let part2_num = part2.trim_end_matches(')');
            Some(
                part1_num.parse::<u64>().expect("expected a number")
                    * part2_num.parse::<u64>().expect("expected a number"),
            )
        })
        .sum()
}

pub fn mul_sum_conditional(mul_stmts: &[String]) -> u64 {
    let mut do_multiply = true;
    let mut total_sum = 0;
    for stmt in mul_stmts {
        match stmt.chars().take(3).collect::<String>().as_str() {
            "mul" => {
                if !do_multiply {
                    continue;
                }
                let (part1, part2) = stmt.split_once(',').expect("must have a comma");
                let (_, part1_num) = part1.split_once('(').expect("expected opening brace");
                let part2_num = part2.trim_end_matches(')');
                total_sum += part1_num.parse::<u64>().expect("expected a number")
                    * part2_num.parse::<u64>().expect("expected a number")
            }
            "do(" => do_multiply = true,
            "don" => do_multiply = false,
            _ => panic!("unknown token"),
        }
    }
    total_sum
}

#[cfg(test)]
mod day3 {
    use super::*;

    fn get_input_lines() -> Vec<String> {
        vec![String::from(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        )]
    }

    #[test]
    fn day3_parse() {
        let mul_statements = parse_day3(&get_input_lines());
        let expected = vec![
            String::from("mul(2,4)"),
            String::from("don't()"),
            String::from("mul(5,5)"),
            String::from("mul(11,8)"),
            String::from("do()"),
            String::from("mul(8,5)"),
        ];
        assert_eq!(mul_statements.len(), expected.len());
        for (stmt, exp_stmt) in mul_statements.iter().zip(expected.iter()) {
            assert_eq!(stmt, exp_stmt);
        }
    }

    #[test]
    fn day3_stage1() {
        let mul_statements = parse_day3(&get_input_lines());
        let total_sum = mul_sum(&mul_statements);
        assert_eq!(total_sum, 161);
    }

    #[test]
    fn day3_stage2() {
        let mul_statements = parse_day3(&get_input_lines());
        let total_sum = mul_sum_conditional(&mul_statements);
        assert_eq!(total_sum, 48);
    }
}
