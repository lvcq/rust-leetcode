/*
 * @lc app=leetcode.cn id=808 lang=rust
 *
 * [808] 分汤
 */

// @lc code=start
impl Solution {
    pub fn soup_servings(n: i32) -> f64 {
        if n >= 25 * 500 {
            return 1.0;
        }
        let N: i32 = if n % 25 == 0 { n / 25 } else { n / 25 + 1 };
        let mut dp: Vec<Vec<f64>> = Vec::new();
        let mut dpr: Vec<f64> = Vec::new();
        dpr.resize((N + 1) as usize, 0.0);
        dp.resize((N + 1) as usize, dpr);
        let mut rs: usize = 0;
        let mut cs: usize = 0;
        while rs <= N as usize {
            cs = 0;
            while cs <= N as usize {
                let mut ans: f64 = 0.0;
                for (i, j) in [
                    (rs as i32 - 4, cs as i32),
                    (rs as i32 - 3, cs as i32 - 1),
                    (rs as i32 - 2, cs as i32 - 2),
                    (rs as i32 - 1, cs as i32 - 3),
                ]
                .iter()
                {
                    if i > &0 && j > &0 {
                        ans = ans + dp[*i as usize][*j as usize];
                    } else if i <= &0 {
                        let tem: f64 = if j <= &0 { 0.5 } else { 1.0 };
                        ans = ans + tem;
                    }
                }
                dp[rs][cs] = 0.25 * ans;
                cs += 1;
            }
            rs += 1;
        }
        return dp[N as usize][N as usize];
    }
}
// @lc code=end
