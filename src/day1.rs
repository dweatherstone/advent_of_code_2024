use std::collections::HashMap;

pub fn parse_day1(lines: &[String]) -> (Vec<i32>, Vec<i32>) {
    let mut col1 = Vec::new();
    let mut col2 = Vec::new();
    for line in lines {
        let (str1, str2) = line.split_once(' ').unwrap();
        let val1 = str1
            .trim()
            .parse::<i32>()
            .expect("expected a number for str1");
        let val2 = str2
            .trim()
            .parse::<i32>()
            .expect("expected a number for str2");
        col1.push(val1);
        col2.push(val2);
    }
    col1.sort_unstable();
    col2.sort_unstable();

    (col1, col2)
}

pub fn sum_of_differences(list1: &[i32], list2: &[i32]) -> u32 {
    assert_eq!(list1.len(), list2.len());

    list1
        .iter()
        .zip(list2.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum()
}

pub fn sum_of_similarity_score(list1: &[i32], list2: &[i32]) -> u32 {
    assert_eq!(list1.len(), list2.len());

    let mut list2_counts: HashMap<i32, u32> = HashMap::new();
    for &val2 in list2.iter() {
        *list2_counts.entry(val2).or_insert(0) += 1;
    }

    list1
        .iter()
        .map(|&val1| val1 as u32 * list2_counts.get(&val1).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod day1 {
    use super::*;

    fn get_input_lines() -> Vec<String> {
        vec![
            String::from("3   4"),
            String::from("4   3"),
            String::from("2   5"),
            String::from("1   3"),
            String::from("3   9"),
            String::from("3   3"),
        ]
    }

    #[test]
    fn day1_parse() {
        let lines = get_input_lines();
        let (result1, result2) = parse_day1(&lines);
        let expected_list1 = vec![1, 2, 3, 3, 3, 4];
        let expected_list2 = vec![3, 3, 3, 4, 5, 9];
        assert_eq!(result1.len(), 6);
        assert_eq!(result2.len(), 6);
        for (res1, expect1) in result1.iter().zip(expected_list1.iter()) {
            assert_eq!(res1, expect1);
        }
        for (res2, expect2) in result2.iter().zip(expected_list2.iter()) {
            assert_eq!(res2, expect2);
        }
    }

    #[test]
    fn day1_stage1() {
        let (list1, list2) = parse_day1(&get_input_lines());
        let total_sum = sum_of_differences(&list1, &list2);
        assert_eq!(total_sum, 11);
    }

    #[test]
    fn day1_stage2() {
        let (list1, list2) = parse_day1(&get_input_lines());
        let similarity_score = sum_of_similarity_score(&list1, &list2);
        assert_eq!(similarity_score, 31);
    }
}
