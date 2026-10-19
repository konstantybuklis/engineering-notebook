use std::collections::HashMap;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<Vec<char>, Vec<usize>> = HashMap::with_capacity(strs.len());

        for (i, v) in strs.iter().enumerate() {
            let mut s: Vec<char> = v.clone().chars().collect();
            s.sort_unstable();

            let bucket = map.entry(s).or_insert(vec![]);
            bucket.push(i);
        }

        let mut results = Vec::with_capacity(strs.len());

        for indexes in map.values() {
            let values: Vec<String> = indexes.iter().map(|index| strs[*index].clone()).collect();
            results.push(values);
        }

        results
    }
}

fn main() {
    let input: Vec<String> = vec!["eat", "tea", "tan", "ate", "nat", "bat"]
        .iter()
        .map(|elem| elem.to_string())
        .collect();

    let result = Solution::group_anagrams(input);
    println!("{result:#?}");
}
