#[derive(Debug)]
#[allow(dead_code)]
struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

#[allow(dead_code)]
impl MinStack {
    fn new() -> Self {
        Self {
            stack: Vec::new(),
            min_stack: Vec::new(),
        }
    }

    fn push(&mut self, value: i32) {
        if self.min_stack.len() == 0 {
            self.min_stack.push(value);
            self.stack.push(value);

            return;
        }

        if self.min_stack.last().unwrap() >= &value {
            self.min_stack.push(value);
        }

        self.stack.push(value);
    }

    fn pop(&mut self) {
        if self.min_stack.last().unwrap() == self.stack.last().unwrap() {
            self.min_stack.pop();
        }

        self.stack.pop();
    }

    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.min_stack.last().unwrap()
    }
}

fn main() {}
