#[allow(dead_code)]
enum OwnOption<T> {
    OwnSome(T),
    OwnNone,
}

#[allow(dead_code)]
enum OwnResult<T, E> {
    OwnOk(T),
    OwnErr(E),
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

#[allow(dead_code)]
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct OtherPoint<X1, Y1> {
    x: X1,
    y: Y1,
}

#[allow(dead_code)]
impl<X1, Y1> OtherPoint<X1, Y1> {
    fn mix_up<X2, Y2>(self, other: OtherPoint<X2, Y2>) -> OtherPoint<X1, Y2> {
        OtherPoint {
            x: self.x,
            y: other.y,
        }
    }
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");

    let origin: Point<i32> = Point { x: 10, y: 20 };
    println!("x = {}, y = {}; {origin:?}", origin.x, origin.y);

    let value_x = origin.x();
    println!("x = {value_x}");

    let p1 = OtherPoint { x: 10.0, y: 'a' };
    let p2 = OtherPoint { x: 10, y: 25.0 };
    let p3 = p1.mix_up(p2);
    println!("{p3:?}");
}
