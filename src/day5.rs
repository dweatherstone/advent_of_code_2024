use std::{
    cell::Cell,
    collections::{HashMap, HashSet, VecDeque},
};

#[derive(Debug, PartialEq)]
pub struct PageOrdering {
    first_page: u32,
    second_page: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Update {
    pages: Vec<u32>,
}

pub fn parse_day5(lines: &[String]) -> (Vec<PageOrdering>, Vec<Update>) {
    let mut ordering = Vec::new();
    let mut updates = Vec::new();
    let mut lines_iter = lines.iter();
    // ordering
    for line in lines_iter.by_ref() {
        // empty line separates ordering section from updates section
        if line.is_empty() {
            break;
        }
        let (first, second) = line
            .split_once('|')
            .expect("ordering section must have 2 pages");
        ordering.push(PageOrdering {
            first_page: first.parse().expect("invalid number"),
            second_page: second.parse().expect("invalid number"),
        });
    }

    // parse updates section
    for line in lines_iter {
        let pages: Vec<u32> = line
            .split(',')
            .map(|s| s.parse().expect("invalid page number"))
            .collect();
        updates.push(Update { pages });
    }

    (ordering, updates)
}

pub fn get_result_day5_stage1(ordering: &[PageOrdering], updates: &[Update]) -> u32 {
    // Build a rule map: for each page, which pages must come after it
    let mut rules_map: HashMap<u32, HashSet<u32>> = HashMap::new();
    for rule in ordering {
        rules_map
            .entry(rule.first_page)
            .or_default()
            .insert(rule.second_page);
    }

    updates
        .iter()
        .filter_map(|update| {
            if is_update_valid(&rules_map, update) {
                Some(update.pages[update.pages.len() / 2])
            } else {
                None
            }
        })
        .sum()
}

pub fn get_result_day5_stage2(ordering: &[PageOrdering], updates: &[Update]) -> u32 {
    // Build a rule map: for each page, which pages must come after it
    let mut rules_map: HashMap<u32, HashSet<u32>> = HashMap::new();
    for rule in ordering {
        rules_map
            .entry(rule.first_page)
            .or_default()
            .insert(rule.second_page);
    }

    updates
        .iter()
        .filter(|update| !is_update_valid(&rules_map, update))
        .map(|update| topo_sort_update(&rules_map, update))
        .map(|pages| pages[pages.len() / 2])
        .sum()
}

fn is_update_valid(rules: &HashMap<u32, HashSet<u32>>, update: &Update) -> bool {
    let index: HashMap<u32, usize> = update
        .pages
        .iter()
        .enumerate()
        .map(|(i, &p)| (p, i))
        .collect();

    for (&x, after_set) in rules {
        if let Some(&i_x) = index.get(&x) {
            for &y in after_set {
                if let Some(&i_y) = index.get(&y)
                    && i_x >= i_y
                {
                    return false;
                }
            }
        }
    }
    true
}

fn topo_sort_update(rules: &HashMap<u32, HashSet<u32>>, update: &Update) -> Vec<u32> {
    // Step 1: record which pages exist in this update
    let pages: HashSet<u32> = update.pages.iter().copied().collect();

    // Step 2: compute in-degree for each page in the update
    let mut in_degree: HashMap<u32, usize> = pages.iter().map(|&p| (p, 0)).collect();

    // For each rule X -> Y, if both are in this update, increase in-degree of Y
    for (&x, after_set) in rules {
        if pages.contains(&x) {
            for &y in after_set {
                if pages.contains(&y) {
                    *in_degree.get_mut(&y).unwrap() += 1;
                }
            }
        }
    }

    // Step 3: queue all pages that currently have in-degree 0
    let mut queue: VecDeque<u32> = in_degree
        .iter()
        .filter(|&(_, &deg)| deg == 0)
        .map(|(&p, _)| p)
        .collect::<VecDeque<_>>();

    // Step 4: repeatedly remove pages from the graph
    let mut sorted = Vec::new();

    while let Some(p) = queue.pop_front() {
        sorted.push(p);

        // Decrease in-degree neighbours (pages that must come after p)
        if let Some(neighbours) = rules.get(&p) {
            for &n in neighbours {
                if pages.contains(&n) {
                    let deg = in_degree.get_mut(&n).unwrap();
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push_back(n);
                    }
                }
            }
        }
    }
    sorted
}

#[cfg(test)]
mod day5 {
    use super::*;

    fn get_input_lines() -> Vec<String> {
        vec![
            String::from("47|53"),
            String::from("97|13"),
            String::from("97|61"),
            String::from("97|47"),
            String::from("75|29"),
            String::from("61|13"),
            String::from("75|53"),
            String::from("29|13"),
            String::from("97|29"),
            String::from("53|29"),
            String::from("61|53"),
            String::from("97|53"),
            String::from("61|29"),
            String::from("47|13"),
            String::from("75|47"),
            String::from("97|75"),
            String::from("47|61"),
            String::from("75|61"),
            String::from("47|29"),
            String::from("75|13"),
            String::from("53|13"),
            String::from(""),
            String::from("75,47,61,53,29"),
            String::from("97,61,53,29,13"),
            String::from("75,29,13"),
            String::from("75,97,47,61,53"),
            String::from("61,13,29"),
            String::from("97,13,75,29,47"),
        ]
    }

    #[test]
    fn day5_parse() {
        let (ordering, updates) = parse_day5(&get_input_lines());
        let expected_ordering = vec![
            PageOrdering {
                first_page: 47,
                second_page: 53,
            },
            PageOrdering {
                first_page: 97,
                second_page: 13,
            },
            PageOrdering {
                first_page: 97,
                second_page: 61,
            },
            PageOrdering {
                first_page: 97,
                second_page: 47,
            },
            PageOrdering {
                first_page: 75,
                second_page: 29,
            },
            PageOrdering {
                first_page: 61,
                second_page: 13,
            },
            PageOrdering {
                first_page: 75,
                second_page: 53,
            },
            PageOrdering {
                first_page: 29,
                second_page: 13,
            },
            PageOrdering {
                first_page: 97,
                second_page: 29,
            },
            PageOrdering {
                first_page: 53,
                second_page: 29,
            },
            PageOrdering {
                first_page: 61,
                second_page: 53,
            },
            PageOrdering {
                first_page: 97,
                second_page: 53,
            },
            PageOrdering {
                first_page: 61,
                second_page: 29,
            },
            PageOrdering {
                first_page: 47,
                second_page: 13,
            },
            PageOrdering {
                first_page: 75,
                second_page: 47,
            },
            PageOrdering {
                first_page: 97,
                second_page: 75,
            },
            PageOrdering {
                first_page: 47,
                second_page: 61,
            },
            PageOrdering {
                first_page: 75,
                second_page: 61,
            },
            PageOrdering {
                first_page: 47,
                second_page: 29,
            },
            PageOrdering {
                first_page: 75,
                second_page: 13,
            },
            PageOrdering {
                first_page: 53,
                second_page: 13,
            },
        ];
        assert_eq!(ordering.len(), expected_ordering.len());
        for (result, expected) in ordering.iter().zip(expected_ordering.iter()) {
            assert_eq!(result, expected);
        }
        let expected_updates = vec![
            Update {
                pages: vec![75, 47, 61, 53, 29],
            },
            Update {
                pages: vec![97, 61, 53, 29, 13],
            },
            Update {
                pages: vec![75, 29, 13],
            },
            Update {
                pages: vec![75, 97, 47, 61, 53],
            },
            Update {
                pages: vec![61, 13, 29],
            },
            Update {
                pages: vec![97, 13, 75, 29, 47],
            },
        ];
        assert_eq!(updates.len(), expected_updates.len());
        for (result, expected) in updates.iter().zip(expected_updates.iter()) {
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn day5_stage1() {
        let (ordering, updates) = parse_day5(&get_input_lines());
        let result_stage1 = get_result_day5_stage1(&ordering, &updates);
        assert_eq!(result_stage1, 143);
    }

    #[test]
    fn day5_stage2() {
        let (ordering, updates) = parse_day5(&get_input_lines());
        let result_stage2 = get_result_day5_stage2(&ordering, &updates);
        assert_eq!(result_stage2, 123);
    }
}
