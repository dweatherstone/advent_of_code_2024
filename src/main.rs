#![allow(unused)]

use std::{fs::read_to_string, path::Path};

use crate::{
    day1::{parse_day1, sum_of_differences, sum_of_similarity_score},
    day2::{parse_day2, safe_qty},
};

pub mod day1;
pub mod day2;

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

fn run_day2() {
    let lines = get_lines(Path::new("input/day2_input.txt"));
    let levels = parse_day2(&lines);
    let num_safe = safe_qty(&levels, true);
    println!("Quantity of safe levels (stage 1): {num_safe}");
    let num_safe = safe_qty(&levels, false);
    println!("Quantity of safe levels (stage 2): {num_safe}");
}

fn main() {
    run_day2();
}
