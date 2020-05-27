/*
 * @lc app=leetcode.cn id=870 lang=rust
 *
 * [870] 优势洗牌
 */

// @lc code=start
impl Solution {
    pub fn advantage_count(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut a_vec=a.clone();
        let mut b_vec=b.clone();
        a_vec.sort();
        let len = a.len();
        let mut index :usize = 0;
        while index < len{

            index =index + 1;
        }
    }
}

fn find_first_larger_num(mut a:&Vec<i32>, target:i32)->Option<i32>{
    
}

fn half_search(a:&Vec<i32>,begin:usize,end:usize,target:i32)->Option<usize>{
    if target<=a[begin] || target>=a[end]{
        return None;
    }
    if begin==end{
        if a[begin]>target{
            return Some(begin);
        }else {
            return None
        }
    }
    let half = (begin+end)/2;
    if a[half]<=target {
        return half_search(a,half+1,end,target);
    } else if half==begin{
        return Some(begin);
    } else 
}
// @lc code=end

