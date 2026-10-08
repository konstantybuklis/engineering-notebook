fn main() {
    let s = String::from("hello world");
    let world = &s[6..11];
    println!("{world}");

    let hello = first_word(&s); // &String
    println!("The value of hello is: {hello}");

    let result = first_word("result is here"); // &str
    println!("The value of result is: {result}");

    let result = first_word(&"result is here"[..6]); // &str
    println!("The value of result is: {result}");

    let a = [1, 2, 3, 4, 5];

    let slice: &[i32] = &a[1..3];
    println!("The value of slice is: {slice:?}");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
