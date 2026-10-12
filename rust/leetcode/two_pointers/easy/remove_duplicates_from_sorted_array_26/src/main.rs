#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut counter = 0;

        let mut slow = 0;
        let mut fast = 0;

        while fast < nums.len() {
            if nums[slow] != nums[fast] {
                nums.swap(slow + 1, fast);

                slow += 1;
                counter += 1;
            }

            fast += 1;
        }

        counter + 1
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_removing() {
        let mut v = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];

        let result = Solution::remove_duplicates(&mut v);
        assert_eq!(result, 5);
    }
}
