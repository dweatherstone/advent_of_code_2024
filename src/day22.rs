pub struct Market {
    traders: Vec<u64>,
}

pub fn parse_day22(lines: &[String]) -> Market {
    let traders = lines
        .iter()
        .map(|line| line.trim().parse::<u64>().expect("Should be an integer"))
        .collect();
    Market { traders }
}

pub fn result_stage1_day22(market: &Market) -> u64 {
    let n = 2000;
    market
        .traders
        .iter()
        .map(|&t| get_secret_number(t, n))
        .sum()
}

pub fn result_stage2_day22(market: &Market) -> u32 {
    // 19 ^ 4 possible sequences
    let mut total_bananas = vec![0u32; 19usize.pow(4)];
    // Track which trader last updated a sequence index
    let mut seen_by_trader = vec![-1i32; 19usize.pow(4)];

    for (trader_id, &start_val) in market.traders.iter().enumerate() {
        let mut val = start_val;
        let mut prev_price = (val % 10) as i8;

        // Circular buffer or simple variables to track the last 4 changes
        let (mut c1, mut c2, mut c3, mut c4) = (0, 0, 0, 0);

        for i in 0..2000 {
            val = get_next(val);
            let price = (val % 10) as i8;
            let diff = price - prev_price;
            prev_price = price;

            // Shift changes
            c1 = c2;
            c2 = c3;
            c3 = c4;
            c4 = diff;

            if i >= 3 {
                // Map sequence to 0..130321
                let idx = ((c1 + 9) as usize * 19usize.pow(3))
                    + ((c2 + 9) as usize * 19usize.pow(2))
                    + ((c3 + 9) as usize * 19)
                    + ((c4 + 9) as usize);

                // Only count the FIRST time this trader sees this sequence
                if seen_by_trader[idx] != trader_id as i32 {
                    total_bananas[idx] += price as u32;
                    seen_by_trader[idx] = trader_id as i32;
                }
            }
        }
    }
    *total_bananas.iter().max().unwrap()
}

fn get_secret_number(start: u64, n: u32) -> u64 {
    let mut val = start;
    (0..n).for_each(|_| val = get_next(val));
    val
}

fn get_next(from: u64) -> u64 {
    let mut val = from;
    const MASK: u64 = 0xFFFFFF; // 16777216 - 1

    val = (val ^ (val << 6)) & MASK;
    val = (val ^ (val >> 5)) & MASK;
    val = (val ^ (val << 11)) & MASK;

    val
}

#[cfg(test)]
mod day22 {
    use super::*;

    fn get_example1() -> Vec<String> {
        vec![
            String::from("1"),
            String::from("10"),
            String::from("100"),
            String::from("2024"),
        ]
    }

    fn get_example2() -> Vec<String> {
        vec![
            String::from("1"),
            String::from("2"),
            String::from("3"),
            String::from("2024"),
        ]
    }

    #[test]
    fn secret_numbers() {
        let market = parse_day22(&get_example1());
        let expected = [8685429, 4700978, 15273692, 8667524];
        assert_eq!(market.traders.len(), expected.len());
        for (&m, &exp) in market.traders.iter().zip(expected.iter()) {
            let result = get_secret_number(m, 2000);
            assert_eq!(result, exp);
        }
    }

    #[test]
    fn day22_stage1() {
        let market = parse_day22(&get_example1());
        let result = result_stage1_day22(&market);
        assert_eq!(result, 37327623);
    }

    #[test]
    fn day22_stage2() {
        let mut market = parse_day22(&get_example2());
        let result = result_stage2_day22(&market);
        assert_eq!(result, 23);
    }
}
