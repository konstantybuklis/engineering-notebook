#[allow(dead_code)]

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_operations(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let mut counter = 0;

        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right {
            if (nums[left] + nums[right]) as i32 == k {
                counter += 1;
                left += 1;
                right -= 1;

                continue;
            }

            if ((nums[left] + nums[right]) as i32) < k {
                left += 1;

                continue;
            }

            if ((nums[left] + nums[right]) as i32) > k {
                right -= 1;

                continue;
            }
        }

        counter
    }
}

fn main() {
    println!("{}", Solution::max_operations(vec![1, 2, 3, 4], 5));
}
