#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left = 0;
        let mut right = numbers.len() - 1;

        while left < right {
            if numbers[left] + numbers[right] == target {
                return vec![(left + 1) as i32, (right + 1) as i32];
            }

            if numbers[left] + numbers[right] < target {
                left += 1;
                continue;
            }

            if numbers[left] + numbers[right] > target {
                right -= 1;
                continue;
            }
        }

        vec![]
    }
}

fn main() {}
