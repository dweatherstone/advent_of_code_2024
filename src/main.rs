use std::{fs::read_to_string, path::Path};

use crate::day1::{parse_day1, sum_of_differences, sum_of_similarity_score};

pub mod day1;

fn get_lines(path: &Path) -> Vec<String> {
    read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn run_day1() {
    let lines = get_lines(Path::new("input/day1_input.txt"));
    let (a, b) = parse_day1(&lines);
    let total_sum = sum_of_differences(&a, &b);
    println!("Total sum of differences (stage 1) = {total_sum}");
    let similarity_score = sum_of_similarity_score(&a, &b);
    println!("Total sum of similarity score (stage 2) = {similarity_score}");
}

fn main() {
    run_day1();
}
