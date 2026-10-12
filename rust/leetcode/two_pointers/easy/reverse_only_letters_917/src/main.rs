#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        let mut char_vec: Vec<char> = s.chars().collect();

        let mut left = 0;
        let mut right = char_vec.len() - 1;

        while left < right {
            if char_vec[left].is_alphabetic() && char_vec[right].is_alphabetic() {
                char_vec.swap(left, right);
                left += 1;
                right -= 1;
            };

            if !char_vec[left].is_alphabetic() && !char_vec[right].is_alphabetic() {
                left += 1;
                right -= 1;
            }

            if !char_vec[left].is_alphabetic() && char_vec[right].is_alphabetic() {
                left += 1;
            }

            if char_vec[left].is_alphabetic() && !char_vec[right].is_alphabetic() {
                right -= 1;
            }
        }

        char_vec.iter().collect()
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_reversing_1() {
        let s = String::from("a-bC-dEf-ghIj");

        let result = Solution::reverse_only_letters(s);

        assert_eq!(result, String::from("j-Ih-gfE-dCba"));
    }

    #[test]
    fn correct_reversing_2() {
        let s = String::from("Test1ng-Leet=code-Q!");

        let result = Solution::reverse_only_letters(s);

        assert_eq!(result, String::from("Qedo1ct-eeLg=ntse-T!"));
    }
}
