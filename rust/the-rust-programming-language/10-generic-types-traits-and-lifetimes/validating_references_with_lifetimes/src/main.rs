use std::fmt::Display;

#[derive(Debug)]
#[allow(dead_code)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let x = 5;
    let r = &x;

    println!("r: {r}");

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is: {result}");

    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(&string1, &string2);
        println!("The longest string is: {result}");
    }

    let string1 = String::from("a");
    let result;
    {
        let string2 = String::from("abc");
        result = longest_v2(&string1, &string2);
    }

    println!("{result}");

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i: ImportantExcerpt<'_> = ImportantExcerpt {
        part: first_sentence,
    };

    println!("{i:?}");
}

// The returned reference will be valid as long as both of the parameters are valid.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn longest_v2<'a, 'b>(x: &'a str, _y: &'b str) -> &'a str {
    x
}

#[allow(dead_code)]
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);

    if x.len() > y.len() { x } else { y }
}
