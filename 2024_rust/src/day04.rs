const INPUT: &str = include_str!("../../2024_inputs/day04.txt");
// const INPUT: &str = include_str!("../../2024_inputs/day04_example.txt");

pub fn part_one() -> i32 {
    let matrix = read_input();

    let max_rows = matrix.len();
    let max_cols = matrix[0].len();

    let mut result = 0;

    for row in 0..max_rows {
        for col in 0..max_cols {
            if matrix[row][col] == "X" {
                //
                // XMAS
                //
                //
                if col < max_cols - 3
                    && matrix[row][col + 1] == "M"
                    && matrix[row][col + 2] == "A"
                    && matrix[row][col + 3] == "S"
                {
                    result += 1;
                }

                // X
                //  M
                //   A
                //    S
                if row < max_rows - 3
                    && col < max_cols - 3
                    && matrix[row + 1][col + 1] == "M"
                    && matrix[row + 2][col + 2] == "A"
                    && matrix[row + 3][col + 3] == "S"
                {
                    result += 1;
                }

                // X
                // M
                // A
                // S
                if row < max_rows - 3
                    && matrix[row + 1][col] == "M"
                    && matrix[row + 2][col] == "A"
                    && matrix[row + 3][col] == "S"
                {
                    result += 1;
                }

                //    X
                //   M
                //  A
                // S
                if row < max_rows - 3
                    && col >= 3
                    && matrix[row + 1][col - 1] == "M"
                    && matrix[row + 2][col - 2] == "A"
                    && matrix[row + 3][col - 3] == "S"
                {
                    result += 1;
                }

                //
                // SAMX
                //
                //
                if col >= 3
                    && matrix[row][col - 1] == "M"
                    && matrix[row][col - 2] == "A"
                    && matrix[row][col - 3] == "S"
                {
                    result += 1;
                }

                // S
                //  A
                //   M
                //    X
                if row >= 3
                    && col >= 3
                    && matrix[row - 1][col - 1] == "M"
                    && matrix[row - 2][col - 2] == "A"
                    && matrix[row - 3][col - 3] == "S"
                {
                    result += 1;
                }

                // S
                // A
                // M
                // X
                if row >= 3
                    && matrix[row - 1][col] == "M"
                    && matrix[row - 2][col] == "A"
                    && matrix[row - 3][col] == "S"
                {
                    result += 1;
                }

                //    S
                //   A
                //  M
                // X
                if row >= 3
                    && col < max_cols - 3
                    && matrix[row - 1][col + 1] == "M"
                    && matrix[row - 2][col + 2] == "A"
                    && matrix[row - 3][col + 3] == "S"
                {
                    result += 1;
                }
            }
        }
    }

    result
}

fn read_input() -> Vec<Vec<String>> {
    INPUT
        .lines()
        .map(|line| line.chars().map(|s| s.to_string()).collect())
        .collect()
}

pub fn part_two() -> i32 {
    let matrix = read_input();

    let max_rows = matrix.len();
    let max_cols = matrix[0].len();

    let mut result = 0;

    for row in 0..max_rows {
        for col in 0..max_cols {
            if matrix[row][col] == "A" {
                // center cannot be on the edge, no point to check further
                if row == 0 || col == 0 || row == max_rows - 1 || col == max_cols - 1 {
                    continue;
                }

                let top_left = &matrix[row - 1][col - 1];
                let top_right = &matrix[row - 1][col + 1];
                let bottom_left = &matrix[row + 1][col - 1];
                let bottom_right = &matrix[row + 1][col + 1];

                // \
                let diagonal_desc = (top_left == "M" && bottom_right == "S")
                    || (top_left == "S" && bottom_right == "M");

                // /
                let diagonal_asc = (top_right == "M" && bottom_left == "S")
                    || (top_right == "S" && bottom_left == "M");

                if diagonal_desc && diagonal_asc {
                    result += 1;
                }
            }
        }
    }

    result
}
