pub struct Machine {
    button_a: Coord,
    button_b: Coord,
    prize: Coord,
}

impl Machine {
    fn get_tokens(&self) -> Option<i128> {
        let mut min_tokens = i128::MAX;
        for a in 0..=100 {
            for b in 0..=100 {
                let tokens = a * 3 + b;
                if tokens >= min_tokens {
                    continue;
                }
                let x_pos = a * self.button_a.0 + b * self.button_b.0;
                let y_pos = a * self.button_a.1 + b * self.button_b.1;
                if (x_pos, y_pos) == self.prize {
                    min_tokens = tokens;
                }
            }
        }

        if min_tokens < i128::MAX {
            Some(min_tokens)
        } else {
            None
        }
    }

    fn get_tokens_stage2(&self) -> Option<i128> {
        todo!()
    }
}

pub fn parse_day13(lines: &[String]) -> Vec<Machine> {
    let mut machines = Vec::new();
    let mut a: Option<Coord> = None;
    let mut b: Option<Coord> = None;
    let mut target: Option<Coord> = None;
    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            // End of a machine definition
            let button_a = a.expect("Button A not set");
            let button_b = b.expect("Button B not set");
            let prize = target.expect("Prize not set");
            machines.push(Machine {
                button_a,
                button_b,
                prize,
            });
            a = None;
            b = None;
            target = None;
            continue;
        }
        let (description, attributes) = line.split_once(':').expect("No colon found on line");
        match description {
            "Button A" => a = get_coords(attributes.trim(), '+'),
            "Button B" => b = get_coords(attributes.trim(), '+'),
            "Prize" => target = get_coords(attributes.trim(), '='),
            _ => panic!("unknown line description: '{description}'"),
        }
    }

    // Add the last machine
    let button_a = a.expect("Button A not set");
    let button_b = b.expect("Button B not set");
    let prize = target.expect("Prize not set");
    machines.push(Machine {
        button_a,
        button_b,
        prize,
    });

    machines
}

pub fn get_result_day13_stage1(machines: &[Machine]) -> i128 {
    machines
        .iter()
        .filter_map(|machine| machine.get_tokens())
        .sum()
}

pub fn get_result_day13_stage2(machines: &mut [Machine]) -> i128 {
    machines.iter_mut().map(|machine| {
        machine.prize.0 += 10000000000000;
        machine.prize.1 += 10000000000000;
    });

    machines
        .iter()
        .filter_map(|machine| machine.get_tokens_stage2())
        .sum()
}

fn get_coords(attrs: &str, delimiter: char) -> Option<Coord> {
    let (x_change, y_change) = attrs.split_once(", ")?;
    let (_, dx_str) = x_change.split_once(delimiter)?;
    let dx = dx_str.parse::<i128>().ok()?;
    let (_, dy_str) = y_change.split_once(delimiter)?;
    let dy = dy_str.parse::<i128>().ok()?;
    Some((dx, dy))
}

type Coord = (i128, i128);

#[cfg(test)]
mod day13 {
    use super::*;

    fn get_example_lines() -> Vec<String> {
        vec![
            String::from("Button A: X+94, Y+34"),
            String::from("Button B: X+22, Y+67"),
            String::from("Prize: X=8400, Y=5400"),
            String::from(""),
            String::from("Button A: X+26, Y+66"),
            String::from("Button B: X+67, Y+21"),
            String::from("Prize: X=12748, Y=12176"),
            String::from(""),
            String::from("Button A: X+17, Y+86"),
            String::from("Button B: X+84, Y+37"),
            String::from("Prize: X=7870, Y=6450"),
            String::from(""),
            String::from("Button A: X+69, Y+23"),
            String::from("Button B: X+27, Y+71"),
            String::from("Prize: X=18641, Y=10279"),
        ]
    }

    #[test]
    fn parse() {
        let lines = get_example_lines();
        let machines = parse_day13(&lines);
        assert_eq!(machines.len(), 4);
        let expected = [
            (94, 34, 22, 67, 8400, 5400),
            (26, 66, 67, 21, 12748, 12176),
            (17, 86, 84, 37, 7870, 6450),
            (69, 23, 27, 71, 18641, 10279),
        ];
        for (res, exp) in machines.iter().zip(expected.iter()) {
            assert_eq!(res.button_a.0, exp.0);
            assert_eq!(res.button_a.1, exp.1);
            assert_eq!(res.button_b.0, exp.2);
            assert_eq!(res.button_b.1, exp.3);
            assert_eq!(res.prize.0, exp.4);
            assert_eq!(res.prize.1, exp.5);
        }
    }

    #[test]
    fn stage1() {
        let machines = parse_day13(&get_example_lines());
        let result = get_result_day13_stage1(&machines);
        assert_eq!(result, 480);
    }
}
