// const INPUT: &'static str = include_str!("../../2024_inputs/day02.txt");
const INPUT: &'static str = include_str!("../../2024_inputs/day02_example.txt");

pub fn part_one() -> i32 {
    let parse_line = |line: &str| -> Vec<i32> {
        line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect()
    };

    INPUT
        .lines()
        .map(parse_line)
        .collect::<Vec<Vec<i32>>>()
        .into_iter()
        .filter(|levels| {
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
            let max = *diffs.iter().max().unwrap();
            if min < 1 || max > 3 {
                return false;
            }

            true
        })
        .count()
        .try_into()
        .unwrap()
}
