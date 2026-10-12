#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let v: Vec<_> = s
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect();

        if v.is_empty() {
            return true;
        }

        let mut left = 0;
        let mut right = v.len() - 1;

        while left < right {
            if v[left] != v[right] {
                return false;
            }

            left += 1;
            right -= 1;
        }

        return true;
    }
}

fn main() {}
