/*
 * @lc app=leetcode.cn id=756 lang=rust
 *
 * [756] 金字塔转换矩阵
 * 动态规划
 * 第一步，底为第一个字符时，能构建金字塔
 * 第二步，底添加一个字符，如果能找到第一字符与第二个字符的转换，则能构成金字塔
 * 第N步，。。。
 *
 */

// @lc code=start
impl Solution {
    pub fn pyramid_transition(bottom: String, allowed: Vec<String>) -> bool {
        let mut temp_vec: Vec<Vec<char>> = Vec::new();
        let bottom_vec: Vec<char> = bottom.chars().map(|letter| letter).collect();
        let allowed_truple: Vec<(char, char, char)> = allowed
            .iter()
            .map(|st| {
                let mut allowed_chars = st.chars();
                (
                    allowed_chars.next().unwrap(),
                    allowed_chars.next().unwrap(),
                    allowed_chars.next().unwrap(),
                )
            })
            .collect();
        let bottom_len: usize = bottom.len();
        if bottom_len == 1 {
            return true;
        }
        temp_vec.push(vec![bottom_vec.get(0).unwrap().clone()]);

        let mut index: usize = 1;
        while index < bottom_len {
            let next_char = bottom_vec.get(index).unwrap().clone();
            temp_vec = get_pyramid_right_side(temp_vec, next_char, &allowed_truple);
            if temp_vec.len() < index + 1 {
                return false;
            }
            index = index + 1;
        }
        return true;
    }
}

fn get_pyramid_right_side(
    origin: Vec<Vec<char>>,
    n_char: char,
    allowed: &Vec<(char, char, char)>,
) -> Vec<Vec<char>> {
    let mut new_temp_vec: Vec<Vec<char>> = Vec::new();
    new_temp_vec.push(vec![n_char]);
    let old_layer_count = origin.len();
    let mut index: usize = 0;
    while index < old_layer_count {
        let side_vec = origin.get(index).unwrap();
        let n_side = new_temp_vec.get(index).unwrap();
        let next = get_probably_up_side(side_vec, n_side, allowed);
        if next.is_empty() {
            break;
        }
        new_temp_vec.push(next);
        index += 1;
    }

    new_temp_vec
}

fn get_probably_up_side(
    left_side: &Vec<char>,
    right_side: &Vec<char>,
    allowed: &Vec<(char, char, char)>,
) -> Vec<char> {
    let mut n_side: Vec<char> = Vec::new();

    for l_c in left_side.iter() {
        for r_c in right_side.iter() {
            let up_chars: Vec<char> = get_up_chars(l_c, r_c, allowed);
            for n_c in up_chars.iter() {
                if !n_side.contains(&n_c) {
                    n_side.push(n_c.clone());
                }
            }
        }
    }

    n_side
}

fn get_up_chars(left: &char, right: &char, allowed: &Vec<(char, char, char)>) -> Vec<char> {
    let mut res: Vec<char> = Vec::new();
    for char_truple in allowed.iter() {
        if char_truple.0 == *left && char_truple.1 == *right {
            res.push(char_truple.2);
        }
    }
    return res;
}

// @lc code=end
// "BDBBAA"\n["ACB","ACA","AAA","ACD","BCD","BAA","BCB","BCC","BAD","BAB","BAC","CAB","CCD","CAA","CCA","CCC","CAD","DAD","DAA","DAC","DCD","DCC","DCA","DDD","BDD","ABB","ABC","ABD","BDB","BBD","BBC","BBA","ADD","ADC","ADA","DDC","DDB","DDA","CDA","CDD","CBA","CDB","CBD","DBA","DBC","DBD","BDA"]
