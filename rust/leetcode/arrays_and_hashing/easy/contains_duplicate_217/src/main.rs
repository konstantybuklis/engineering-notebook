use std::collections::HashSet;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let set: HashSet<i32> = nums.clone().into_iter().collect();

        nums.len() != set.len()
    }
}

fn main() {}
