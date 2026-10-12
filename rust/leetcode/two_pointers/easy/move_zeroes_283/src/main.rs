#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut slow = 0;
        let mut fast = 0;

        while fast < nums.len() {
            if nums[fast] != 0 {
                nums.swap(slow, fast);
                slow += 1;
            }

            fast += 1;
        }
    }
}

fn main() {}
