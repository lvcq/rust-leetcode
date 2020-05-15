/*
 * @lc app=leetcode.cn id=780 lang=rust
 *
 * [780] 到达终点
 */

// @lc code=start
impl Solution {
    pub fn reaching_points(sx: i32, sy: i32, tx: i32, ty: i32) -> bool {
        let mut sx = sx;
        let mut sy = sy;
        let mut temp_tx = tx;
        let mut temp_ty = ty;
        while temp_tx >= sx && temp_ty >= sy {
            if temp_tx == temp_ty {
                break;
            }
            if temp_tx > temp_ty {
                if temp_ty > sy {
                    temp_tx = temp_tx % temp_ty;
                } else {
                    return (temp_tx - sx) % temp_ty == 0;
                }
            } else {
                if temp_tx > sx {
                    temp_ty = temp_ty % temp_tx;
                } else {
                    return return (temp_ty - sy) % temp_tx == 0;
                }
            }
        }
        return sx == temp_tx && sy == temp_ty;
    }
}
// @lc code=end
