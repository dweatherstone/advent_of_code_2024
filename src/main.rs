#![allow(unused)]

use std::{fs::read_to_string, path::Path};

use crate::{
    day1::{parse_day1, sum_of_differences, sum_of_similarity_score},
    day2::{parse_day2, safe_qty},
    day3::{mul_sum, mul_sum_conditional, parse_day3},
    day4::{count_x_mas, count_xmas, parse_day4},
};

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

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

fn run_day3() {
    let lines = get_lines(Path::new("input/day3_input.txt"));
    let mul_stmts = parse_day3(&lines);
    let total_sum = mul_sum(&mul_stmts);
    println!("Total sum (stage 1): {total_sum}");
    let total_sum = mul_sum_conditional(&mul_stmts);
    println!("Total sum (stage 2): {total_sum}");
}

fn run_day4() {
    let lines = get_lines(Path::new("input/day4_input.txt"));
    let puzzle = parse_day4(&lines);
    let xmas_count = count_xmas(&puzzle);
    println!("Total number of 'XMAS' (stage 1): {xmas_count}");
    let x_mas_count = count_x_mas(&puzzle);
    println!("Total number of 'X-MAS' (stage 2): {x_mas_count}");
}

fn main() {
    run_day4();
}
