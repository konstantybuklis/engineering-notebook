use std::collections::HashSet;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut s: Vec<char> = s.clone().chars().collect();

        let mut set = HashSet::new();
        set.insert('a');
        set.insert('A');
        set.insert('e');
        set.insert('E');
        set.insert('i');
        set.insert('I');
        set.insert('o');
        set.insert('O');
        set.insert('u');
        set.insert('U');

        let mut left = 0;
        let mut right = s.len() - 1;

        while left < right {
            if set.contains(&s[left]) && set.contains(&s[right]) {
                s.swap(left, right);
                left += 1;
                right -= 1;

                continue;
            }

            if set.contains(&s[left]) && !set.contains(&s[right]) {
                right -= 1;

                continue;
            }

            if !set.contains(&s[left]) && set.contains(&s[right]) {
                left += 1;

                continue;
            }

            if !set.contains(&s[left]) && !set.contains(&s[right]) {
                left += 1;
                right -= 1;

                continue;
            }
        }

        s.into_iter().collect()
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_reversing() {
        let input = String::from("IceCreAm");

        assert_eq!(Solution::reverse_vowels(input), String::from("AceCreIm"));

        let input = String::from("leetcode");
        assert_eq!(Solution::reverse_vowels(input), String::from("leotcede"))
    }
}
