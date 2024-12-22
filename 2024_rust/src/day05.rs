use std::collections::HashSet;

const INPUT: &str = include_str!("../../2024_inputs/day05.txt");
// const INPUT: &str = include_str!("../../2024_inputs/day05_example.txt");

fn are_valid_pages_for_update(rules: &HashSet<(&str, &str)>, pages: &Vec<&str>) -> bool {
    for (i, left) in pages.iter().enumerate() {
        for right in pages[i..].iter() {
            if rules.contains(&(right, left)) {
                return false;
            }
        }
    }

    true
}

pub fn part_one() -> i32 {
    let input_parts: Vec<_> = INPUT.split("\n\n").collect();

    let page_ordering_rules: HashSet<(&str, &str)> =
        input_parts[0]
            .split("\n")
            .fold(HashSet::new(), |mut acc, rule| {
                let pages: Vec<_> = rule.split("|").collect();
                acc.insert((pages[0], pages[1]));
                acc
            });

    let pages_for_updates: Vec<_> = input_parts[1]
        .split("\n")
        .map(|pages| pages.split(",").collect::<Vec<_>>())
        .collect();

    let valid_updates: Vec<_> = pages_for_updates
        .into_iter()
        .filter(|pages| are_valid_pages_for_update(&page_ordering_rules, pages))
        .collect();

    valid_updates
        .iter()
        .map(|coll| coll[coll.len() / 2].parse::<i32>().unwrap())
        .sum()
}

pub fn part_two() -> i32 {
    -1
}
