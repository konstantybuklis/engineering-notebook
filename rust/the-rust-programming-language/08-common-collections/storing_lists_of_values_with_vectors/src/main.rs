#[allow(dead_code)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let _v: Vec<i32> = Vec::new();

    let _v = vec![1, 2, 3];

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is: {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is: {third}"),
        None => println!("There is no third element"),
    };

    let v = vec![100, 32, 57];
    for elem in &v {
        println!("{elem}");
    }

    let mut v = vec![100, 32, 57];
    for elem in &mut v {
        *elem += 50;
    }

    println!("{v:?}");

    let mut v = Vec::new();
    v.push(SpreadsheetCell::Int(3));
    v.push(SpreadsheetCell::Float(10.12));
    v.push(SpreadsheetCell::Text(String::from("blue")));
} // <- v goes out of scope and is freed here
