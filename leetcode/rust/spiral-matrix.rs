// https://leetcode.com/problems/spiral-matrix/

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.is_empty() {
            return vec![];
        }

        let mut result: Vec<i32> = vec![];

        let (mut row_start, mut row_end) = (0, matrix.len() - 1);
        let (mut col_start, mut col_end) = (0, matrix[0].len() - 1);

        while row_start <= row_end && col_start <= col_end {
            for col in col_start..=col_end {
                result.push(matrix[row_start][col]);
            }

            #[allow(clippy::needless_range_loop)]
            for row in (row_start + 1)..=row_end {
                result.push(matrix[row][col_end]);
            }

            for col in (col_start..col_end).rev() {
                if row_start == row_end {
                    break;
                }
                result.push(matrix[row_end][col]);
            }

            for row in (row_start + 1..row_end).rev() {
                if col_start == col_end {
                    break;
                }
                result.push(matrix[row][col_start]);
            }

            if row_end != 0 && col_end != 0 {
                row_start += 1;
                row_end -= 1;

                col_start += 1;
                col_end -= 1;
            } else {
                break;
            }
        }

        result
    }
}
