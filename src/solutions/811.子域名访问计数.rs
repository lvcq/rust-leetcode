/*
 * @lc app=leetcode.cn id=811 lang=rust
 *
 * [811] 子域名访问计数
 */

// @lc code=start
impl Solution {
    pub fn subdomain_visits(cpdomains: Vec<String>) -> Vec<String> {
        use std::collections::HashMap;
        let mut count_map: HashMap<String, i32> = HashMap::new();
        let mut res: Vec<String> = Vec::new();
        for input in cpdomains.iter() {
            let space_point = input.find(char::is_whitespace).unwrap();
            let count: i32 = input[..space_point].parse().unwrap();
            let mut domain: &str = &input[space_point + 1..];
            while domain.len() > 0 {
                if count_map.contains_key(domain) {
                    count_map.insert(domain.to_string(), count_map.get(domain).unwrap() + count);
                } else {
                    count_map.insert(domain.to_string(), count);
                }
                match domain.find('.') {
                    Some(dot_point) => {
                        domain = &domain[dot_point + 1..];
                    }
                    None => {
                        domain = "";
                    }
                }
            }
        }
        for key in count_map.keys() {
            res.push(format!("{} {}", count_map.get(key).unwrap(), key));
        }
        return res;
    }
}
// @lc code=end
