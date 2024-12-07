const INPUT: &'static str = include_str!("../../2024_inputs/day01.txt");
// const INPUT: &'static str = include_str!("../../2024_inputs/day01_example.txt");

pub fn part_one() -> i32 {
    let parse_numbers = |line: &str| -> Vec<i32> {
        line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect()
    };

    let numbers: Vec<Vec<i32>> = INPUT.lines().map(parse_numbers).collect();

    let mut transposed = vec![Vec::with_capacity(numbers.len()); numbers[0].len()];
    for row in numbers {
        for (i, elem) in row.into_iter().enumerate() {
            transposed[i].push(elem);
        }
    }

    let [mut left, mut right] = [transposed.remove(0), transposed.remove(0)];

    left.sort_unstable();
    right.sort_unstable();

    left.iter().zip(right).map(|(a, b)| (a - b).abs()).sum()
}
