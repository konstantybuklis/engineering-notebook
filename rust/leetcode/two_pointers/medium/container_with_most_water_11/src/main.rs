#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut max_water = 0;

        while left < right {
            let result = (right as i32 - left as i32) * height[left].min(height[right]);
            max_water = max_water.max(result);

            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }

        max_water
    }
}

fn main() {}
