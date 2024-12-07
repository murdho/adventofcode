const INPUT: &'static str = include_str!("../../2024_inputs/day01.txt");
// const INPUT: &'static str = include_str!("../../2024_inputs/day01_example.txt");

pub fn part_one() -> i32 {
    let numbers: Vec<Vec<i32>> = INPUT
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<_>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut transposed = vec![Vec::with_capacity(numbers.len()); numbers[0].len()];
    for row in numbers {
        for (i, elem) in row.into_iter().enumerate() {
            transposed[i].push(elem);
        }
    }

    transposed[0].sort_unstable();
    transposed[1].sort_unstable();

    let sum = &transposed[0]
        .iter()
        .zip(&transposed[1])
        .map(|(a, b)| (a - b).abs())
        .sum::<i32>();

    *sum
}
