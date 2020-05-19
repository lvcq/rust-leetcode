/*
 * @lc app=leetcode.cn id=798 lang=rust
 *
 * [798] 得分最高的最小轮调
 */

// @lc code=start
impl Solution {
    pub fn best_rotation(a: Vec<i32>) -> i32 {
        let len: usize = a.len();
        let mut break_point_vec: Vec<i32> = Vec::new();
        break_point_vec.resize(len, 0);
        let mut index: usize = 0;
        let mut res:usize = 0;
        while index < len {
            let b_index:usize = (index - (a[index] as usize) + 1 + len) % len;
            break_point_vec[b_index] = break_point_vec[b_index]+1;
            index = index + 1;
        }
        index = 1;
        while index < len {
            break_point_vec[index] = break_point_vec[index] + break_point_vec[index - 1] - 1;
            if break_point_vec[index] < break_point_vec[res] {
                res = index;
            }
            index = index + 1;
        }
        return (res as i32);
    }
}
// @lc code=end
