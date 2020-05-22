/*
 * @lc app=leetcode.cn id=857 lang=rust
 *
 * [857] 雇佣 K 名工人的最低成本
 */

// @lc code=start
use std::fmt::Debug;

impl Solution {
    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        let len: usize = quality.len();
        let mut worker_vec: Vec<Worker> = Vec::new();
        let mut index: usize = 0;
        while index < len {
            worker_vec.push(Worker::new(quality[index], wage[index]));
            index = index + 1;
        }
        worker_vec.sort_by(|a, b| a.unit_cost.partial_cmp(&b.unit_cost).unwrap());
        let mut quality_heap: MaxHeap<i32> = MaxHeap::new(-1);
        index = 0;
        let mut qsum: i32 = 0;
        let mut ans: f64 = 1.0e9;
        while index < len {
            quality_heap.push(worker_vec[index].quality);
            qsum = qsum + worker_vec[index].quality;

            if quality_heap.size() > k as usize {
                let pop_num = quality_heap.pop();

                qsum = qsum - pop_num;
            }
            if quality_heap.size() == k as usize {
                let n_count: f64 = worker_vec[index].unit_cost * (qsum as f64);
                ans = if n_count < ans { n_count } else { ans };
            }
            index = index + 1;
        }
        return ans;
    }
}

struct Worker {
    pub quality: i32,
    pub wage: i32,
    pub unit_cost: f64,
}

impl Worker {
    pub fn new(quality: i32, wage: i32) -> Self {
        let unit_cost = (wage as f64) / (quality as f64);
        Worker {
            quality,
            wage,
            unit_cost,
        }
    }
}

struct MaxHeap<T>
where
    T: PartialEq + PartialOrd + Copy + Debug,
{
    ele: Vec<T>,
    default: T,
}

impl<T> MaxHeap<T>
where
    T: PartialEq + PartialOrd + Copy + Debug,
{
    pub fn new(default: T) -> Self {
        MaxHeap {
            ele: Vec::new(),
            default,
        }
    }

    pub fn size(&self) -> usize {
        self.ele.len()
    }

    pub fn push(&mut self, quality: T) {
        self.ele.push(quality);
        let len = self.ele.len();
        let mut index: usize = len - 1;
        while index > 0 {
            let p_index = (index - 1) / 2;
            if self.ele[p_index] < self.ele[index] {
                let temp = self.ele[index];
                self.ele[index] = self.ele[p_index];
                self.ele[p_index] = temp;
            } else {
                break;
            }
            index = p_index;
        }
    }

    pub fn pop(&mut self) -> T {
        if self.ele.len() > 0 {
            let res = self.ele[0];
            let last = self.ele.pop().unwrap();
            self.ele[0] = last;
            let mut index: usize = 0;
            let len = self.ele.len();
            while index < len {
                if 2 * index + 1 >= len {
                    break;
                }
                let mut next = 2 * index + 1;
                if 2 * index + 2 < len && self.ele[2 * index + 1] < self.ele[2 * index + 2] {
                    next = 2 * index + 2;
                }
                if self.ele[index] < self.ele[next] {
                    let temp = self.ele[next];
                    self.ele[next] = self.ele[index];
                    self.ele[index] = temp;
                    index = next;
                } else {
                    break;
                }
            }
            return res;
        }
        return self.default;
    }
}

// @lc code=end
//[14,56,59,89,39,26,86,76,3,36]\n[90,217,301,202,294,445,473,245,415,487]\n2
