struct Solution;

impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<i32> = Vec::new();

        for asteroid in asteroids {
            if stack.len() == 0 {
                stack.push(asteroid);
                continue;
            }

            while let Some(stack_asteroid) = stack.pop() {
                if stack_asteroid.is_positive() && asteroid.is_positive()
                    || stack_asteroid.is_negative() && asteroid.is_negative()
                {
                    stack.push(stack_asteroid);
                    stack.push(asteroid);
                    break;
                }

                if stack_asteroid.is_positive() && asteroid.is_negative() {
                    if stack_asteroid.abs() < asteroid.abs() {
                        if stack.len() == 0 {
                            stack.push(asteroid);
                            break;
                        }

                        continue;
                    } else if stack_asteroid.abs() > asteroid.abs() {
                        stack.push(stack_asteroid);
                        break;
                    } else {
                        break;
                    }
                }

                if stack_asteroid.is_negative() && asteroid.is_positive() {
                    stack.push(stack_asteroid);
                    stack.push(asteroid);
                    break;
                }
            }
        }

        stack
    }
}

fn main() {
    let result = Solution::asteroid_collision(vec![8, -8]);
    println!("{result:?}")
}
