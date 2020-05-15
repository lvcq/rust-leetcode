/*
 * @lc app=leetcode.cn id=748 lang=rust
 *
 * [748] 最短完整词
 */

// @lc code=start
impl Solution {
    pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
        use std::collections::HashMap;
        let mut char_cache: HashMap<char, usize> = HashMap::new();
        let mut chars = license_plate.chars();
        let mut char_count: usize = 0;
        let mut target_word: String = "".to_string();
        let mut reach: bool = false;
        // 将目标字符信息存储到hashMap中
        while let Some(curr) = chars.next() {
            let curr_low = curr.to_ascii_lowercase();
            if curr_low.is_alphabetic() {
                if char_cache.contains_key(&curr_low) {
                    char_cache.insert(curr_low, char_cache.get(&curr_low).unwrap() + 1);
                } else {
                    char_cache.insert(curr_low, 1);
                    char_count += 1;
                }
            }
        }

        /// 循环判断单词是否是完成词
        for word in words.iter() {
            let mut temp_cache: HashMap<char, usize> = HashMap::new();
            let mut matched_vec: Vec<char> = Vec::new();
            let mut w_chars = word.chars();
            while let Some(letter) = w_chars.next() {
                if matched_vec.contains(&letter) || !char_cache.contains_key(&letter) {
                    continue;
                }
                if !temp_cache.contains_key(&letter) {
                    temp_cache.insert(letter.clone(), 0);
                }

                if temp_cache.get(&letter).unwrap() + 1 == char_cache.get(&letter).unwrap().clone() {
                    matched_vec.push(letter);
                    if matched_vec.len() == char_count {
                        break;
                    }
                } else {
                    temp_cache.insert(letter, temp_cache.get(&letter).unwrap() + 1);
                }
            }
            if matched_vec.len() == char_count {
                if !reach {
                    target_word = word.clone();
                    reach = true;
                } else if word.len() < target_word.len() {
                    target_word = word.clone();
                }
            }
        }
        target_word
    }
}
// @lc code=end
