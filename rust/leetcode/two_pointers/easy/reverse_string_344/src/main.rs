#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let mut left = 0;
        let mut right = s.len() - 1;

        while left < right {
            s.swap(left, right);
            left += 1;
            right -= 1;
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_reversing() {
        let mut v = vec!['h', 'e', 'l', 'l', 'o'];

        Solution::reverse_string(&mut v);

        assert_eq!(v, ['o', 'l', 'l', 'e', 'h']);
    }
}
