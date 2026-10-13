#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();

        let mut nums1 = nums1.clone();
        nums1.sort();

        let mut nums2 = nums2.clone();
        nums2.sort();

        let mut p1 = 0;
        let mut p2 = 0;

        while p1 < nums1.len() && p2 < nums2.len() {
            if nums1[p1] == nums2[p2] {
                result.push(nums1[p1]);

                p1 += 1;
                p2 += 1;

                continue;
            }

            if nums1[p1] < nums2[p2] {
                p1 += 1;
                continue;
            }

            if nums2[p2] < nums1[p1] {
                p2 += 1;
            }
        }

        result
    }
}

fn main() {}
