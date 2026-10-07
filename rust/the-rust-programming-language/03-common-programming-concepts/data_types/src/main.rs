use std::io;

fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The guess value is: {guess}");

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("{x}, {y}");

    let t = true;
    let f: bool = false;
    println!("{t}, {f}");

    let heart_eyed_cat = '😻'; // char
    println!("{heart_eyed_cat}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{tup:?}");

    // use pattern matching to destructure a tuple value
    let (x, y, z) = tup;
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("tup.0: {five_hundred}, tup.1: {six_point_four}, tup.2: {one}");

    let unit: () = ();
    println!("The unit type: {unit:?}");

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of a is: {a:?}");

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("{months:#?}");

    let a = [3; 5];
    println!("The value of a is: {a:?}");

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("first: {first}, second: {second}");

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of element at index {index} is: {element}");
}
