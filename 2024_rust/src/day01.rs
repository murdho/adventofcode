use std::collections::HashMap;

const INPUT: &'static str = include_str!("../../2024_inputs/day01.txt");
// const INPUT: &'static str = include_str!("../../2024_inputs/day01_example.txt");

pub fn part_one() -> i32 {
    let (mut left, mut right) = read_lists();

    left.sort_unstable();
    right.sort_unstable();

    left.iter().zip(right).map(|(a, b)| (a - b).abs()).sum()
}

pub fn part_two() -> i32 {
    let (left, right) = read_lists();

    let tallies: HashMap<_, _> = right.iter().fold(HashMap::new(), |mut acc, num| {
        *acc.entry(num).or_insert(0) += 1;
        acc
    });

    left.iter()
        .map(|num| num * tallies.get(num).unwrap_or(&0))
        .sum()
}

fn read_lists() -> (Vec<i32>, Vec<i32>) {
    let parse_line = |line: &str| -> Vec<i32> {
        line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect()
    };

    let numbers: Vec<Vec<i32>> = INPUT.lines().map(parse_line).collect();

    let mut transposed = vec![Vec::with_capacity(numbers.len()); numbers[0].len()];
    for row in numbers {
        for (i, elem) in row.into_iter().enumerate() {
            transposed[i].push(elem);
        }
    }

    (transposed.remove(0), transposed.remove(0))
}
