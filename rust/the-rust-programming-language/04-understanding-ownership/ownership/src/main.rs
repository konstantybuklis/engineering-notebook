#[allow(unused_assignments)]
fn main() {
    {
        // s is not valid here, since it's not yet declared
        let s = "hello"; // s is valid from this point forward

        // do stuff with s
        println!("{s}");
    } // this scope is now over, and s is no longer valid

    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("The value of s is: {s}");

    {
        // s is not valid here, since it's not yet declared
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
        println!("{s}");
    } // this scope is now over, and s is no longer valid (Rust calls drop function here)

    let x = 5;
    let y = x; // copy on the stack
    println!("The value of y is: {y}");

    let s1 = String::from("hello");
    let s2 = s1; // move
    println!("The value of s2 is: {s2}");

    let mut s = String::from("hello");
    s = String::from("ahoy"); // immediately droped "hello"
    println!("{s}, world!");

    let s1 = String::from("hello");
    let s2 = s1.clone(); // deep copy
    println!("s1 = {s1}, s2 = {s2}");

    let some_string = String::from("some string");
    takes_ownership(some_string); // move
    // some_string is no longer valid here

    let some_integer = 42;
    makes_copy(some_integer); // copy
    // some_integer is valid here

    let some_string = String::from("Hello, world!");
    let other_string = takes_and_give_back(some_string);
    println!("The value of other_string is: {other_string}");

    let (original_string, length) = calculate_length(other_string);
    println!("original_string = {original_string}, length = {length}");
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
} // drop is called here

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn takes_and_give_back(some_string: String) -> String {
    some_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
