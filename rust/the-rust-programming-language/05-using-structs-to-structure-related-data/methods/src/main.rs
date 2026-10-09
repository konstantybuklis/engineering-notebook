#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    fn square(size: u32) -> Self {
        Self::new(size, size)
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn change_width(&mut self, new_width: u32) {
        self.width = new_width;
    }

    fn change_height(&mut self, new_height: u32) {
        self.height = new_height;
    }

    fn can_hold(&self, other_rect: &Self) -> bool {
        self.width > other_rect.width && self.height > other_rect.height
    }
}

fn main() {
    let mut rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    rect1.change_width(50);
    rect1.change_height(70);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let rect2 = Rectangle::new(40, 60);
    println!("rect1 can hold rect2 = {}", rect1.can_hold(&rect2));

    let sq = Rectangle::square(40);
    println!("{sq:?}");
}
