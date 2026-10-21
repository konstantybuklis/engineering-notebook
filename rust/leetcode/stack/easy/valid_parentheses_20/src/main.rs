use std::collections::HashMap;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut parentheses = HashMap::new();
        parentheses.insert('(', ')');
        parentheses.insert('{', '}');
        parentheses.insert('[', ']');

        let mut stack: Vec<char> = Vec::new();

        for parenthesis in s.chars() {
            if stack.len() == 0 {
                stack.push(parenthesis);

                if !parentheses.contains_key(&parenthesis) {
                    return false;
                }

                continue;
            }

            while let Some(stack_parenthesis) = stack.pop() {
                if parentheses.contains_key(&parenthesis) {
                    stack.push(stack_parenthesis);
                    stack.push(parenthesis);
                    break;
                }

                if !parentheses.contains_key(&parenthesis) {
                    if parentheses.get(&stack_parenthesis).unwrap() != &parenthesis {
                        return false;
                    }
                }

                break;
            }
        }

        stack.is_empty()
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct() {
        let parentheses = String::from("()[]{}");
        assert!(Solution::is_valid(parentheses));

        let parentheses = String::from("([])");
        assert!(Solution::is_valid(parentheses));
    }

    #[test]
    fn incorrect() {
        let parentheses = String::from("(]");
        assert!(!Solution::is_valid(parentheses));

        let parentheses = String::from("([)]");
        assert!(!Solution::is_valid(parentheses));
    }
}
