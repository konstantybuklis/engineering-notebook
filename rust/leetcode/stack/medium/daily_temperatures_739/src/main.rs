struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let n = temperatures.len();
        let mut stack: Vec<usize> = Vec::new();
        let mut results = vec![0; n];

        for i in (0..n).rev() {
            while let Some(&j) = stack.last() {
                if temperatures[j] <= temperatures[i] {
                    stack.pop();
                } else {
                    break;
                }
            }

            if let Some(&j) = stack.last() {
                results[i] = (j - i) as i32;
            }

            stack.push(i);
        }

        results
    }
}

fn main() {
    println!(
        "{:#?}",
        Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73])
    )
}
