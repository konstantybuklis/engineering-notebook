#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_stars(s: String) -> String {
        let mut stack = String::with_capacity(s.len());

        for c in s.chars() {
            if c == '*' {
                stack.pop();
            } else {
                stack.push(c);
            }
        }

        stack
    }
}

fn main() {}
