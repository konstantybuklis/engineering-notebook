use std::collections::HashMap;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::with_capacity(nums.len());

        for num in &nums {
            map.entry(*num).and_modify(|freq| *freq += 1).or_insert(1);
        }

        let mut frequencies: Vec<(i32, usize)> = map.into_iter().collect();

        frequencies.sort_by(|(_, value_1), (_, value_2)| value_2.cmp(value_1));

        let mut results: Vec<i32> = frequencies.into_iter().map(|(value, _)| value).collect();

        results.drain(..k as usize).collect()
    }
}

fn main() {
    let v = vec![1, 2, 1, 2, 1, 2, 3, 1, 3, 2];
    let k = 3;

    let result = Solution::top_k_frequent(v, k);
    println!("{result:#?}");
}
