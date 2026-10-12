use std::collections::HashSet;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();

        let mut nums: Vec<i32> = nums.clone();
        nums.sort();

        for accum in 0..nums.len() {
            let mut left = accum + 1;
            let mut right = nums.len() - 1;

            while left < right {
                if nums[left] + nums[right] + nums[accum] == 0 {
                    result.push(vec![nums[left], nums[right], nums[accum as usize]]);

                    left += 1;
                    right -= 1;
                }

                if nums[left] + nums[right] + nums[accum] > 0 {
                    right -= 1;
                }

                if nums[left] + nums[right] + nums[accum] < 0 {
                    left += 1;
                }
            }
        }

        let set: HashSet<_> = result.drain(..).collect();
        result.extend(set.into_iter());

        result
    }
}

fn main() {}
