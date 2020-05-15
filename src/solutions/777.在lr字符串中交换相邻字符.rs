/*
 * @lc app=leetcode.cn id=777 lang=rust
 *
 * [777] 在LR字符串中交换相邻字符
 */

// @lc code=start
impl Solution {
    pub fn can_transform(start: String, end: String) -> bool {
        if !start.replace("X", "").eq(&end.replace("X", "")) {
            return false;
        }

        let len: usize = start.len();

        let mut index: usize = 0;
        let mut e_index: usize = 0;
        while index < len {
            if start.get(index..index + 1) == Some("L") {
                while e_index < len && end.get(e_index..e_index + 1) != Some("L") {
                    e_index = e_index + 1;
                }
                if index < e_index {
                    return false;
                }
                e_index += 1;
            }
            index += 1;
        }
        index = 0;
        e_index = 0;
        while index < len {
            if start.get(index..index + 1) == Some("R") {
                while e_index < len && end.get(e_index..e_index + 1) != Some("R") {
                    e_index = e_index + 1;
                }
                if index > e_index {
                    return false;
                }
                e_index += 1;
            }
            index += 1;
        }
        return true;
    }
}
// @lc code=end
