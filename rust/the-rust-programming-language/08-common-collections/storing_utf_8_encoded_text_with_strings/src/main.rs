fn main() {
    let mut _s = String::new();

    let data = "initial contents";
    let s = data.to_string();
    println!("{s}");

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{s}");

    let mut s = String::from("lo");
    s.push('l');

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");

    let s = String::from("Здравствуйте");

    for c in s.chars() {
        println!("{c}");
    }
}
