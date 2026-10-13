#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut nums = nums.clone();
        nums.sort();

        let mut result = Vec::new();

        for accum_first in 0..nums.len() {
            for accum_second in accum_first + 1..nums.len() {
                let mut left = accum_second + 1;
                let mut right = nums.len() - 1;

                while left < right {
                    if nums[accum_first]
                        .saturating_add(nums[accum_second])
                        .saturating_add(nums[left])
                        .saturating_add(nums[right])
                        == target
                    {
                        if !result.contains(&vec![
                            nums[accum_first],
                            nums[accum_second],
                            nums[left],
                            nums[right],
                        ]) {
                            result.push(vec![
                                nums[accum_first],
                                nums[accum_second],
                                nums[left],
                                nums[right],
                            ]);
                        }

                        left += 1;
                        right -= 1;
                    }

                    if nums[accum_first]
                        .saturating_add(nums[accum_second])
                        .saturating_add(nums[left])
                        .saturating_add(nums[right])
                        < target
                    {
                        left += 1;
                    }

                    if nums[accum_first]
                        .saturating_add(nums[accum_second])
                        .saturating_add(nums[left])
                        .saturating_add(nums[right])
                        > target
                    {
                        right -= 1;
                    }
                }
            }
        }

        result
    }
}

fn main() {}
