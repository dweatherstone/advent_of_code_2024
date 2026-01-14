#![allow(unused)]

use std::{fs::read_to_string, path::Path};

use crate::{
    day1::{parse_day1, sum_of_differences, sum_of_similarity_score},
    day2::{parse_day2, safe_qty},
    day3::{mul_sum, mul_sum_conditional, parse_day3},
    day4::{count_x_mas, count_xmas, parse_day4},
    day5::{get_result_day5_stage1, get_result_day5_stage2, parse_day5},
    day6::parse_day6,
    day7::{get_result_day7_stage1, get_result_day7_stage2, parse_day7},
    day8::parse_day8,
    day9::{defrag, defrag_stage2, get_checksum, parse_day9},
    day10::parse_day10,
    day11::{number_of_stones, parse_day11},
    day13::{get_result_day13_stage1, get_result_day13_stage2, parse_day13},
};

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

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

fn run_day5() {
    let lines = get_lines(Path::new("input/day5_input.txt"));
    let (ordering, updates) = parse_day5(&lines);
    let result = get_result_day5_stage1(&ordering, &updates);
    println!("Result day 5 (stage 1): {result}");
    let result = get_result_day5_stage2(&ordering, &updates);
    println!("Result day 5 (stage 2): {result}");
}

fn run_day6() {
    let lines = get_lines(Path::new("input/day6_input.txt"));
    let mut map = parse_day6(&lines);
    let result = map.patrol();
    println!("Result day 6 (stage 1): {result}");
    let mut map = parse_day6(&lines);
    let result = map.count_loop_positions();
    println!("Result day 6 (stage 2): {result}");
}

fn run_day7() {
    let lines = get_lines(Path::new("input/day7_input.txt"));
    let equations = parse_day7(&lines);
    let result = get_result_day7_stage1(&equations);
    println!("Result day 7 (stage 1): {result}");
    let result = get_result_day7_stage2(&equations);
    println!("Result day 7 (stage 2): {result}");
}

fn run_day8() {
    let lines = get_lines(Path::new("input/day8_input.txt"));
    let map = parse_day8(&lines);
    let result = map.get_antinodes_stage1();
    println!("Result day 8 (stage 1): {result}");
    let result = map.get_antinodes_stage2();
    println!("Result day 8 (stage 2): {result}");
}

fn run_day9() {
    let lines = get_lines(Path::new("input/day9_input.txt"));
    let filesystem = parse_day9(&lines);
    // let defragged = defrag(&filesystem);
    // let checksum = get_checksum(&defragged);
    // println!("Result day 9 (stage 1): {checksum}");
    let defragged = defrag_stage2(&filesystem);
    let checksum = get_checksum(&defragged);
    println!("Result day 9 (stage 2): {checksum}");
}

fn run_day10() {
    let lines = get_lines(Path::new("input/day10_input.txt"));
    let map = parse_day10(&lines);
    let score = map.get_score_stage1();
    println!("Result day 10 (stage 1): {score}");
    let rating = map.get_rating_stage2();
    println!("Result day 10 (stage 2): {rating}");
}

fn run_day11() {
    let lines = get_lines(Path::new("input/day11_input.txt"));
    let stones = parse_day11(&lines);
    let stone_count = number_of_stones(&stones, 25);
    println!("Result day 11 (stage 1): {stone_count}");
    let stone_count = number_of_stones(&stones, 75);
    println!("Result day 11 (stage 2): {stone_count}");
}

fn run_day12() {
    let lines = get_lines(Path::new("input/day12_input.txt"));
    // let lines = vec![
    //     String::from("AAAA"),
    //     String::from("BBCD"),
    //     String::from("BBCC"),
    //     String::from("EEEC"),
    // ];
    let grid = day12::parse_grid(&lines);
    let regions = day12::find_regions(&grid);
    // for region in regions.iter() {
    //     println!("{region}");
    //     println!("Perimeter = {}", region.perimeter());
    // }
    let stage1 = day12::stage1_result(&regions);
    println!("Result day 12 (stage 1): {stage1}");
    let stage2 = day12::stage2_result(&regions);
    println!("Result day 12 (stage 2): {stage2}");
}

fn run_day13() {
    let mut machines = parse_day13(&get_lines(Path::new("input/day13_input.txt")));
    let stage1 = get_result_day13_stage1(&machines);
    println!("Result day 13 (stage 1): {stage1}");
    let stage2 = get_result_day13_stage2(&mut machines);
    println!("Result day 13 (stage 2): {stage2}");
}

fn main() {
    run_day13();
}
