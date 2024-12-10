const INPUT: &str = include_str!("../../2024_inputs/day04.txt");
// const INPUT: &str = include_str!("../../2024_inputs/day04_example.txt");

pub fn part_one() -> i32 {
    let m: Vec<Vec<_>> = INPUT
        .lines()
        .map(|line| line.chars().map(|s| s.to_string()).collect::<Vec<_>>())
        .collect();

    let max_rows = m.len();
    let max_cols = m.len();

    let mut result = 0;

    for i in 0..max_rows {
        for j in 0..max_cols {
            if m[i][j] == "X" {
                //
                // XMAS
                //
                //
                if j < max_cols - 3
                    && m[i][j + 1] == "M"
                    && m[i][j + 2] == "A"
                    && m[i][j + 3] == "S"
                {
                    result += 1;
                }

                // X
                //  M
                //   A
                //    S
                if i < max_rows - 3
                    && j < max_cols - 3
                    && m[i + 1][j + 1] == "M"
                    && m[i + 2][j + 2] == "A"
                    && m[i + 3][j + 3] == "S"
                {
                    result += 1;
                }

                // X
                // M
                // A
                // S
                if i < max_rows - 3
                    && m[i + 1][j] == "M"
                    && m[i + 2][j] == "A"
                    && m[i + 3][j] == "S"
                {
                    result += 1;
                }

                //    X
                //   M
                //  A
                // S
                if i < max_rows - 3
                    && j >= 3
                    && m[i + 1][j - 1] == "M"
                    && m[i + 2][j - 2] == "A"
                    && m[i + 3][j - 3] == "S"
                {
                    result += 1;
                }

                //
                // SAMX
                //
                //
                if j >= 3 && m[i][j - 1] == "M" && m[i][j - 2] == "A" && m[i][j - 3] == "S" {
                    result += 1;
                }

                // S
                //  A
                //   M
                //    X
                if i >= 3
                    && j >= 3
                    && m[i - 1][j - 1] == "M"
                    && m[i - 2][j - 2] == "A"
                    && m[i - 3][j - 3] == "S"
                {
                    result += 1;
                }

                // S
                // A
                // M
                // X
                if i >= 3 && m[i - 1][j] == "M" && m[i - 2][j] == "A" && m[i - 3][j] == "S" {
                    result += 1;
                }

                //    S
                //   A
                //  M
                // X
                if i >= 3
                    && j < max_cols - 3
                    && m[i - 1][j + 1] == "M"
                    && m[i - 2][j + 2] == "A"
                    && m[i - 3][j + 3] == "S"
                {
                    result += 1;
                }
            }
        }
    }

    result
}

pub fn part_two() -> i32 {
    -1
}
