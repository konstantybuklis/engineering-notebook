#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::new();

        for token in tokens {
            match token.parse::<i32>() {
                Ok(num) => stack.push(num),
                Err(_) => {
                    let first: i32 = stack.pop().unwrap();
                    let second: i32 = stack.pop().unwrap();

                    let result = match token.as_str() {
                        "+" => second + first,
                        "-" => second - first,
                        "*" => second * first,
                        "/" => second / first,
                        _ => unreachable!(),
                    };

                    stack.push(result);
                }
            }
        }

        *stack.first().unwrap()
    }
}

fn main() {}
