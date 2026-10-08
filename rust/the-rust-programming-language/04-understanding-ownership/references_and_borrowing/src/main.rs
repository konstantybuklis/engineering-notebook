fn main() {
    let s1 = String::from("hello");

    let length = calculate_length(&s1);
    println!("The length of '{s1}' is {length}.");

    let mut s = String::from("hello");
    change(&mut s);

    {
        let r1 = &mut s;
        r1.push_str(" string");
    }

    let r2 = &mut s;
    println!("{r2}");

    let mut some_string = String::from("some text");

    let r1 = &some_string;
    let r2 = &some_string;
    println!("r1 = {r1}, r2 = {r2}");

    let r3 = &mut some_string;
    println!("r3 = {r3}"); // allowed (scopes of r1 and r2 end afte the println!)
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}
