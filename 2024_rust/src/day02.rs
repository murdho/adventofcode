const INPUT: &str = include_str!("../../2024_inputs/day02.txt");
// const INPUT: &str = include_str!("../../2024_inputs/day02_example.txt");

pub fn part_one() -> i32 {
    count_save_levels(parse_levels())
}

pub fn part_two() -> i32 {
    parse_levels()
        .into_iter()
        .filter(|levels| {
            let candidates = generate_candidates(levels);
            candidates.iter().any(are_levels_safe)
        })
        .count()
        .try_into()
        .unwrap()
}

fn parse_levels() -> Vec<Vec<i32>> {
    let parse_line = |line: &str| -> Vec<i32> {
        line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect()
    };

    INPUT.lines().map(parse_line).collect()
}

fn count_save_levels(levels: Vec<Vec<i32>>) -> i32 {
    levels
        .into_iter()
        .filter(are_levels_safe)
        .count()
        .try_into()
        .unwrap()
}

fn are_levels_safe(levels: &Vec<i32>) -> bool {
    let ascending = levels.windows(2).all(|pair| pair[0] < pair[1]);
    let descending = levels.windows(2).all(|pair| pair[0] > pair[1]);
    if !ascending && !descending {
        return false;
    }

    let diffs: Vec<i32> = levels
        .windows(2)
        .map(|pair| (pair[0] - pair[1]).abs())
        .collect();

    let min = *diffs.iter().min().unwrap();
    if min < 1 {
        return false;
    }

    let max = *diffs.iter().max().unwrap();
    if max > 3 {
        return false;
    }

    true
}

fn generate_candidates(levels: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut candidates: Vec<Vec<i32>> = Vec::with_capacity(levels.len());

    for skip_idx in 0..levels.len() {
        let mut combination = Vec::new();
        combination.extend_from_slice(&levels[..skip_idx]);
        combination.extend_from_slice(&levels[skip_idx + 1..]);
        candidates.push(combination);
    }

    candidates
}
