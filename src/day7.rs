use std::{fmt, iter::repeat};

use itertools::Itertools;

pub struct Equation {
    target: u64,
    operands: Vec<u64>,
}

impl Equation {
    fn is_valid(&self, include_concatenation: bool) -> bool {
        let n = self.operands.len();
        if n == 0 {
            return false;
        }

        let slots = n - 1;

        let operator_options = if include_concatenation {
            vec![
                Operator::Addition,
                Operator::Multiplication,
                Operator::Concatenation,
            ]
        } else {
            vec![Operator::Addition, Operator::Multiplication]
        };

        for ops in repeat(operator_options)
            .take(slots)
            .multi_cartesian_product()
        {
            let mut total = self.operands[0];
            for (i, &b) in self.operands.iter().skip(1).enumerate() {
                total = ops[i].calculate(total, b);
            }
            if total == self.target {
                return true;
            }
        }

        false
    }
}

impl fmt::Display for Equation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let operands_str = self.operands.iter().map(|op| op.to_string()).join(" ");
        write!(f, "{}: {}", self.target, operands_str)
    }
}

#[derive(Clone)]
enum Operator {
    Addition,
    Multiplication,
    Concatenation,
}

impl Operator {
    fn calculate(&self, a: u64, b: u64) -> u64 {
        match self {
            Operator::Addition => a + b,
            Operator::Multiplication => a * b,
            Operator::Concatenation => {
                let concat_str = a.to_string() + b.to_string().as_str();
                concat_str
                    .parse()
                    .expect("concatenation does not produce an integer")
            }
        }
    }
}

pub fn parse_day7(lines: &[String]) -> Vec<Equation> {
    let mut equations = Vec::new();
    for line in lines {
        let (target_str, operands_str) = line.split_once(':').expect("missing ':' delimiter");
        let target = target_str.parse::<u64>().expect("not an integer");
        let operands = operands_str
            .split_whitespace()
            .map(|s| s.trim().parse::<u64>().expect("not an integer"))
            .collect::<Vec<_>>();
        equations.push(Equation { target, operands });
    }
    equations
}

pub fn get_result_day7_stage1(equations: &[Equation]) -> u64 {
    equations
        .iter()
        .filter_map(|eq| {
            if eq.is_valid(false) {
                Some(eq.target)
            } else {
                None
            }
        })
        .sum()
}

pub fn get_result_day7_stage2(equations: &[Equation]) -> u64 {
    equations
        .iter()
        .filter_map(|eq| {
            if eq.is_valid(true) {
                Some(eq.target)
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod day7 {
    use super::*;

    fn get_lines() -> Vec<String> {
        vec![
            String::from("190: 10 19"),
            String::from("3267: 81 40 27"),
            String::from("83: 17 5"),
            String::from("156: 15 6"),
            String::from("7290: 6 8 6 15"),
            String::from("161011: 16 10 13"),
            String::from("192: 17 8 14"),
            String::from("21037: 9 7 18 13"),
            String::from("292: 11 6 16 20"),
        ]
    }

    #[test]
    fn day7_parse() {
        let equations = parse_day7(&get_lines());
        let expected_equations = vec![
            Equation {
                target: 190,
                operands: vec![10, 19],
            },
            Equation {
                target: 3267,
                operands: vec![81, 40, 27],
            },
            Equation {
                target: 83,
                operands: vec![17, 5],
            },
            Equation {
                target: 156,
                operands: vec![15, 6],
            },
            Equation {
                target: 7290,
                operands: vec![6, 8, 6, 15],
            },
            Equation {
                target: 161011,
                operands: vec![16, 10, 13],
            },
            Equation {
                target: 192,
                operands: vec![17, 8, 14],
            },
            Equation {
                target: 21037,
                operands: vec![9, 7, 18, 13],
            },
            Equation {
                target: 292,
                operands: vec![11, 6, 16, 20],
            },
        ];
        assert_eq!(equations.len(), expected_equations.len());
        for (result, expected) in equations.iter().zip(expected_equations.iter()) {
            assert_eq!(result.target, expected.target);
            assert_eq!(result.operands, expected.operands);
        }
    }

    #[test]
    fn day7_is_valid_stage1() {
        let equations = parse_day7(&get_lines());
        let expected = [true, true, false, false, false, false, false, false, true];
        for (equation, exp) in equations.iter().zip(expected) {
            assert_eq!(
                equation.is_valid(false),
                exp,
                "{equation} should be {exp}, but got {}",
                equation.is_valid(false)
            );
        }
    }

    #[test]
    fn day7_is_valid_stage2() {
        let equations = parse_day7(&get_lines());
        let expected = [true, true, false, true, true, false, true, false, true];
        for (equation, exp) in equations.iter().zip(expected) {
            assert_eq!(
                equation.is_valid(true),
                exp,
                "{equation} should be {exp}, but got {}",
                equation.is_valid(true)
            );
        }
    }

    #[test]
    fn day7_stage1() {
        let equations = parse_day7(&get_lines());
        let result = get_result_day7_stage1(&equations);
        assert_eq!(result, 3749);
    }

    #[test]
    fn day7_stage2() {
        let equations = parse_day7(&get_lines());
        let result = get_result_day7_stage2(&equations);
        assert_eq!(result, 11387);
    }
}
