use std::collections::HashSet;

// const INPUT: &str = include_str!("../../2024_inputs/day05.txt");
const INPUT: &str = include_str!("../../2024_inputs/day05_example.txt");

type Rules<'a> = HashSet<(&'a str, &'a str)>;

type Update<'a> = Vec<&'a str>;
type Updates<'a> = Vec<Update<'a>>;

pub fn part_one() -> i32 {
    let (rules, updates) = read_input();

    let valid_updates: Vec<_> = updates
        .into_iter()
        .filter(|pages| is_valid_update(&rules, pages))
        .collect();

    sum_centers(valid_updates)
}

pub fn part_two() -> i32 {
    let (rules, updates) = read_input();

    let invalid_updates: Updates = updates
        .into_iter()
        .filter(|update| !is_valid_update(&rules, update))
        .map(|update| reorder_update(&rules, update))
        .collect();

    sum_centers(invalid_updates)
}

fn read_input<'a>() -> (Rules<'a>, Updates<'a>) {
    let input_parts: Vec<_> = INPUT.split("\n\n").collect();

    let rules: Rules = input_parts[0]
        .split("\n")
        .fold(HashSet::new(), |mut acc, rule| {
            let pages: Vec<_> = rule.split("|").collect();
            acc.insert((pages[0], pages[1]));
            acc
        });

    let updates: Updates = input_parts[1]
        .split("\n")
        .map(|pages| pages.split(",").collect::<Update>())
        .collect();

    (rules, updates)
}

fn is_valid_update(rules: &Rules, update: &Update) -> bool {
    !update.iter().enumerate().any(|(i, left)| {
        update[i..]
            .iter()
            .any(|right| rules.contains(&(right, left)))
    })
}

fn reorder_update<'a>(rules: &Rules<'a>, mut update: Update<'a>) -> Update<'a> {
    update.sort_by(|a, b| match rules.contains(&(a, b)) {
        true => std::cmp::Ordering::Less,
        false => std::cmp::Ordering::Greater,
    });
    update
}

fn sum_centers(updates: Updates) -> i32 {
    updates
        .iter()
        .map(|coll| coll[coll.len() / 2].parse::<i32>().unwrap())
        .sum()
}
