/*
 * @lc app=leetcode.cn id=852 lang=rust
 *
 * [852] 山脉数组的峰顶索引
 * 二分法
 */

// @lc code=start
impl Solution {
    pub fn peak_index_in_mountain_array(a: Vec<i32>) -> i32 {
        binary_mountain(&a,0,a.len()-1,a.len())
    }
}

fn binary_mountain(a: &Vec<i32>, start: usize, end: usize, len: usize) -> i32 {
    if start == end {
        if start == 0 || start == len - 1 {
            return -1;
        }
        if a[start] > a[start - 1] && a[start] > a[start + 1] {
            return start as i32;
        } else {
            return -1;
        }
    }

    let half = (start + end) / 2;
    if half == 0 || half == len - 1 {
        return -1;
    }
    if a[half] > a[half - 1] && a[half] > a[half + 1] {
        return half as i32;
    }
    let mut left: i32 = -1;
    let mut right: i32 = -1;
    if half > start {
        left = binary_mountain(a, start, half - 1, len);
        if (left > -1) {
            return left as i32;
        }
    }
    if left == -1 && half < end {
        right = binary_mountain(a, half + 1, end, len);
        if (right > -1) {
            return right as i32;
        }
    }
    return -1;
}
// @lc code=end
