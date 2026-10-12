#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let s_v: Vec<char> = s.chars().collect();
        let t_v: Vec<char> = t.chars().collect();

        let mut s_pointer = 0;
        let mut t_pointer = 0;
        let mut counter = 0;

        while s_pointer < s.len() && t_pointer < t.len() {
            if s_v[s_pointer] == t_v[t_pointer] {
                counter += 1;
                s_pointer += 1;
                t_pointer += 1;

                continue;
            }

            t_pointer += 1;
        }

        counter == s_v.len()
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_correct_subsequence() {
        let s = String::from("abc");
        let t = String::from("ahbgdc");

        assert!(Solution::is_subsequence(s, t))
    }

    #[test]
    fn is_incorrect_subsequence() {
        let s = String::from("axc");
        let t = String::from("ahbgdc");

        assert!(!Solution::is_subsequence(s, t))
    }
}
