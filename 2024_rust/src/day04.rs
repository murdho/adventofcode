const INPUT: &str = include_str!("../../2024_inputs/day04.txt");
// const INPUT: &str = include_str!("../../2024_inputs/day04_example.txt");

pub fn part_one() -> i32 {
    let matrix: Vec<Vec<_>> = INPUT
        .lines()
        .map(|line| line.chars().map(|s| s.to_string()).collect())
        .collect();

    let max_rows = matrix.len();
    let max_cols = matrix.len();

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

pub fn part_two() -> i32 {
    -1
}
