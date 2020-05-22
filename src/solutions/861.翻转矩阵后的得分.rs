/*
 * @lc app=leetcode.cn id=861 lang=rust
 *
 * [861] 翻转矩阵后的得分
 */

// @lc code=start
impl Solution {
    pub fn matrix_score(a: Vec<Vec<i32>>) -> i32 {
        let row: usize = a.len();
        let col: usize = a[0].len();
        let mut col_index: usize = 0;
        let mut row_index: usize = 0;
        let mut res: i32 = 0;
        while col_index < col {
            row_index = 0;
            let mut col_count: i32 = 0;
            while row_index < row {
                let xor = a[row_index][col_index] ^ a[row_index][0];
                col_count = col_count + xor;
                row_index = row_index + 1;
            }
            let max_num = if col_count > row as i32 - col_count {
                col_count
            } else {
                row as i32 - col_count
            };
            res += max_num * (1 << col - 1 - col_index);
            col_index = col_index + 1;
        }
        return res;
    }
}
// @lc code=end
