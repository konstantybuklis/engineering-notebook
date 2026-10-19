#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut s: Vec<char> = s.chars().collect();
        s.sort();

        let mut t: Vec<char> = t.chars().collect();
        t.sort();

        s == t
    }
}

fn main() {}
