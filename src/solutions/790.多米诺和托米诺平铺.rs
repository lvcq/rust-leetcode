/*
 * @lc app=leetcode.cn id=790 lang=rust
 *
 * [790] 多米诺和托米诺平铺
 */

// @lc code=start
impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }
        if n == 2 {
            return 2;
        }
        let mut num_n: i64 = 0;
        let mut num_n_sub_1: i64 = 2;
        let mut num_n_sub_2: i64 = 1;
        let mut num_sum: i64 = 1;
        let mut index: i32 = 3;
        let mod_num:i64 = 1000000007;
        while index <= n {
            num_n = (num_n_sub_1 + num_n_sub_2 + 2 * num_sum)%mod_num;
            num_sum = num_sum + num_n_sub_2;
            num_n_sub_2 = num_n_sub_1;
            num_n_sub_1 = num_n;
            index = index + 1;
        }
        return num_n as i32;
    }
}
// @lc code=end
