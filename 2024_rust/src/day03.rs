use regex::Regex;

const INPUT: &str = include_str!("../../2024_inputs/day03.txt");
// const INPUT: &str = include_str!("../../2024_inputs/day03_example_part1.txt");
// const INPUT: &str = include_str!("../../2024_inputs/day03_example_part2.txt");

pub fn part_one() -> i32 {
    sum_mul_instructions(INPUT)
}

pub fn part_two() -> i32 {
    let re = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();
    let muls: String = re
        .captures_iter(INPUT)
        .map(|c| c.get(0).unwrap().as_str())
        .fold(vec![vec![]], |mut acc, s| {
            if s.starts_with("do") {
                acc.push(vec![]);
            }
            acc.last_mut().unwrap().push(s);
            acc
        })
        .into_iter()
        .filter(|group| *group.first().unwrap_or(&"") != "don't()")
        .flatten()
        .filter(|instr| instr.starts_with("mul"))
        .collect::<Vec<&str>>()
        .join("");

    sum_mul_instructions(muls.as_str())
}

fn sum_mul_instructions(instructions: &str) -> i32 {
    let re = Regex::new(r"mul\((?<a>\d+),(?<b>\d+)\)").unwrap();
    re.captures_iter(instructions)
        .map(|captures| {
            let a: i32 = captures["a"].parse().unwrap();
            let b: i32 = captures["b"].parse().unwrap();
            a * b
        })
        .sum()
}
