/*
 * @lc app=leetcode.cn id=779 lang=rust
 *
 * [779] 第K个语法符号
 */

// @lc code=start
impl Solution {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        return if half_tower(n, k, false) { 1 } else { 0 };
    }
}
fn half_tower(n: i32, k: i32, top: bool) -> bool {
    if n == 1 || k == 1 {
        return top;
    }

    if n == 2 && k == 2 {
        return !top;
    }
    let mut n_index: i32 = 1;
    let mut col_count: i32 = 1;
    while col_count < k {
        n_index = n_index + 1;
        col_count = col_count * 2;
    }
    return half_tower(n - 1, k - col_count / 2, !top);
}
// @lc code=end
