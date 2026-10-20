#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn product_except_self(mut nums: Vec<i32>) -> Vec<i32> {
        let mut zero_counts = 0;
        let mut product = 1;

        for num in &nums {
            if *num == 0 {
                zero_counts += 1;
            } else {
                product *= num;
            }
        }

        if zero_counts > 1 {
            return vec![0; nums.len()];
        }

        for num in nums.iter_mut() {
            if *num == 0 {
                *num = product;
                continue;
            }

            if zero_counts != 0 {
                *num = 0;
                continue;
            }

            *num = product / *num;
        }

        nums
    }
}

fn main() {}
